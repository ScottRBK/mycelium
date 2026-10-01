//! Direct .NET attributes and Java annotations, with repository type-conflict guards.
use super::*;

const MSTEST: &str = "Microsoft.VisualStudio.TestTools.UnitTesting";

pub(super) fn detect(
    root: Node<'_>,
    source: &[u8],
    file: &str,
    detection: &mut Detection,
    vb: bool,
) {
    let normalize = |name: &str| {
        if vb {
            name.to_lowercase()
        } else {
            name.to_string()
        }
    };
    let mut imports = Vec::new();
    visit(root, &mut |node| {
        if matches!(node.kind(), "using_directive" | "imports_statement") {
            imports.push(node);
            if node.kind() == "using_directive" && keyword(node, "global") {
                if keyword(node, "static") {
                    detection
                        .bindings
                        .declared
                        .insert("dotnet:global-static-import".into());
                }
                let parts = children(node);
                if parts.len() == 2 {
                    detection
                        .bindings
                        .declared
                        .insert(key(&text(parts[0], source)));
                }
            }
        }
        if type_node(node) || matches!(node.kind(), "namespace_declaration" | "namespace_block") {
            if let Some(name) = field(node, "name", source) {
                for omit_modules in [false, true] {
                    if omit_modules && !vb {
                        continue;
                    }
                    let full = if node.kind() == "namespace_block"
                        && name.to_lowercase().starts_with("global.")
                    {
                        name[7..].to_string()
                    } else {
                        format!("{}.{}", scope_inner(root, node, source, omit_modules), name)
                    };
                    // VB promotes a Module's nested types into its enclosing namespace.
                    detection
                        .bindings
                        .declared
                        .insert(key(full.trim_start_matches('.')));
                }
            }
        }
    });
    // Recovered declarations can disprove a binding in another file, but cannot prove tests.
    if !detection.syntax_valid {
        return;
    }
    visit(root, &mut |node| {
        let container = matches!(node.kind(), "class_declaration" | "class_block");
        if !container && node.kind() != "method_declaration" {
            return;
        }
        let mut namespaces = Vec::new();
        let mut static_import = false;
        let mut aliases: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for directive in imports
            .iter()
            .copied()
            .filter(|d| d.parent().is_some_and(|p| ancestor(p, node)))
        {
            if vb {
                if directive.parent().is_none_or(|p| p.id() != root.id()) {
                    continue;
                }
                for part in children(directive) {
                    if part.kind() == "imports_alias" {
                        if let (Some(alias), Some(ns)) = (
                            field(part, "alias", source),
                            field(part, "namespace", source),
                        ) {
                            aliases
                                .entry(normalize(&alias))
                                .or_default()
                                .push(clean(&normalize(&ns)));
                        }
                    } else if part.kind() == "namespace_name" {
                        namespaces.push(clean(&normalize(&text(part, source))));
                    }
                }
                continue;
            }
            if keyword(directive, "static") {
                static_import = true;
                continue;
            }
            if keyword(directive, "global") {
                continue;
            }
            let parts = children(directive);
            if parts.len() == 2 {
                aliases
                    .entry(text(parts[0], source))
                    .or_default()
                    .push(clean(&text(parts[1], source)));
            } else if let Some(name) = parts.first() {
                namespaces.push(clean(&text(*name, source)));
            }
        }
        let mut lists: Vec<_> = children(node)
            .into_iter()
            .filter(|n| {
                !(vb && container) && matches!(n.kind(), "attribute_list" | "attribute_block")
            })
            .collect();
        if vb && container {
            let wrapper = node
                .parent()
                .filter(|p| p.kind() == "type_declaration")
                .unwrap_or(node);
            let mut previous = wrapper.prev_named_sibling();
            while let Some(n) = previous {
                if n.kind() == "attribute_block" {
                    lists.push(n);
                } else if !n.is_extra() && n.kind() != "blank_line" {
                    break;
                }
                previous = n.prev_named_sibling();
            }
        }
        for list in lists {
            if children(list)
                .iter()
                .any(|n| n.kind() == "attribute_target_specifier")
            {
                continue;
            }
            for attr in children(list)
                .into_iter()
                .filter(|n| n.kind() == "attribute")
            {
                let Some(name) = field(attr, "name", source) else {
                    continue;
                };
                if attr.child_by_field_name("target").is_some() {
                    continue;
                }
                let name = normalize(&name);
                let spelling = clean(&name);
                let (head, tail) = spelling.split_once('.').unwrap_or((&spelling, ""));
                let absolute = name.starts_with("global::") || (vb && name.starts_with("global."));
                if !absolute && (static_import || inherited_scope(node)) {
                    continue;
                }
                let candidates = if absolute {
                    vec![spelling.clone()]
                } else if let Some(targets) = aliases.get(head) {
                    if targets.len() != 1 {
                        continue;
                    }
                    vec![if tail.is_empty() {
                        targets[0].clone()
                    } else {
                        format!("{}.{tail}", targets[0])
                    }]
                } else if spelling.contains('.') {
                    vec![spelling.clone()]
                } else {
                    namespaces
                        .iter()
                        .map(|ns| format!("{ns}.{spelling}"))
                        .collect()
                };
                let matches: BTreeSet<_> = candidates
                    .iter()
                    .filter_map(|name| rule(name, container, vb).map(|rule| (name, rule)))
                    .collect();
                if matches.len() != 1 {
                    continue;
                }
                let (resolved, rule) = matches.first().unwrap();
                let mut guards = BTreeSet::new();
                guard_name(&mut guards, resolved);
                for candidate in &candidates {
                    guard_name(&mut guards, candidate);
                }
                if !absolute {
                    guards.insert("dotnet:global-static-import".into());
                    let current = scope(root, node, source);
                    let mut prefix = current.as_str();
                    loop {
                        // Using/alias targets can themselves be relative to a namespace.
                        for candidate in &candidates {
                            let imported = if prefix.is_empty() {
                                candidate.clone()
                            } else {
                                format!("{prefix}.{candidate}")
                            };
                            guard_name(&mut guards, &imported);
                            let root_name = candidate.split('.').next().unwrap_or("");
                            guards.insert(key(&if prefix.is_empty() {
                                root_name.into()
                            } else {
                                format!("{prefix}.{root_name}")
                            }));
                        }
                        let full = if prefix.is_empty() {
                            spelling.clone()
                        } else {
                            format!("{prefix}.{spelling}")
                        };
                        guard_name(&mut guards, &full);
                        if !tail.is_empty() {
                            guards.insert(key(&if prefix.is_empty() {
                                head.into()
                            } else {
                                format!("{prefix}.{head}")
                            }));
                        }
                        if prefix.is_empty() {
                            break;
                        }
                        prefix = prefix.rsplit_once('.').map(|(p, _)| p).unwrap_or("");
                    }
                }
                detection.mark(node, attr, file, rule);
                detection
                    .bindings
                    .guards
                    .push((detection.marks[&node.id()].clone(), guards));
            }
        }
    });
}

