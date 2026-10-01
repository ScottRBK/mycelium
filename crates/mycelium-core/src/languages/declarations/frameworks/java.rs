use super::*;

pub(super) fn detect(root: Node<'_>, source: &[u8], file: &str, detection: &mut Detection) {
    let package = children(root)
        .into_iter()
        .find(|n| n.kind() == "package_declaration")
        .and_then(|n| {
            children(n)
                .into_iter()
                .find(|c| matches!(c.kind(), "identifier" | "scoped_identifier"))
        })
        .map(|n| text(n, source))
        .unwrap_or_default();
    let scope = |node| {
        let nested = attributes::scope(root, node, source);
        [package.as_str(), nested.as_str()]
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(".")
    };
    let mut imports: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for import in children(root)
        .into_iter()
        .filter(|n| n.kind() == "import_declaration")
    {
        let raw = text(import, source);
        if raw.contains('*') {
            continue;
        }
        if let Some(name) = children(import).first().map(|n| text(*n, source)) {
            imports
                .entry(name.rsplit('.').next().unwrap_or("").into())
                .or_default()
                .insert(name);
        }
    }
    visit(root, &mut |node| {
        if attributes::type_node(node) {
            if let Some(name) = field(node, "name", source) {
                detection
                    .bindings
                    .declared
                    .insert(format!("java:{}.{}", scope(node), name).replace("java:.", "java:"));
            }
        }
    });
    // Recovered declarations can disprove a binding in another file, but cannot prove tests.
    if !detection.syntax_valid {
        return;
    }
    visit(root, &mut |node| {
        if node.kind() != "method_declaration" || attributes::inherited_scope(node) {
            return;
        }
        for modifiers in children(node)
            .into_iter()
            .filter(|n| n.kind() == "modifiers")
        {
            for attr in children(modifiers)
                .into_iter()
                .filter(|n| matches!(n.kind(), "annotation" | "marker_annotation"))
            {
                let Some(name) = field(attr, "name", source) else {
                    continue;
                };
                let resolved = if name.contains('.') {
                    name.clone()
                } else {
                    let Some(values) = imports.get(&name).filter(|v| v.len() == 1) else {
                        continue;
                    };
                    values.first().unwrap().clone()
                };
                if !matches!(
                    resolved.as_str(),
                    "org.junit.Test"
                        | "org.junit.jupiter.api.Test"
                        | "org.junit.jupiter.api.RepeatedTest"
                        | "org.junit.jupiter.api.TestFactory"
                        | "org.junit.jupiter.api.TestTemplate"
                        | "org.junit.jupiter.params.ParameterizedTest"
                ) {
                    continue;
                }
                let mut guards = BTreeSet::from([format!("java:{resolved}")]);
                let current = scope(node);
                let mut prefix = current.as_str();
                loop {
                    let head = name.split('.').next().unwrap_or("");
                    guards.insert(format!("java:{}.{}", prefix, head).replace("java:.", "java:"));
                    if prefix.is_empty() {
                        break;
                    }
                    prefix = prefix.rsplit_once('.').map(|(p, _)| p).unwrap_or("");
                }
                detection.mark(node, attr, file, "java.junit-method");
                detection
                    .bindings
                    .guards
                    .push((detection.marks[&node.id()].clone(), guards));
            }
        }
    });
}
