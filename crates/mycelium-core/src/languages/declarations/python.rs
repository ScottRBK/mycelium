use super::*;
use crate::declarations::{PythonBinding, PythonBindings};
use std::collections::BTreeSet;

pub(super) fn bindings(root: Node<'_>, source: &[u8], file: &str) -> PythonBindings {
    let mut bindings = PythonBindings {
        uncertain: root.has_error(),
        ..Default::default()
    };
    for node in children(root) {
        if matches!(node.kind(), "import_statement" | "import_from_statement") {
            import_bindings(node, source, file, &mut bindings);
        } else {
            let definition = if node.kind() == "decorated_definition" {
                node.child_by_field_name("definition").unwrap_or(node)
            } else {
                node
            };
            if definition.kind() == "class_definition" {
                if let Some(name) = field(definition, "name", source) {
                    insert_binding(
                        &mut bindings,
                        name,
                        PythonBinding::Class(definition.start_position().row + 1),
                    );
                }
            }
        }
    }
    // First reject shadows of the typing guard itself, including shadows inside other scopes.
    let mut guard_bindings = bindings.clone();
    uncertain_bindings(root, root, source, &mut guard_bindings, &BTreeSet::new());
    let mut annotation_imports = BTreeSet::new();
    for node in children(root) {
        if !typing_guard(node, source, &guard_bindings) {
            continue;
        }
        if let Some(body) = node.child_by_field_name("consequence") {
            for import in children(body) {
                if matches!(import.kind(), "import_statement" | "import_from_statement") {
                    import_bindings(import, source, file, &mut bindings);
                    annotation_imports.insert(import.id());
                }
            }
        }
    }
    uncertain_bindings(root, root, source, &mut bindings, &annotation_imports);
    bindings
}

fn import_bindings(node: Node<'_>, source: &[u8], file: &str, bindings: &mut PythonBindings) {
    let module = field(node, "module_name", source);
    for name in node.children_by_field_name("name", &mut node.walk()) {
        let original = field(name, "name", source).unwrap_or_else(|| text(name, source));
        let alias = field(name, "alias", source);
        let (local, binding) = if let Some(module) = &module {
            (
                alias.unwrap_or_else(|| original.clone()),
                import_target(module, &original, file)
                    .map(PythonBinding::Import)
                    .unwrap_or(PythonBinding::Unknown),
            )
        } else if let Some(alias) = alias {
            (alias, PythonBinding::Module(original))
        } else {
            let root = original.split('.').next().unwrap_or("").to_string();
            (root, PythonBinding::Modules([original].into()))
        };
        insert_binding(bindings, local, binding);
    }
}

fn typing_guard(node: Node<'_>, source: &[u8], bindings: &PythonBindings) -> bool {
    if bindings.uncertain
        || node.kind() != "if_statement"
        || node.child_by_field_name("alternative").is_some()
    {
        return false;
    }
    let Some(condition) = field(node, "condition", source) else {
        return false;
    };
    let (root, rest) = condition.split_once('.').unwrap_or((&condition, ""));
    match bindings.names.get(root) {
        Some(PythonBinding::Import(target)) => rest.is_empty() && target == "typing.TYPE_CHECKING",
        Some(PythonBinding::Module(module)) => module == "typing" && rest == "TYPE_CHECKING",
        Some(PythonBinding::Modules(modules)) => {
            modules.contains("typing") && root == "typing" && rest == "TYPE_CHECKING"
        }
        _ => false,
    }
}

// File-wide by design: without lexical/data-flow analysis, rebinding can disprove a match but
// cannot select another class. Never let an uncertain import fall through to name heuristics.
fn uncertain_bindings(
    node: Node<'_>,
    root: Node<'_>,
    source: &[u8],
    bindings: &mut PythonBindings,
    annotation_imports: &BTreeSet<usize>,
) {
    if node.kind() == "wildcard_import" {
        bindings.uncertain = true;
    }
    let parent = node.parent().map(|parent| {
        if parent.kind() == "decorated_definition" {
            parent.parent().unwrap_or(parent)
        } else {
            parent
        }
    });
    let top_level = parent.is_some_and(|p| p.id() == root.id());
    if node.kind() == "function_definition" || (node.kind() == "class_definition" && !top_level) {
        if let Some(name) = field(node, "name", source) {
            let nested_class = node.kind() == "class_definition"
                && parent
                    .and_then(|p| p.parent())
                    .is_some_and(|p| p.kind() == "class_definition");
            if !nested_class || bindings.names.contains_key(&name) {
                bindings.names.insert(name, PythonBinding::Unknown);
            }
        }
    }
    if matches!(node.kind(), "import_statement" | "import_from_statement")
        && !top_level
        && !annotation_imports.contains(&node.id())
    {
        for name in node.children_by_field_name("name", &mut node.walk()) {
            let original = field(name, "name", source).unwrap_or_else(|| text(name, source));
            let local = field(name, "alias", source)
                .unwrap_or_else(|| original.split('.').next().unwrap_or("").to_string());
            bindings.names.insert(local, PythonBinding::Unknown);
        }
    }
    let target = match node.kind() {
        "assignment" if node.child_by_field_name("right").is_none() => None,
        "assignment"
        | "augmented_assignment"
        | "for_statement"
        | "for_in_clause"
        | "type_alias_statement" => node.child_by_field_name("left"),
        "named_expression" => node.child_by_field_name("name"),
        "as_pattern" => node.child_by_field_name("alias"),
        "parameters" | "lambda_parameters" | "delete_statement" | "case_pattern" => Some(node),
        _ => None,
    };
    if let Some(target) = target {
        block_bound_names(target, source, bindings);
    }
    if let Some(parameters) = node.child_by_field_name("type_parameters") {
        block_bound_names(parameters, source, bindings);
    }
    for child in children(node) {
        uncertain_bindings(child, root, source, bindings, annotation_imports);
    }
}

