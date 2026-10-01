use super::*;

pub(super) fn walk(
    node: Node<'_>,
    source: &[u8],
    file: &str,
    owner: Option<usize>,
    diagram: &mut ClassDiagram,
) {
    let mut owner = owner;
    if node.kind() == "type_spec" {
        if let (Some(name), Some(value)) = (
            field(node, "name", source),
            node.child_by_field_name("type"),
        ) {
            let kind = match value.kind() {
                "struct_type" => "struct",
                "interface_type" => "interface",
                _ => "type_alias",
            };
            owner = Some(add_class(
                diagram,
                file,
                name,
                kind,
                node.start_position().row + 1,
            ));
        }
    }
    if node.kind() == "type_elem" {
        if let Some(index) = owner {
            let types = children(node);
            if types.len() == 1
                && matches!(
                    types[0].kind(),
                    "type_identifier" | "qualified_type" | "generic_type"
                )
            {
                diagram.classes[index].bases.push(Base {
                    name: text(types[0], source),
                    relation: "inherits".into(),
                });
            }
        }
        return;
    }
    let method = matches!(
        node.kind(),
        "method_declaration" | "function_declaration" | "method_elem"
    );
    if method || node.kind() == "field_declaration" {
        if let Some(receiver) = node.child_by_field_name("receiver") {
            if let Some(parameter) = children(receiver).first() {
                let name = field(*parameter, "type", source)
                    .unwrap_or_default()
                    .trim_start_matches('*')
                    .to_string();
                owner = Some(add_class(
                    diagram,
                    file,
                    name,
                    "impl",
                    node.start_position().row + 1,
                ));
            }
        }
        let index = owner.unwrap_or_else(|| module(diagram, file));
        let mut names: Vec<_> = node
            .children_by_field_name("name", &mut node.walk())
            .map(|n| text(n, source))
            .collect();
        if !method && names.is_empty() {
            if let Some(value) = field(node, "type", source) {
                names.push(
                    value
                        .split('[')
                        .next()
                        .unwrap_or(&value)
                        .trim_start_matches('*')
                        .rsplit('.')
                        .next()
                        .unwrap_or(&value)
                        .to_string(),
                );
            }
        }
        let parameters = node
            .child_by_field_name("parameters")
            .map(|p| {
                children(p)
                    .into_iter()
                    .flat_map(|p| {
                        let mut names: Vec<_> = p
                            .children_by_field_name("name", &mut p.walk())
                            .map(|n| text(n, source))
                            .collect();
                        if names.is_empty() {
                            names.push("_".into());
                        }
                        names.into_iter().map(move |name| Parameter {
                            name,
                            value_type: field(p, "type", source),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        for name in names {
            diagram.classes[index].members.push(Member {
                test: None,
                file: String::new(),
                visibility: if name.chars().next().is_some_and(char::is_uppercase) {
                    "+"
                } else {
                    "~"
                }
                .into(),
                name,
                kind: if method { "method" } else { "field" }.into(),
                parameters: parameters.clone(),
                value_type: field(node, if method { "result" } else { "type" }, source)
                    .map(|value| {
                        if !method && node.child(0).is_some_and(|n| n.kind() == "*") {
                            format!("*{value}")
                        } else {
                            value
                        }
                    })
                    .or_else(|| method.then(|| "()".into())),
                line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
        return;
    }
    for child in children(node) {
        walk(child, source, file, owner, diagram);
    }
}
