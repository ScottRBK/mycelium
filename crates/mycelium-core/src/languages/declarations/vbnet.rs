use super::*;

pub(super) fn walk(
    node: Node<'_>,
    source: &[u8],
    file: &str,
    owner: Option<usize>,
    diagram: &mut ClassDiagram,
    detection: &frameworks::Detection,
) {
    if node.kind() == "enum_member" {
        if let (Some(index), Some(name)) = (owner, field(node, "name", source)) {
            enum_member(node, index, name, diagram);
        }
        return;
    }
    let mut owner = owner;
    let kind = match node.kind() {
        "class_block" => Some("class"),
        "interface_block" => Some("interface"),
        "structure_block" => Some("struct"),
        "module_block" => Some("module"),
        "enum_block" => Some("enum"),
        _ => None,
    };
    if let Some(kind) = kind {
        if let Some(name) = field(node, "name", source) {
            let index = add_class(diagram, file, name, kind, node.start_position().row + 1);
            for base in children(node)
                .into_iter()
                .filter(|n| matches!(n.kind(), "inherits_clause" | "implements_clause"))
            {
                for name in children(base) {
                    diagram.classes[index].bases.push(Base {
                        name: text(name, source),
                        relation: if base.kind() == "implements_clause" {
                            "implements"
                        } else {
                            "inherits"
                        }
                        .into(),
                    });
                }
            }
            // The vendored grammar only recognises base clauses on the class header line.
            // Recover explicit leading multiline clauses, never arbitrary method-body text.
            for line in node.utf8_text(source).unwrap_or("").lines().skip(1) {
                let line = line.trim();
                if line.is_empty() || line.starts_with('\'') {
                    continue;
                }
                let Some((keyword, names)) = line.split_once(char::is_whitespace) else {
                    break;
                };
                let relation = if keyword.eq_ignore_ascii_case("Implements") {
                    "implements"
                } else if keyword.eq_ignore_ascii_case("Inherits") {
                    "inherits"
                } else {
                    break;
                };
                for name in names.split(',') {
                    diagram.classes[index].bases.push(Base {
                        name: name.trim().to_string(),
                        relation: relation.into(),
                    });
                }
            }
            diagram.classes[index].test = detection.evidence(node);
            owner = Some(index);
        }
    }
    if matches!(
        node.kind(),
        "method_declaration"
            | "constructor_declaration"
            | "property_declaration"
            | "field_declaration"
    ) {
        let index = owner.unwrap_or_else(|| module(diagram, file));
        let method = matches!(
            node.kind(),
            "method_declaration" | "constructor_declaration"
        );
        let modifiers = field(node, "modifiers", source)
            .unwrap_or_default()
            .to_lowercase();
        let visibility = if modifiers.contains("private") {
            "-"
        } else if modifiers.contains("protected") {
            "#"
        } else if modifiers.contains("friend") {
            "~"
        } else {
            "+"
        };
        let members = if node.kind() == "field_declaration" {
            children(node)
                .into_iter()
                .filter(|n| n.kind() == "variable_declarator")
                .collect()
        } else {
            vec![node]
        };
        for member in members {
            let name = field(member, "name", source).unwrap_or_else(|| "New".into());
            if name.eq_ignore_ascii_case("Implements") || name.eq_ignore_ascii_case("Inherits") {
                continue;
            }
            let parameters = member
                .child_by_field_name("parameters")
                .map(|p| {
                    children(p)
                        .into_iter()
                        .map(|p| Parameter {
                            name: field(p, "name", source).unwrap_or_default(),
                            value_type: vb_type(p, source),
                        })
                        .collect()
                })
                .unwrap_or_default();
            diagram.classes[index].members.push(Member {
                test: detection.evidence(node),
                file: String::new(),
                name,
                kind: if method { "method" } else { "field" }.into(),
                visibility: visibility.into(),
                parameters,
                value_type: vb_type(member, source),
                line: member.start_position().row + 1,
                end_line: member.end_position().row + 1,
            });
        }
        return;
    }
    for child in children(node) {
        walk(child, source, file, owner, diagram, detection);
    }
}

fn vb_type(node: Node<'_>, source: &[u8]) -> Option<String> {
    if let Some(ret) = node.child_by_field_name("return_type") {
        return Some(text(ret, source).trim_start_matches("As ").to_string());
    }
    children(node)
        .into_iter()
        .find(|n| n.kind() == "as_clause")
        .and_then(|n| field(n, "type", source))
}
