use super::*;

pub(super) fn walk(
    node: Node<'_>,
    source: &[u8],
    file: &str,
    owner: Option<usize>,
    diagram: &mut ClassDiagram,
) {
    if node.kind() == "enumerator" {
        if let (Some(index), Some(name)) = (owner, field(node, "name", source)) {
            enum_member(node, index, name, diagram);
        }
        return;
    }
    let mut owner = owner;
    if matches!(
        node.kind(),
        "class_specifier" | "struct_specifier" | "enum_specifier" | "union_specifier"
    ) {
        // A specifier without a body is a reference (or forward declaration), not a new type.
        if node.child_by_field_name("body").is_none() {
            return;
        }
        let name = field(node, "name", source).or_else(|| {
            node.parent()
                .filter(|p| p.kind() == "type_definition")
                .and_then(|p| p.child_by_field_name("declarator"))
                .map(|n| text(n, source))
        });
        if let Some(name) = name {
            let kind = node.kind().trim_end_matches("_specifier");
            let index = add_class(diagram, file, name, kind, node.start_position().row + 1);
            for base_list in children(node)
                .into_iter()
                .filter(|n| n.kind() == "base_class_clause")
            {
                for base in children(base_list)
                    .into_iter()
                    .filter(|n| n.kind() != "access_specifier")
                {
                    diagram.classes[index].bases.push(Base {
                        name: text(base, source),
                        relation: "inherits".into(),
                    });
                }
            }
            owner = Some(index);
        }
    }
    if matches!(
        node.kind(),
        "function_definition" | "declaration" | "field_declaration"
    ) {
        let declarators: Vec<_> = node
            .children_by_field_name("declarator", &mut node.walk())
            .collect();
        for declarator in declarators {
            let function = find_function(declarator);
            let leaf = function.unwrap_or(declarator);
            let raw_name = declarator_name(leaf, source);
            let Some(raw_name) = raw_name else {
                continue;
            };
            let (index, name) = if let Some((class, method)) = raw_name.rsplit_once("::") {
                (
                    add_class(
                        diagram,
                        file,
                        class.into(),
                        "impl",
                        node.start_position().row + 1,
                    ),
                    method.to_string(),
                )
            } else {
                (owner.unwrap_or_else(|| module(diagram, file)), raw_name)
            };
            let parameters = function
                .and_then(|f| f.child_by_field_name("parameters"))
                .map(|p| {
                    children(p)
                        .into_iter()
                        .filter(|p| p.kind().contains("parameter_declaration"))
                        .map(|p| Parameter {
                            name: p
                                .child_by_field_name("declarator")
                                .and_then(|n| declarator_name(n, source))
                                .unwrap_or_else(|| "_".into()),
                            value_type: declaration_type(
                                p,
                                p.child_by_field_name("declarator"),
                                source,
                            ),
                        })
                        .collect()
                })
                .unwrap_or_default();
            diagram.classes[index].members.push(Member {
                test: None,
                file: String::new(),
                name,
                kind: if function.is_some() {
                    "method"
                } else {
                    "field"
                }
                .into(),
                visibility: String::new(),
                parameters,
                value_type: declaration_type(node, Some(declarator), source),
                line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
        // Inline struct bodies in typedefs/declarations still need traversal.
        if let Some(value) = node.child_by_field_name("type") {
            if value.kind().ends_with("_specifier") {
                walk(value, source, file, owner, diagram);
            }
        }
        return;
    }
    for child in children(node) {
        walk(child, source, file, owner, diagram);
    }
}

fn find_function(node: Node<'_>) -> Option<Node<'_>> {
    if node.kind() == "function_declarator" {
        return Some(node);
    }
    node.child_by_field_name("declarator")
        .and_then(find_function)
}

fn declarator_name(node: Node<'_>, source: &[u8]) -> Option<String> {
    if matches!(
        node.kind(),
        "identifier"
            | "field_identifier"
            | "qualified_identifier"
            | "destructor_name"
            | "operator_name"
    ) {
        return Some(text(node, source));
    }
    node.child_by_field_name("declarator")
        .and_then(|n| declarator_name(n, source))
}

fn declaration_type(
    node: Node<'_>,
    mut declarator: Option<Node<'_>>,
    source: &[u8],
) -> Option<String> {
    let mut value = field(node, "type", source)?;
    let qualifiers = children(node)
        .into_iter()
        .filter(|n| n.kind() == "type_qualifier")
        .map(|n| text(n, source))
        .collect::<Vec<_>>()
        .join(" ");
    if !qualifiers.is_empty() {
        value = format!("{qualifiers} {value}");
    }
    while let Some(n) = declarator {
        match n.kind() {
            "pointer_declarator" => value.push('*'),
            "reference_declarator" => value.push('&'),
            "array_declarator" => value.push_str("[]"),
            "function_declarator" => break,
            _ => (),
        }
        declarator = n.child_by_field_name("declarator");
    }
    Some(value)
}
