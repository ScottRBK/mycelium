//! Language-specific declaration extraction reuses the parsing phase's syntax tree.

mod c_cpp;
pub(crate) mod frameworks;
mod go;
mod nominal;
mod python;
mod test_detection;
mod vbnet;

use tree_sitter::{Node, Tree};

use crate::declarations::{Base, Class, ClassDiagram, Member, Parameter};

pub(crate) fn extract(
    tree: &Tree,
    source: &[u8],
    file: &str,
    language: &str,
    bindings: &mut frameworks::Bindings,
) -> ClassDiagram {
    let mut diagram = ClassDiagram {
        test_detection: Some(crate::declarations::TestDetection::default()),
        ..Default::default()
    };
    if tree.root_node().has_error() {
        diagram.warnings.push(format!(
            "{file}: syntax errors; declarations may be incomplete"
        ));
    }
    let detection = frameworks::Detection::new(tree.root_node(), source, file, language);
    if language == "Rust" {
        let mut detection = test_detection::RustDetection::new(tree.root_node(), source);
        rust_walk(
            tree.root_node(),
            source,
            file,
            None,
            &mut diagram,
            &mut detection,
        );
        let message = if !detection.syntax_valid {
            Some("syntax-derived test detection disabled; malformed syntax")
        } else if detection.skipped_test {
            Some("test attribute binding is uncertain; #[test] detection disabled")
        } else {
            None
        };
        if let (Some(metadata), Some(message)) = (&mut diagram.test_detection, message) {
            metadata
                .diagnostics
                .push(crate::declarations::TestDiagnostic {
                    file: file.into(),
                    message: message.into(),
                });
        }
    } else if language == "Python" {
        diagram.python_bindings.insert(
            file.into(),
            python::bindings(tree.root_node(), source, file),
        );
        python::walk(
            tree.root_node(),
            source,
            file,
            None,
            &mut diagram,
            &detection,
        );
    } else if matches!(language, "C#" | "TypeScript" | "Java") {
        nominal::walk(
            tree.root_node(),
            source,
            file,
            None,
            &mut diagram,
            &detection,
        );
    } else if language == "Go" {
        go::walk(tree.root_node(), source, file, None, &mut diagram);
    } else if language == "VB.NET" {
        vbnet::walk(
            tree.root_node(),
            source,
            file,
            None,
            &mut diagram,
            &detection,
        );
    } else if matches!(language, "C" | "C++") {
        c_cpp::walk(tree.root_node(), source, file, None, &mut diagram);
    } else {
        diagram.warnings.push(format!(
            "{file}: declaration extraction unavailable for {language}"
        ));
    }
    if !detection.syntax_valid {
        diagram.test_detection.as_mut().unwrap().diagnostics.push(
            crate::declarations::TestDiagnostic {
                file: file.into(),
                message: "syntax-derived test detection disabled; malformed syntax".into(),
            },
        );
    }
    let go_test = language == "Go"
        && file
            .rsplit('/')
            .next()
            .is_some_and(|name| name.ends_with("_test.go") && !name.starts_with(['.', '_']));
    for class in &mut diagram.classes {
        if go_test {
            class.test = Some(crate::declarations::TestEvidence {
                rule: "go.test-file".into(),
                file: file.into(),
                line: 1,
            });
        }
        for member in &mut class.members {
            member.file = file.to_string();
            if go_test {
                member.test = class.test.clone();
            }
        }
    }
    bindings.extend(detection.bindings);
    diagram
}

