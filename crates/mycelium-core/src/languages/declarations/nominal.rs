//! Nominal classes: C#, Java, TypeScript and JavaScript share declaration shapes.
use super::*;

pub(super) fn walk(
    node: Node<'_>,
    source: &[u8],
    file: &str,
    owner: Option<usize>,
    diagram: &mut ClassDiagram,
    detection: &frameworks::Detection,
) {
    let mut owner = owner;
    let kind = match node.kind() {
        "class_declaration" | "abstract_class_declaration" => Some("class"),
        "struct_declaration" => Some("struct"),
        "interface_declaration" => Some("interface"),
        "enum_declaration" => Some("enum"),
        "record_declaration" => Some("record"),
        "type_alias_declaration" => Some("type_alias"),
        _ => None,
    };
    if let Some(kind) = kind {
        if let Some(name) = field(node, "name", source) {
            let index = add_class(diagram, file, name, kind, node.start_position().row + 1);
            for base_list in children(node).into_iter().filter(|n| {
                matches!(
                    n.kind(),
                    "base_list"
                        | "class_heritage"
                        | "extends_type_clause"
                        | "superclass"
                        | "super_interfaces"
                        | "extends_interfaces"
                )
            }) {
                for base in base_types(base_list) {
                    diagram.classes[index].bases.push(Base {
                        name: text(base, source),
                        relation: "inherits".into(),
                    });
                }
            }
            if kind == "type_alias" {
                if let Some(value) = node.child_by_field_name("value") {
                    if value.kind() != "object_type" {
                        add_field(
                            node,
                            source,
                            index,
                            "alias".into(),
                            Some(text(value, source)),
                            diagram,
                            detection,
                        );
                    }
                }
            }
            if kind == "record" {
                for params in children(node)
                    .into_iter()
                    .filter(|n| matches!(n.kind(), "parameter_list" | "formal_parameters"))
                {
                    for param in children(params).into_iter().filter(|p| !p.is_extra()) {
                        if let Some(name) = field(param, "name", source) {
                            add_field(
                                param,
                                source,
                                index,
                                name,
                                type_field(param, "type", source),
                                diagram,
                                detection,
                            );
                        }
                    }
                }
            }
            diagram.classes[index].test = detection.evidence(node);
            owner = Some(index);
        }
    }
    if let Some(index) = owner {
        if matches!(
            node.kind(),
            "enum_member_declaration" | "enum_constant" | "enum_assignment"
        ) {
            if let Some(name) = field(node, "name", source) {
                enum_member(node, index, name, diagram);
            }
            return;
        }
        if node.kind() == "enum_body" {
            for name in node.children_by_field_name("name", &mut node.walk()) {
                enum_member(name, index, text(name, source), diagram);
            }
        }
    }
    if matches!(
        node.kind(),
        "method_declaration"
            | "constructor_declaration"
            | "method_definition"
            | "method_signature"
            | "abstract_method_signature"
            | "function_declaration"
            | "function_signature"
    ) {
        let index = owner.unwrap_or_else(|| module(diagram, file));
        if let Some(name) = field(node, "name", source) {
            let parameters = node
                .child_by_field_name("parameters")
                .map(|p| {
                    children(p)
                        .into_iter()
                        .filter(|p| !p.is_extra())
                        .map(|p| Parameter {
                            name: field(p, "name", source)
                                .or_else(|| field(p, "pattern", source))
                                .unwrap_or_else(|| text(p, source)),
                            value_type: type_field(p, "type", source),
                        })
                        .collect()
                })
                .unwrap_or_default();
            if name == "constructor" {
                if let Some(params) = node.child_by_field_name("parameters") {
                    for param in children(params) {
                        let parameter_property = children(param)
                            .iter()
                            .any(|n| n.kind() == "accessibility_modifier")
                            || text(param, source).starts_with("readonly ");
                        if parameter_property {
                            if let Some(name) = field(param, "pattern", source)
                                .or_else(|| field(param, "name", source))
                            {
                                add_field(
                                    param,
                                    source,
                                    index,
                                    name,
                                    type_field(param, "type", source),
                                    diagram,
                                    detection,
                                );
                            }
                        }
                    }
                }
            }
            let public = diagram.classes[index].kind == "interface";
            diagram.classes[index].members.push(Member {
                test: detection.evidence(node),
                file: String::new(),
                name,
                kind: "method".into(),
                visibility: visibility(node, source, public),
                parameters,
                value_type: field(node, "returns", source)
                    .or_else(|| type_field(node, "return_type", source))
                    .or_else(|| field(node, "type", source)),
                line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
        return;
    }
    if matches!(
        node.kind(),
        "property_declaration"
            | "public_field_definition"
            | "property_signature"
            | "field_definition"
    ) {
        if let (Some(index), Some(name)) = (
            owner,
            field(node, "name", source).or_else(|| field(node, "property", source)),
        ) {
            add_field(
                node,
                source,
                index,
                name,
                type_field(node, "type", source),
                diagram,
                detection,
            );
        }
        return;
    }
    if node.kind() == "variable_declarator" {
        if let Some(value) = node.child_by_field_name("value") {
            if matches!(value.kind(), "arrow_function" | "function_expression") {
                let index = owner.unwrap_or_else(|| module(diagram, file));
                let name = field(node, "name", source).unwrap_or_default();
                let parameters = value
                    .child_by_field_name("parameters")
                    .map(|p| {
                        children(p)
                            .into_iter()
                            .map(|p| Parameter {
                                name: field(p, "pattern", source)
                                    .or_else(|| field(p, "name", source))
                                    .unwrap_or_else(|| text(p, source)),
                                value_type: type_field(p, "type", source),
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                diagram.classes[index].members.push(Member {
                    test: detection.evidence(node),
                    file: String::new(),
                    name,
                    kind: "method".into(),
                    visibility: String::new(),
                    parameters,
                    value_type: type_field(value, "return_type", source),
                    line: node.start_position().row + 1,
                    end_line: node.end_position().row + 1,
                });
            }
        }
        return;
    }
    if node.kind() == "field_declaration" {
        if let Some(index) = owner {
            for variable in children(node)
                .into_iter()
                .filter(|n| n.kind() == "variable_declarator")
            {
                if let Some(name) = field(variable, "name", source) {
                    add_field(
                        node,
                        source,
                        index,
                        name,
                        field(node, "type", source),
                        diagram,
                        detection,
                    );
                }
            }
            for declaration in children(node)
                .into_iter()
                .filter(|n| n.kind() == "variable_declaration")
            {
                for variable in children(declaration)
                    .into_iter()
                    .filter(|n| n.kind() == "variable_declarator")
                {
                    if let Some(name) = field(variable, "name", source) {
                        add_field(
                            node,
                            source,
                            index,
                            name,
                            field(declaration, "type", source),
                            diagram,
                            detection,
                        );
                    }
                }
            }
        }
        return;
    }
    for child in children(node) {
        walk(child, source, file, owner, diagram, detection);
    }
}

fn visibility(node: Node<'_>, source: &[u8], public_default: bool) -> String {
    let modifiers = children(node)
        .into_iter()
        .filter(|n| {
            matches!(
                n.kind(),
                "modifier" | "modifiers" | "accessibility_modifier"
            )
        })
        .map(|n| text(n, source))
        .collect::<Vec<_>>()
        .join(" ");
    if modifiers.contains("private") {
        "-"
    } else if modifiers.contains("protected") {
        "#"
    } else if modifiers.contains("public") || public_default {
        "+"
    } else if modifiers.contains("internal") {
        "~"
    } else {
        ""
    }
    .into()
}

fn add_field(
    node: Node<'_>,
    source: &[u8],
    index: usize,
    name: String,
    value_type: Option<String>,
    diagram: &mut ClassDiagram,
    detection: &frameworks::Detection,
) {
    diagram.classes[index].members.push(Member {
        test: detection.evidence(node),
        file: String::new(),
        name,
        kind: "field".into(),
        visibility: visibility(node, source, false),
        parameters: Vec::new(),
        value_type,
        line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
    });
}

fn type_field(node: Node<'_>, name: &str, source: &[u8]) -> Option<String> {
    field(node, name, source).map(|s| s.trim_start_matches(':').trim().to_string())
}

fn base_types(node: Node<'_>) -> Vec<Node<'_>> {
    if node.kind() == "extends_clause" {
        return node
            .children_by_field_name("value", &mut node.walk())
            .collect();
    }
    children(node)
        .into_iter()
        .flat_map(|child| {
            if matches!(
                child.kind(),
                "extends_clause" | "implements_clause" | "type_list"
            ) {
                base_types(child)
            } else {
                vec![child]
            }
        })
        .collect()
}
