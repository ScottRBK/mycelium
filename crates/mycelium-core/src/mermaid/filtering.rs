//! Export-time selection of raw source occurrences, before owner merging.

use std::collections::{BTreeMap, BTreeSet};

use super::{markdown_code, ExportError, MermaidOptions, TestMode};
use crate::config::AnalysisResult;
use crate::declarations::{Class, TestEvidence, TEST_DETECTOR_VERSION};

pub(super) struct TestFilter {
    test_paths: Vec<String>,
    keep_paths: Vec<String>,
    files: BTreeSet<String>,
    include: bool,
    detector_version: Option<u32>,
    explain: bool,
    pub notices: BTreeSet<String>,
    counts: BTreeMap<(String, String), usize>,
    explanations: BTreeSet<String>,
}

impl TestFilter {
    pub fn new(result: &AnalysisResult, options: &MermaidOptions) -> Result<Self, ExportError> {
        let scope = normalize_path(&options.path)?;
        let mut notices = BTreeSet::new();
        let normalize = |paths: &[String], notices: &mut BTreeSet<String>| {
            let mut normalized = BTreeSet::new();
            for path in paths {
                let path = normalize_path(path)?;
                let files: Vec<_> = result
                    .structure
                    .files
                    .iter()
                    .filter(|f| matches_path(&f.path, &path))
                    .collect();
                if files.is_empty() {
                    return Err(ExportError::NoMatchingTestPath(path));
                }
                if !files.iter().any(|f| matches_path(&f.path, &scope)) {
                    notices.insert(format!(
                        "Test/keep path is outside the selected scope: {path}"
                    ));
                }
                normalized.insert(path);
            }
            Ok(normalized.into_iter().collect())
        };
        let metadata = result
            .class_diagram
            .as_ref()
            .and_then(|d| d.test_detection.as_ref());
        let detector_version = metadata.map(|m| m.version);
        if options.tests == TestMode::Exclude {
            if !detector_version.is_some_and(|v| (1..=TEST_DETECTOR_VERSION).contains(&v)) {
                notices.insert(
                    concat!(
                        "Automatic test detection unavailable; rerun analysis. ",
                        "Explicit test/keep paths still apply."
                    )
                    .into(),
                );
            } else if let Some(metadata) = metadata {
                if metadata.version == 1 {
                    notices.insert(
                        concat!(
                            "Saved Rust/Go detection only; ",
                            "rerun analysis to enable framework detection."
                        )
                        .into(),
                    );
                }
                notices.extend(
                    metadata
                        .diagnostics
                        .iter()
                        .filter(|d| matches_path(&d.file, &scope))
                        .map(|d| format!("{}: {}", d.file, d.message)),
                );
            }
        }
        Ok(Self {
            files: result
                .structure
                .files
                .iter()
                .map(|f| f.path.clone())
                .collect(),
            detector_version,
            test_paths: normalize(&options.test_paths, &mut notices)?,
            keep_paths: normalize(&options.keep_paths, &mut notices)?,
            include: options.tests == TestMode::Include,
            explain: options.explain_tests,
            notices,
            counts: BTreeMap::new(),
            explanations: BTreeSet::new(),
        })
    }

    fn hidden(
        &mut self,
        file: &str,
        line: usize,
        name: &str,
        evidence: Option<&TestEvidence>,
        in_scope: bool,
    ) -> bool {
        if !self.valid_location(file, line) {
            self.notices
                .insert(format!("{file}:{line}: invalid source location; retained"));
            return false;
        }
        let automatic = evidence
            .filter(|_| {
                self.detector_version
                    .is_some_and(|v| (1..=TEST_DETECTOR_VERSION).contains(&v))
            })
            .filter(|e| {
                let valid = self.valid_location(&e.file, e.line)
                    && e.file == file
                    && self
                        .detector_version
                        .is_some_and(|v| crate::declarations::supported_test_rule(v, &e.rule));
                if !valid {
                    self.notices
                        .insert(format!("{file}:{line}: invalid test evidence; retained"));
                }
                valid
            });
        let rule = if self.test_paths.iter().any(|p| matches_path(file, p)) {
            Some("test-path")
        } else {
            automatic.map(|e| e.rule.as_str())
        };
        let test = rule.is_some();
        let keep = self.keep_paths.iter().any(|p| matches_path(file, p));
        if test && in_scope {
            let rule = if keep {
                "keep-path override"
            } else {
                rule.unwrap_or("")
            };
            *self.counts.entry((file.into(), rule.into())).or_default() += 1;
            if self.explain {
                self.explanations
                    .insert(format!("{file}:{line} {name}: {rule}"));
            }
        }
        test && !keep
    }

