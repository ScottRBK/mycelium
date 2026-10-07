//! Resolve saved Python bindings before the exporter's language-wide name heuristics.

use std::collections::{BTreeMap, BTreeSet};

use crate::declarations::{Class, PythonBinding, PythonBindings};

pub(super) struct PythonTypes<'a> {
    bindings: &'a BTreeMap<String, PythonBindings>,
    modules: BTreeMap<String, Vec<&'a str>>,
    module_suffixes: BTreeMap<String, Vec<(&'a str, &'a str)>>,
    classes: BTreeMap<(&'a str, &'a str, usize), Option<usize>>,
}

enum Target<'a> {
    Class(usize),
    Module(&'a str),
}

impl<'a> PythonTypes<'a> {
    pub(super) fn new(
        bindings: &'a BTreeMap<String, PythonBindings>,
        declarations: &'a [Class],
    ) -> Self {
        let mut modules: BTreeMap<String, Vec<&str>> = BTreeMap::new();
        let mut module_suffixes: BTreeMap<String, Vec<(&str, &str)>> = BTreeMap::new();
        for file in bindings.keys() {
            if let Some(module) = file.strip_suffix(".py") {
                let module = module.strip_suffix("/__init__").unwrap_or(module);
                for (position, _) in module.match_indices('/') {
                    module_suffixes
                        .entry(module[position + 1..].replace('/', "."))
                        .or_default()
                        .push((file, &module[..position]));
                }
                modules
                    .entry(module.replace('/', "."))
                    .or_default()
                    .push(file);
            }
        }
        let mut classes = BTreeMap::new();
        for (index, class) in declarations
            .iter()
            .enumerate()
            .filter(|(_, c)| c.kind != "module")
        {
            classes
                .entry((class.file.as_str(), class.name.as_str(), class.line))
                .and_modify(|entry| *entry = None)
                .or_insert(Some(index));
        }
        Self {
            bindings,
            modules,
            module_suffixes,
            classes,
        }
    }

    /// Outer None permits legacy heuristics; Some(None) explicitly forbids guessing.
    pub(super) fn resolve(&self, file: &str, name: &str) -> Option<Option<usize>> {
        let bindings = self.bindings.get(file)?;
        if bindings.uncertain {
            return Some(None);
        }
        let name = name.split(['<', '[', '(']).next()?.trim();
        let (root, rest) = name.split_once('.').unwrap_or((name, ""));
        let binding = bindings.names.get(root)?;
        let mut seen = BTreeSet::new();
        let target = if let PythonBinding::Modules(modules) = binding {
            name.rsplit_once('.')
                .filter(|(module, _)| modules.contains(*module))
                .and_then(|(module, name)| self.module(module, file).map(|file| (file, name)))
                .and_then(|(file, name)| self.member(file, name, &mut seen))
        } else {
            self.member(file, root, &mut seen)
                .and_then(|target| match (target, rest) {
                    (target, "") => Some(target),
                    (Target::Module(file), rest) if !rest.contains('.') => {
                        self.member(file, rest, &mut seen)
                    }
                    // Neither a class nor a package establishes imported nested types/submodules.
                    _ => None,
                })
        };
        Some(match target {
            Some(Target::Class(index)) => Some(index),
            _ => None,
        })
    }

    fn module(&self, name: &str, origin: &str) -> Option<&'a str> {
        let files = self.module_files(name, origin);
        (files.len() == 1).then(|| files[0])
    }

    /// Use original import spelling when looking for repository candidates for a diagnostic.
    pub(super) fn reference_name<'b>(&'b self, file: &str, name: &'b str) -> &'b str {
        let name = name.split(['<', '[', '(']).next().unwrap_or(name).trim();
        if let Some((_, leaf)) = name.rsplit_once('.') {
            return leaf;
        }
        match self
            .bindings
            .get(file)
            .and_then(|bindings| bindings.names.get(name))
        {
            Some(PythonBinding::Import(target)) => target.rsplit('.').next().unwrap_or(name),
            _ => name,
        }
    }

    fn module_files(&self, name: &str, origin: &str) -> Vec<&'a str> {
        if let Some(files) = self.modules.get(name) {
            return files.clone();
        }
        self.module_suffixes
            .get(name)
            .into_iter()
            .flatten()
            .filter(|(_, prefix)| {
                origin
                    .strip_prefix(prefix)
                    .is_some_and(|rest| rest.starts_with('/'))
                    && !self.bindings.contains_key(&format!("{prefix}/__init__.py"))
            })
            .map(|(file, _)| *file)
            .collect()
    }

    fn imported(
        &self,
        target: &str,
        origin: &str,
        seen: &mut BTreeSet<(String, String)>,
    ) -> Option<Target<'a>> {
        let (module, name) = target.rsplit_once('.')?;
        let files = self.module_files(module, origin);
        if !files.is_empty() {
            let [file] = files.as_slice() else {
                return None;
            };
            let file = *file;
            let bindings = &self.bindings[file];
            if bindings.uncertain {
                return None;
            }
            if file.ends_with("/__init__.py")
                && matches!(
                    bindings.names.get(name),
                    Some(PythonBinding::Import(own)) if own == target
                )
            {
                // `from . import child` in __init__.py establishes the child module itself.
                return self.module(target, file).map(Target::Module);
            }
            if bindings.names.contains_key(name) {
                // From-imports prefer existing attributes, including re-exports, over submodules.
                return self.member(file, name, seen);
            }
            if !file.ends_with("/__init__.py") {
                return None;
            }
        }
        // A namespace package need not contain an __init__.py file.
        self.module(target, origin).map(Target::Module)
    }

    fn member(
        &self,
        file: &str,
        name: &str,
        seen: &mut BTreeSet<(String, String)>,
    ) -> Option<Target<'a>> {
        let bindings = self.bindings.get(file)?;
        // Bound the stack as well as rejecting re-export cycles.
        if bindings.uncertain || seen.len() >= 64 || !seen.insert((file.into(), name.into())) {
            return None;
        }
        match bindings.names.get(name)? {
            PythonBinding::Class(line) => self.class(file, name, *line).map(Target::Class),
            PythonBinding::Import(target) => self.imported(target, file, seen),
            PythonBinding::Module(module) => self.module(module, file).map(Target::Module),
            PythonBinding::Modules(modules) if modules.contains(name) => {
                self.module(name, file).map(Target::Module)
            }
            _ => None,
        }
    }

    fn class(&self, file: &str, name: &str, line: usize) -> Option<usize> {
        self.classes.get(&(file, name, line)).copied().flatten()
    }
}
