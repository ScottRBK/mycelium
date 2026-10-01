//! Conservative framework bindings, independent of the heuristic call resolver.
mod attributes;
mod java;
mod javascript;
mod python;

use super::{children, field, text};
use crate::declarations::TestEvidence;
use std::collections::{BTreeMap, BTreeSet};
use tree_sitter::Node;

#[derive(Default)]
pub(super) struct Detection {
    marks: BTreeMap<usize, TestEvidence>,
    pub bindings: Bindings,
    pub syntax_valid: bool,
}
impl Detection {
    pub fn new(root: Node<'_>, source: &[u8], file: &str, language: &str) -> Self {
        let mut detection = Self {
            syntax_valid: !matches!(language, "Python" | "C#" | "VB.NET" | "Java" | "TypeScript")
                || !super::test_detection::malformed(root),
            ..Self::default()
        };
        match language {
            "C#" | "VB.NET" => {
                attributes::detect(root, source, file, &mut detection, language == "VB.NET");
            }
            "Java" => java::detect(root, source, file, &mut detection),
            "Python" if detection.syntax_valid => {
                python::detect(root, source, file, &mut detection);
            }
            "TypeScript" if detection.syntax_valid => {
                javascript::detect(root, source, file, &mut detection);
            }
            _ => {}
        }
        detection
    }
    fn mark(&mut self, node: Node<'_>, marker: Node<'_>, file: &str, rule: &str) {
        self.marks.insert(
            node.id(),
            TestEvidence {
                rule: rule.into(),
                file: file.into(),
                line: marker.start_position().row + 1,
            },
        );
    }
    pub fn evidence(&self, node: Node<'_>) -> Option<TestEvidence> {
        let mut current = Some(node);
        let mut crossed_type = false;
        while let Some(n) = current {
            if let Some(mark) = self.marks.get(&n.id()) {
                if !crossed_type || mark.rule.starts_with("javascript.") {
                    return Some(mark.clone());
                }
            }
            // Nested types have independent identities.
            if n.kind() == "class_definition" || attributes::type_node(n) {
                crossed_type = true;
            }
            current = n.parent();
        }
        None
    }
}
fn visit<'tree>(node: Node<'tree>, callback: &mut impl FnMut(Node<'tree>)) {
    callback(node);
    for child in children(node) {
        visit(child, callback);
    }
}

/// Apply repository-wide conflicts only after every file's declaration facts are available.
#[derive(Default)]
pub(crate) struct Bindings {
    declared: BTreeSet<String>,
    guards: Vec<(TestEvidence, BTreeSet<String>)>,
}
impl Bindings {
    pub fn extend(&mut self, other: Self) {
        self.declared.extend(other.declared);
        self.guards.extend(other.guards);
    }
}

pub(crate) fn finish(
    diagram: &mut crate::declarations::ClassDiagram,
    files: &[String],
    bindings: Bindings,
) {
    let conflicts: BTreeSet<_> = bindings
        .guards
        .into_iter()
        .filter(|(_, names)| names.iter().any(|n| bindings.declared.contains(n)))
        .map(|(evidence, _)| evidence)
        .collect();
    let local_module = |module: &str| {
        files.iter().any(|file| {
            file.rsplit('/').next() == Some(&format!("{module}.py"))
                || file.split('/').any(|part| part == module)
        })
    };
    let unittest = local_module("unittest");
    let pytest = local_module("pytest");
    let reject = |e: &Option<TestEvidence>| {
        e.as_ref().is_some_and(|e| {
            conflicts.contains(e)
                || (unittest && e.rule == "python.unittest-case")
                || (pytest && e.rule == "python.pytest-fixture")
        })
    };
    let mut uncertain = std::collections::BTreeSet::new();
    for class in &mut diagram.classes {
        if reject(&class.test) {
            uncertain.insert(class.file.clone());
            class.test = None;
        }
        for member in &mut class.members {
            if reject(&member.test) {
                uncertain.insert(member.file.clone());
                member.test = None;
            }
        }
    }
    if let Some(metadata) = &mut diagram.test_detection {
        metadata
            .diagnostics
            .extend(
                uncertain
                    .into_iter()
                    .map(|file| crate::declarations::TestDiagnostic {
                        file,
                        message:
                            "framework binding conflicts with a repository declaration; retained"
                                .into(),
                    }),
            );
    }
}

fn keyword(node: Node<'_>, name: &str) -> bool {
    node.children(&mut node.walk())
        .any(|child| child.kind() == name)
}