fn block_bound_names(node: Node<'_>, source: &[u8], bindings: &mut PythonBindings) {
    let target = match node.kind() {
        "identifier" => {
            bindings
                .names
                .insert(text(node, source), PythonBinding::Unknown);
            return;
        }
        "attribute" => node.child_by_field_name("object"),
        "subscript" => node.child_by_field_name("value"),
        "typed_parameter" => node.named_child(0),
        "default_parameter" | "typed_default_parameter" => node.child_by_field_name("name"),
        _ => {
            for child in children(node) {
                block_bound_names(child, source, bindings);
            }
            return;
        }
    };
    if let Some(target) = target {
        block_bound_names(target, source, bindings);
    }
}

fn import_target(module: &str, name: &str, file: &str) -> Option<String> {
    let dots = module.chars().take_while(|c| *c == '.').count();
    if dots == 0 {
        return Some(format!("{module}.{name}"));
    }
    let mut package: Vec<_> = file.rsplit_once('/')?.0.split('/').collect();
    if dots > package.len() {
        return None;
    }
    package.truncate(package.len() + 1 - dots);
    package.extend(module[dots..].split('.').filter(|s| !s.is_empty()));
    package.push(name);
    Some(package.join("."))
}

fn insert_binding(bindings: &mut PythonBindings, name: String, value: PythonBinding) {
    bindings
        .names
        .entry(name)
        .and_modify(|existing| match (&mut *existing, &value) {
            (PythonBinding::Modules(saved), PythonBinding::Modules(added)) => {
                saved.extend(added.iter().cloned());
            }
            _ => *existing = PythonBinding::Unknown,
        })
        .or_insert(value);
}

pub(super) fn walk(
    node: Node<'_>,
    source: &[u8],
    file: &str,
    owner: Option<usize>,
    diagram: &mut ClassDiagram,
    detection: &frameworks::Detection,
) {
    let mut owner = owner;
    if node.kind() == "class_definition" {
        if let Some(name) = field(node, "name", source) {
            let index = add_class(diagram, file, name, "class", node.start_position().row + 1);
            if let Some(bases) = node.child_by_field_name("superclasses") {
                for base in children(bases)
                    .into_iter()
                    .filter(|b| b.kind() != "keyword_argument")
                {
                    diagram.classes[index].bases.push(Base {
                        name: text(base, source),
                        relation: "inherits".into(),
                    });
                }
            }
            diagram.classes[index].test = detection.evidence(node);
            owner = Some(index);
        }
    }
    if node.kind() == "function_definition" {
        let index = owner.unwrap_or_else(|| module(diagram, file));
        if let Some(name) = field(node, "name", source) {
            let parameters = node
                .child_by_field_name("parameters")
                .map(|p| {
                    children(p)
                        .into_iter()
                        .filter_map(|p| {
                            let name = field(p, "name", source)
                                .or_else(|| {
                                    children(p)
                                        .into_iter()
                                        .find(|n| {
                                            matches!(
                                                n.kind(),
                                                "identifier"
                                                    | "list_splat_pattern"
                                                    | "dictionary_splat_pattern"
                                            )
                                        })
                                        .map(|n| text(n, source))
                                })
                                .unwrap_or_else(|| text(p, source));
                            if name == "self" || name == "cls" || name == "/" || name == "*" {
                                None
                            } else {
                                Some(Parameter {
                                    name,
                                    value_type: field(p, "type", source),
                                })
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
            let property = node.parent().is_some_and(|p| {
                p.kind() == "decorated_definition"
                    && children(p)
                        .iter()
                        .any(|d| d.kind() == "decorator" && text(*d, source) == "@property")
            });
            diagram.classes[index].members.push(Member {
                test: detection.evidence(node),
                file: String::new(),
                visibility: if name.starts_with('_') { "-" } else { "+" }.into(),
                name,
                kind: if property { "field" } else { "method" }.into(),
                parameters,
                value_type: field(node, "return_type", source),
                line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
        // Instance attributes are declarations too; local variables and nested functions aren't.
        instance_fields(node, source, index, diagram, detection);
        return;
    }
    if node.kind() == "assignment" {
        if let (Some(index), Some(name)) = (owner, field(node, "left", source)) {
            if !name.contains('.') {
                push_field(node, source, index, name, diagram, detection);
            }
        }
        return;
    }
    for child in children(node) {
        walk(child, source, file, owner, diagram, detection);
    }
}

fn push_field(
    node: Node<'_>,
    source: &[u8],
    index: usize,
    name: String,
    diagram: &mut ClassDiagram,
    detection: &frameworks::Detection,
) {
    if diagram.classes[index]
        .members
        .iter()
        .any(|m| m.kind == "field" && m.name == name)
    {
        return;
    }
    diagram.classes[index].members.push(Member {
        test: detection.evidence(node),
        file: String::new(),
        visibility: if name.starts_with('_') { "-" } else { "+" }.into(),
        name,
        kind: "field".into(),
        parameters: Vec::new(),
        value_type: field(node, "type", source),
        line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
    });
}

fn instance_fields(
    node: Node<'_>,
    source: &[u8],
    index: usize,
    diagram: &mut ClassDiagram,
    detection: &frameworks::Detection,
) {
    for child in children(node) {
        if matches!(child.kind(), "class_definition" | "function_definition") {
            continue;
        }
        if child.kind() == "assignment" {
            if let Some(name) = field(child, "left", source)
                .and_then(|s| s.strip_prefix("self.").map(str::to_string))
            {
                push_field(child, source, index, name, diagram, detection);
            }
        } else {
            instance_fields(child, source, index, diagram, detection);
        }
    }
}