fn guard_name(guards: &mut BTreeSet<String>, name: &str) {
    guards.insert(key(name));
    if name.to_lowercase().ends_with("attribute") {
        guards.insert(key(&name[..name.len() - 9]));
    } else {
        guards.insert(key(&format!("{name}Attribute")));
    }
}

fn key(name: &str) -> String {
    format!("dotnet:{}", name.to_lowercase())
}

pub(super) fn type_node(node: Node<'_>) -> bool {
    matches!(
        node.kind(),
        "class_declaration"
            | "struct_declaration"
            | "interface_declaration"
            | "enum_declaration"
            | "record_declaration"
            | "delegate_declaration"
            | "type_parameter"
            | "class_block"
            | "structure_block"
            | "interface_block"
            | "enum_block"
            | "module_block"
            | "annotation_type_declaration"
    )
}

pub(super) fn scope(root: Node<'_>, node: Node<'_>, source: &[u8]) -> String {
    scope_inner(root, node, source, false)
}

fn scope_inner(root: Node<'_>, node: Node<'_>, source: &[u8], omit_modules: bool) -> String {
    let mut names = Vec::new();
    let mut parent = node.parent();
    while let Some(n) = parent {
        if (type_node(n) && !(omit_modules && n.kind() == "module_block"))
            || matches!(n.kind(), "namespace_declaration" | "namespace_block")
        {
            if let Some(name) = field(n, "name", source) {
                if n.kind() == "namespace_block" && name.to_lowercase().starts_with("global.") {
                    names.push(name[7..].to_string());
                    break;
                }
                names.push(name);
            }
        }
        parent = n.parent();
    }
    if let Some(ns) = children(root)
        .into_iter()
        .find(|n| n.kind() == "file_scoped_namespace_declaration")
        .and_then(|n| field(n, "name", source))
    {
        names.push(ns);
    }
    names.reverse();
    names.join(".")
}

fn ancestor(parent: Node<'_>, node: Node<'_>) -> bool {
    let mut current = Some(node);
    while let Some(n) = current {
        if n.id() == parent.id() {
            return true;
        }
        current = n.parent();
    }
    false
}

/// Member types from a base/interface need compiler binding, outside this direct syntax subset.
pub(super) fn inherited_scope(node: Node<'_>) -> bool {
    let mut current = Some(node);
    while let Some(n) = current {
        if type_node(n)
            && children(n).iter().any(|child| {
                matches!(
                    child.kind(),
                    "base_list"
                        | "superclass"
                        | "super_interfaces"
                        | "extends_interfaces"
                        | "inherits_clause"
                        | "implements_clause"
                )
            })
        {
            return true;
        }
        current = n.parent();
    }
    false
}

fn clean(name: &str) -> String {
    name.replace(' ', "")
        .trim_start_matches("global::")
        .trim_start_matches("global.")
        .to_string()
}

fn rule(name: &str, container: bool, vb: bool) -> Option<&'static str> {
    let compare = |left: &str, right: &str| {
        if vb {
            left.eq_ignore_ascii_case(right)
        } else {
            left == right
        }
    };
    let entries = [
        (format!("{MSTEST}.TestClass"), true, "dotnet.mstest-class"),
        (
            format!("{MSTEST}.TestMethod"),
            false,
            "dotnet.mstest-method",
        ),
        (
            "NUnit.Framework.TestFixture".into(),
            true,
            "dotnet.nunit-class",
        ),
        ("NUnit.Framework.Test".into(), false, "dotnet.nunit-method"),
        (
            "NUnit.Framework.TestCase".into(),
            false,
            "dotnet.nunit-method",
        ),
        (
            "NUnit.Framework.TestCaseSource".into(),
            false,
            "dotnet.nunit-method",
        ),
        ("Xunit.Fact".into(), false, "dotnet.xunit-method"),
        ("Xunit.Theory".into(), false, "dotnet.xunit-method"),
    ];
    entries
        .into_iter()
        .find(|(fq, class, _)| {
            *class == container && (compare(name, fq) || compare(name, &format!("{fq}Attribute")))
        })
        .map(|(_, _, rule)| rule)
}