    fn valid_location(&self, file: &str, line: usize) -> bool {
        line > 0
            && !file.is_empty()
            && self.files.contains(file)
            && normalize_path(file).is_ok_and(|normalized| normalized == file)
    }

    pub fn retain(&mut self, raw: &[Class], in_scope: &BTreeSet<String>) -> Vec<Class> {
        if self.include {
            return raw.to_vec();
        }
        let mut retained = Vec::new();
        for class in raw {
            let hidden = self.hidden(
                &class.file,
                class.line,
                &class.name,
                class.test.as_ref(),
                in_scope.contains(&class.id),
            );
            let mut class = class.clone();
            class.members.retain(|m| {
                !self.hidden(
                    &m.file,
                    m.line,
                    &m.name,
                    m.test.as_ref(),
                    in_scope.contains(&class.id),
                )
            });
            let uncertain_members = class
                .members
                .iter()
                .any(|m| !self.valid_location(&m.file, m.line));
            if hidden && uncertain_members {
                class
                    .members
                    .retain(|m| !self.valid_location(&m.file, m.line));
                class.bases.clear();
            }
            if (!hidden || uncertain_members)
                && (class.kind != "module" || !class.members.is_empty())
            {
                retained.push(class);
            }
        }
        retained
    }

    pub fn summary(&self, filtered_calls: usize, filtered_edges: usize) -> String {
        if self.include {
            return String::new();
        }
        let mut out = String::from("## Test filtering\n\nMode: exclude.\n");
        out.push_str(&format!(
            "Saved detector version: {}.\n",
            self.detector_version
                .map(|v| v.to_string())
                .unwrap_or_else(|| "unavailable".into())
        ));
        for (label, paths) in [
            ("Test paths", &self.test_paths),
            ("Keep paths", &self.keep_paths),
        ] {
            out.push_str(&format!(
                "{label}: {}.\n",
                if paths.is_empty() {
                    "none".into()
                } else {
                    paths
                        .iter()
                        .map(|p| markdown_code(if p.is_empty() { "." } else { p }))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
        }
        out.push_str(&format!(
            "Calls removed by test filtering: {filtered_calls}. \
             Type relationships removed: {filtered_edges}.\n\n"
        ));
        for ((file, rule), count) in &self.counts {
            out.push_str(&format!(
                "- {}: {count} source occurrences ({rule}).\n",
                markdown_code(file)
            ));
        }
        for notice in &self.notices {
            out.push_str(&format!("- {}\n", markdown_code(notice)));
        }
        if self.explain && !self.explanations.is_empty() {
            out.push_str("\n<details>\n<summary>Test selection explanations</summary>\n\n");
            for explanation in &self.explanations {
                out.push_str(&format!("- {}\n", markdown_code(explanation)));
            }
            out.push_str("\n</details>\n");
        }
        out.push('\n');
        out
    }
}

pub(super) fn normalize_path(path: &str) -> Result<String, ExportError> {
    let path = path.replace('\\', "/");
    if path.starts_with('/') || path.split('/').any(|p| p == ".." || p.contains(':')) {
        return Err(ExportError::InvalidOptions);
    }
    Ok(path
        .split('/')
        .filter(|p| !p.is_empty() && *p != ".")
        .collect::<Vec<_>>()
        .join("/"))
}

pub(super) fn matches_path(file: &str, prefix: &str) -> bool {
    prefix.is_empty() || file == prefix || file.starts_with(&format!("{prefix}/"))
}
