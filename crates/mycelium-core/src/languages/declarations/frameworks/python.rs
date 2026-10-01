use super::*;

pub(super) fn detect(root: Node<'_>, source: &[u8], file: &str, detection: &mut Detection) {
    let mut imports = BTreeMap::new();
    let mut counts = BTreeMap::<String, usize>::new();
    for node in children(root) {
        let prefix = if node.kind() == "import_from_statement" {
            field(node, "module_name", source).map(|s| format!("{s}."))
        } else if node.kind() == "import_statement" {
            Some(String::new())
        } else {
            None
        };
        let Some(prefix) = prefix else { continue };
        for name in node.children_by_field_name("name", &mut node.walk()) {
            let original = field(name, "name", source).unwrap_or_else(|| text(name, source));
            let local = field(name, "alias", source).unwrap_or_else(|| original.clone());
            *counts.entry(local.clone()).or_default() += 1;
            imports.insert(local, format!("{prefix}{original}"));
        }
    }
    let mut shadows = std::collections::BTreeSet::new();
    let mut wildcard = false;
    visit(root, &mut |node| {
        wildcard |= node.kind() == "wildcard_import";
        if matches!(node.kind(), "class_definition" | "function_definition") {
            if let Some(name) = field(node, "name", source) {
                shadows.insert(name);
            }
        }
        let target = match node.kind() {
            "assignment" | "augmented_assignment" | "for_statement" | "for_in_clause" => {
                node.child_by_field_name("left")
            }
            "named_expression" => node.child_by_field_name("name"),
            "parameters" | "delete_statement" | "case_pattern" => Some(node),
            "as_pattern" => node.child_by_field_name("alias"),
            _ => None,
        };
        if let Some(target) = target {
            bound_names(target, source, &mut shadows);
        }
        if matches!(node.kind(), "import_statement" | "import_from_statement")
            && node.parent().is_some_and(|p| p.id() != root.id())
        {
            for name in node.children_by_field_name("name", &mut node.walk()) {
                let name = field(name, "alias", source)
                    .or_else(|| field(name, "name", source))
                    .unwrap_or_else(|| text(name, source));
                shadows.insert(name.split('.').next().unwrap_or("").into());
            }
        }
    });
    imports.retain(|name, _| !wildcard && counts[name] == 1 && !shadows.contains(name));
    let resolve = |node: Node<'_>| {
        let spelling = text(node, source).replace(' ', "");
        let (root, rest) = spelling.split_once('.').unwrap_or((&spelling, ""));
        imports.get(root).map(|name| {
            if rest.is_empty() {
                name.clone()
            } else {
                format!("{name}.{rest}")
            }
        })
    };
    visit(root, &mut |node| {
        if node.kind() == "class_definition" {
            if let Some(bases) = node.child_by_field_name("superclasses") {
                for base in children(bases) {
                    if resolve(base).as_deref() == Some("unittest.TestCase") {
                        detection.mark(node, base, file, "python.unittest-case");
                    }
                }
            }
        }
        if node.kind() == "decorated_definition" {
            let Some(function) = node
                .child_by_field_name("definition")
                .filter(|n| n.kind() == "function_definition")
            else {
                return;
            };
            for decorator in children(node)
                .into_iter()
                .filter(|n| n.kind() == "decorator")
            {
                let Some(expr) = decorator.named_child(0) else {
                    continue;
                };
                let name = if expr.kind() == "call" {
                    expr.child_by_field_name("function").unwrap_or(expr)
                } else {
                    expr
                };
                if resolve(name).as_deref() == Some("pytest.fixture") {
                    detection.mark(function, decorator, file, "python.pytest-fixture");
                }
            }
        }
    });
}

// Deliberately file-wide: uncertain lexical rebinding keeps declarations visible.
fn bound_names(node: Node<'_>, source: &[u8], names: &mut std::collections::BTreeSet<String>) {
    if node.kind() == "identifier" {
        names.insert(text(node, source));
        return;
    }
    if node.kind() == "attribute" {
        if let Some(object) = node.child_by_field_name("object") {
            bound_names(object, source, names);
        }
        return;
    }
    for child in children(node) {
        bound_names(child, source, names);
    }
}