fn text(node: Node<'_>, source: &[u8]) -> String {
    node.utf8_text(source)
        .unwrap_or("")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn field(node: Node<'_>, name: &str, source: &[u8]) -> Option<String> {
    node.child_by_field_name(name).map(|n| text(n, source))
}

fn children(node: Node<'_>) -> Vec<Node<'_>> {
    node.named_children(&mut node.walk())
        .filter(|n| !n.is_extra())
        .collect()
}

fn add_class(
    diagram: &mut ClassDiagram,
    file: &str,
    name: String,
    kind: &str,
    line: usize,
) -> usize {
    let index = diagram.classes.len();
    diagram.classes.push(Class {
        test: None,
        id: format!("{file}:{line}:{kind}:{name}"),
        name,
        kind: kind.to_string(),
        file: file.to_string(),
        line,
        members: Vec::new(),
        bases: Vec::new(),
    });
    index
}

fn module(diagram: &mut ClassDiagram, file: &str) -> usize {
    if let Some(index) = diagram
        .classes
        .iter()
        .position(|c| c.kind == "module" && c.file == file)
    {
        return index;
    }
    add_class(diagram, file, file.to_string(), "module", 1)
}

fn rust_walk(
    node: Node<'_>,
    source: &[u8],
    file: &str,
    owner: Option<usize>,
    diagram: &mut ClassDiagram,
    detection: &mut test_detection::RustDetection,
) {
    let mut owner = owner;
    let kind = match node.kind() {
        "struct_item" => Some("struct"),
        "enum_item" => Some("enum"),
        "trait_item" => Some("trait"),
        "impl_item" => Some("impl"),
        _ => None,
    };
    if let Some(kind) = kind {
        let name = field(node, if kind == "impl" { "type" } else { "name" }, source);
        if let Some(name) = name {
            let index = add_class(diagram, file, name, kind, node.start_position().row + 1);
            diagram.classes[index].test =
                test_detection::rust_evidence(node, source, file, detection);
            if let Some(name) = field(node, "trait", source) {
                diagram.classes[index].bases.push(Base {
                    name,
                    relation: "implements".into(),
                });
            }
            owner = Some(index);
        }
    }
    if node.kind() == "ordered_field_declaration_list" {
        if let Some(index) = owner {
            for (number, value) in node
                .children_by_field_name("type", &mut node.walk())
                .enumerate()
            {
                let public = value
                    .prev_named_sibling()
                    .is_some_and(|n| n.kind() == "visibility_modifier");
                diagram.classes[index].members.push(Member {
                    test: test_detection::rust_evidence(value, source, file, detection),
                    file: String::new(),
                    name: number.to_string(),
                    kind: "field".into(),
                    visibility: if public { "+" } else { "-" }.into(),
                    parameters: Vec::new(),
                    value_type: Some(text(value, source)),
                    line: value.start_position().row + 1,
                    end_line: value.end_position().row + 1,
                });
            }
        }
        return;
    }
    if matches!(
        node.kind(),
        "function_item" | "function_signature_item" | "field_declaration" | "enum_variant"
    ) {
        if let Some(name) = field(node, "name", source) {
            let index = owner.unwrap_or_else(|| module(diagram, file));
            let method = matches!(node.kind(), "function_item" | "function_signature_item");
            let parameters = node
                .child_by_field_name("parameters")
                .map(|p| {
                    children(p)
                        .into_iter()
                        .filter(|p| p.kind() == "parameter")
                        .map(|p| Parameter {
                            name: field(p, "pattern", source).unwrap_or_default(),
                            value_type: field(p, "type", source),
                        })
                        .collect()
                })
                .unwrap_or_default();
            let public = children(node)
                .iter()
                .any(|n| n.kind() == "visibility_modifier")
                || diagram.classes[index].kind == "trait"
                || !diagram.classes[index].bases.is_empty();
            diagram.classes[index].members.push(Member {
                test: test_detection::rust_evidence(node, source, file, detection),
                file: String::new(),
                name,
                kind: if method { "method" } else { "field" }.into(),
                visibility: if public { "+" } else { "-" }.into(),
                parameters,
                value_type: field(node, if method { "return_type" } else { "type" }, source)
                    .or_else(|| {
                        (node.kind() == "enum_variant")
                            .then(|| field(node, "body", source))
                            .flatten()
                    })
                    .or_else(|| method.then(|| "()".into())),
                line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
        return;
    }
    for child in children(node) {
        rust_walk(child, source, file, owner, diagram, detection);
    }
}

fn enum_member(node: Node<'_>, owner: usize, name: String, diagram: &mut ClassDiagram) {
    let value_type = Some(diagram.classes[owner].name.clone());
    diagram.classes[owner].members.push(Member {
        test: None,
        file: String::new(),
        name,
        kind: "field".into(),
        visibility: String::new(),
        parameters: Vec::new(),
        value_type,
        line: node.start_position().row + 1,
        end_line: node.end_position().row + 1,
    });
}
