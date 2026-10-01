//! Narrow language evidence; no names, call heuristics, or runtime discovery.

use crate::declarations::TestEvidence;
use tree_sitter::Node;

pub(super) struct RustDetection {
    pub syntax_valid: bool,
    pub uncertain_test_binding: bool,
    pub skipped_test: bool,
}

impl RustDetection {
    pub fn new(root: Node<'_>, source: &[u8]) -> Self {
        Self {
            syntax_valid: !malformed(root),
            skipped_test: false,
            // Without import binding, a wildcard or an imported test attribute is uncertain.
            // Conservatively disable only #[test] throughout this file; cfg(test) is independent.
            uncertain_test_binding: uncertain_import(root, source),
        }
    }
}

pub(super) fn malformed(node: Node<'_>) -> bool {
    node.has_error() || node.is_missing() || node.children(&mut node.walk()).any(malformed)
}

fn uncertain_import(node: Node<'_>, source: &[u8]) -> bool {
    if node.kind() == "use_declaration" {
        return node
            .child_by_field_name("argument")
            .is_some_and(|n| binds_test(n, source));
    }
    node.named_children(&mut node.walk())
        .any(|n| uncertain_import(n, source))
}

fn binds_test(node: Node<'_>, source: &[u8]) -> bool {
    match node.kind() {
        "use_wildcard" => true,
        "use_as_clause" => node
            .child_by_field_name("alias")
            .is_some_and(|n| matches!(tokens(n, source).as_str(), "test" | "r#test")),
        "scoped_identifier" => node
            .child_by_field_name("name")
            .is_some_and(|n| binds_test(n, source)),
        "identifier" => matches!(tokens(node, source).as_str(), "test" | "r#test"),
        "scoped_use_list" => node
            .child_by_field_name("list")
            .is_some_and(|n| binds_test(n, source)),
        "use_list" => node
            .named_children(&mut node.walk())
            .any(|n| binds_test(n, source)),
        _ => false,
    }
}

pub(super) fn rust_evidence(
    node: Node<'_>,
    source: &[u8],
    file: &str,
    detection: &mut RustDetection,
) -> Option<TestEvidence> {
    if !detection.syntax_valid {
        return None;
    }
    let mut skipped_test = false;
    let mut current = Some(node);
    while let Some(node) = current {
        let mut previous = node.prev_named_sibling();
        while let Some(attr) = previous {
            if attr.kind().contains("comment")
                || (attr.kind() == "visibility_modifier"
                    && node
                        .parent()
                        .is_some_and(|p| p.kind() == "ordered_field_declaration_list"))
            {
                previous = attr.prev_named_sibling();
                continue;
            }
            if attr.kind() != "attribute_item" {
                break;
            }
            let spelling = tokens(attr, source);
            if spelling == "#[test]"
                && node.kind() == "function_item"
                && detection.uncertain_test_binding
            {
                skipped_test = true;
            }
            let rule = if spelling == "#[cfg(test)]" {
                Some("rust.cfg-test")
            } else if spelling == "#[test]"
                && node.kind() == "function_item"
                && !detection.uncertain_test_binding
            {
                Some("rust.test")
            } else {
                None
            };
            if let Some(rule) = rule {
                return Some(evidence(rule, attr, file));
            }
            previous = attr.prev_named_sibling();
        }
        // Inner attributes apply to their enclosing item, including a crate or inline module.
        for container in [Some(node), node.child_by_field_name("body")]
            .into_iter()
            .flatten()
        {
            for attr in container.named_children(&mut container.walk()) {
                // Inner attributes precede the scope's items; do not rescan a whole file for
                // every declaration in it.
                if attr.kind() != "inner_attribute_item" {
                    if attr.is_extra() || attr.kind() == "shebang" {
                        continue;
                    }
                    break;
                }
                if attr.kind() == "inner_attribute_item" && tokens(attr, source) == "#![cfg(test)]"
                {
                    return Some(evidence("rust.cfg-test", attr, file));
                }
            }
        }
        current = node.parent();
    }
    detection.skipped_test |= skipped_test;
    None
}

fn evidence(rule: &str, node: Node<'_>, file: &str) -> TestEvidence {
    TestEvidence {
        rule: rule.into(),
        file: file.into(),
        line: node.start_position().row + 1,
    }
}

fn tokens(node: Node<'_>, source: &[u8]) -> String {
    if node.kind().contains("comment") {
        return String::new();
    }
    if node.child_count() == 0 {
        return node.utf8_text(source).unwrap_or("").into();
    }
    node.children(&mut node.walk())
        .map(|child| tokens(child, source))
        .collect()
}
