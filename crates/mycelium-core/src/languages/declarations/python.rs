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
