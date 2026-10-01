//! Deterministic Markdown export of source declarations and recorded call edges.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use crate::config::AnalysisResult;
use crate::declarations::{Class, Member};

mod filtering;
use filtering::TestFilter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TestMode {
    #[default]
    Exclude,
    Include,
}

impl std::str::FromStr for TestMode {
    type Err = ExportError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "exclude" => Ok(Self::Exclude),
            "include" => Ok(Self::Include),
            _ => Err(ExportError::InvalidTestMode),
        }
    }
}

#[derive(Debug)]
pub struct MermaidExport {
    pub markdown: String,
    /// Option-validation notices, also available when include mode preserves the Markdown.
    pub notices: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct MermaidOptions {
    /// Repository-relative file or directory prefix. Empty selects all files.
    pub path: String,
    pub max_classes: usize,
    pub tests: TestMode,
    pub explain_tests: bool,
    pub test_paths: Vec<String>,
    pub keep_paths: Vec<String>,
}

impl Default for MermaidOptions {
    fn default() -> Self {
        Self {
            path: String::new(),
            max_classes: 8,
            tests: TestMode::Exclude,
            explain_tests: false,
            test_paths: Vec::new(),
            keep_paths: Vec::new(),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ExportError {
    MissingDeclarations,
    InvalidOptions,
    NoMatchingPath,
    NoMatchingTestPath(String),
    InvalidTestMode,
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoMatchingTestPath(path) => {
                write!(f, "test/keep path matches no analysed file: {path}")
            }
            Self::InvalidTestMode => f.write_str("tests must be exclude or include"),
            Self::NoMatchingPath => f.write_str("no declarations match the requested path"),
            Self::MissingDeclarations => {
                f.write_str("map has no class declarations; rerun analysis")
            }
            Self::InvalidOptions => {
                f.write_str("max_classes must be positive and paths must be repository-relative")
            }
        }
    }
}
impl std::error::Error for ExportError {}

/// Export only source facts; timestamps, timings, communities and machine paths never enter output.
pub fn export_mermaid(
    result: &AnalysisResult,
    options: &MermaidOptions,
) -> Result<String, ExportError> {
    Ok(export_mermaid_report(result, options)?.markdown)
}

pub fn export_mermaid_report(
    result: &AnalysisResult,
    options: &MermaidOptions,
) -> Result<MermaidExport, ExportError> {
    let prefix = filtering::normalize_path(&options.path)?;
    if options.max_classes == 0 {
        return Err(ExportError::InvalidOptions);
    }
    let diagram = result
        .class_diagram
        .as_ref()
        .ok_or(ExportError::MissingDeclarations)?;
    let mut filter = TestFilter::new(result, options)?;
    let raw = unique_occurrences(&diagram.classes);
    let mut warnings: BTreeSet<String> = diagram.warnings.iter().cloned().collect();
    let all_classes = merge_classes(&raw, &raw, &mut BTreeSet::new());
    let owners: Vec<_> = raw.iter().filter(|c| c.kind != "impl").cloned().collect();
    let owner_names = name_index(&owners);
    let in_scope: BTreeSet<_> = raw
        .iter()
        .filter(|c| {
            let file = if c.kind == "impl" {
                resolve(&owners, &owner_names, &c.file, &c.name)
                    .map(|i| owners[i].file.as_str())
                    .unwrap_or(&c.file)
            } else {
                &c.file
            };
            filtering::matches_path(file, &prefix)
        })
        .map(|c| c.id.clone())
        .collect();
    let retained = filter.retain(&raw, &in_scope);
    let mut classes = merge_classes(&retained, &raw, &mut warnings);
    let mut scoped: Vec<_> = all_classes
        .iter()
        .filter(|c| filtering::matches_path(&c.file, &prefix))
        .collect();
    if !prefix.is_empty() && scoped.is_empty() {
        return Err(ExportError::NoMatchingPath);
    }
    scoped.sort_by(|a, b| (&a.file, &a.name, a.line).cmp(&(&b.file, &b.name, b.line)));
    let stable_ids: BTreeMap<_, _> = scoped
        .iter()
        .enumerate()
        .map(|(i, c)| (c.id.as_str(), format!("c{i:04}")))
        .collect();
    classes.retain(|c| stable_ids.contains_key(c.id.as_str()));
    classes.sort_by(|a, b| (&a.file, &a.name, a.line).cmp(&(&b.file, &b.name, b.line)));
    let retained_classes = classes.clone();
    for class in &mut classes {
        class
            .members
            .sort_by(|a, b| (&a.file, a.line, &a.name).cmp(&(&b.file, b.line, &b.name)));
        // C/C++ prototypes and definitions are one displayed method. Retain every location in
        // all_classes below so calls to either declaration still resolve to the same box.
        if language_family(&class.file) == "C/C++" {
            let mut seen = BTreeSet::new();
            class.members.retain(|m| {
                seen.insert((
                    m.kind.clone(),
                    m.name.clone(),
                    m.value_type.clone(),
                    m.parameters
                        .iter()
                        .map(|p| p.value_type.clone())
                        .collect::<Vec<_>>(),
                ))
            });
        }
    }
    let ids: Vec<_> = classes
        .iter()
        .map(|c| stable_ids[c.id.as_str()].clone())
        .collect();
    let visible: BTreeMap<_, _> = classes
        .iter()
        .enumerate()
        .map(|(i, c)| (c.id.as_str(), i))
        .collect();
    let hidden_ids: BTreeSet<_> = stable_ids
        .keys()
        .copied()
        .filter(|id| !visible.contains_key(id))
        .collect();
    let mut edges = type_edges(&classes, &all_classes, &hidden_ids, &mut warnings);
    let filtered_edges = if options.tests == TestMode::Include {
        0
    } else {
        let full_scope: Vec<_> = scoped.iter().map(|c| (*c).clone()).collect();
        let full_edges = type_edges(
            &full_scope,
            &all_classes,
            &BTreeSet::new(),
            &mut BTreeSet::new(),
        );
        let identify = |edges: &BTreeSet<DiagramEdge>, classes: &[Class]| -> BTreeSet<_> {
            edges
                .iter()
                .map(|(i, j, arrow, label)| {
                    (
                        classes[*i].id.clone(),
                        classes[*j].id.clone(),
                        arrow.clone(),
                        label.clone(),
                    )
                })
                .collect()
        };
        identify(&full_edges, &full_scope)
            .difference(&identify(&edges, &classes))
            .count()
    };
    let symbols: BTreeMap<_, _> = result.symbols.iter().map(|s| (s.id.as_str(), s)).collect();
    let mut members_by_location: BTreeMap<_, Vec<_>> = BTreeMap::new();
    for class in &all_classes {
        for member in &class.members {
            members_by_location
                .entry((member.file.as_str(), member.line, member.name.as_str()))
                .or_default()
                .push((class.id.as_str(), member));
        }
    }
    let retained_locations: BTreeSet<_> = retained_classes
        .iter()
        .flat_map(|c| {
            c.members
                .iter()
                .map(|m| (c.id.as_str(), m.file.as_str(), m.line, m.name.as_str()))
        })
        .collect();
    let locate = |id: &str| {
        let symbol = symbols.get(id)?;
        let matches =
            members_by_location.get(&(symbol.file.as_str(), symbol.line, symbol.name.as_str()))?;
        (matches.len() == 1).then(|| matches[0])
    };
    let mut omitted_calls = 0;
    let mut filtered_calls = 0;
    for call in &result.calls {
        if let (Some((from, caller)), Some((to, callee))) = (locate(&call.from), locate(&call.to)) {
            if stable_ids.contains_key(from) && stable_ids.contains_key(to) {
                let kept = |owner, member: &Member| {
                    retained_locations.contains(&(
                        owner,
                        member.file.as_str(),
                        member.line,
                        member.name.as_str(),
                    ))
                };
                if kept(from, caller) && kept(to, callee) {
                    let label = format!("{}() calls {}()", caller.name, callee.name);
                    edges.insert((visible[from], visible[to], "..>".into(), label));
                } else {
                    filtered_calls += 1;
                }
                continue;
            }
        }
        omitted_calls += 1;
    }
    let mut edge_groups: BTreeMap<_, Vec<&str>> = BTreeMap::new();
    for (i, j, arrow, label) in &edges {
        edge_groups
            .entry((*i, *j, arrow.as_str()))
            .or_default()
            .push(label);
    }
    let mut out = String::from("# Mermaid class diagrams\n\n");
    out.push_str(
        "Declared types and signatures; unknown types are `unknown`. Receivers are omitted.\n",
    );
    out.push_str(
        "Fields are associations, not lifetime ownership. Calls are static heuristic estimates.\n",
    );
    out.push_str(
        "Views contain at most 40 members per box. Connections within each view are uncapped.\n",
    );
    out.push_str("Repeated boxes continue their members; parallel arrows are summarized.\n");
    out.push_str("Cross-diagram relationships are retained in the complete relationship list.\n\n");
    out.push_str(&format!(
        "Included: {} boxes. Calls without in-scope member endpoints: {omitted_calls}.\n\n",
        classes.len()
    ));
    out.push_str(&filter.summary(filtered_calls, filtered_edges));
    if classes.is_empty() && !scoped.is_empty() && options.tests == TestMode::Exclude {
        out.push_str("No declarations remain after test filtering.\n\n");
    }
    let mut aliases = BTreeMap::new();
    let mut signatures = BTreeMap::new();
    for (page_index, page) in pages(&classes, options.max_classes).iter().enumerate() {
        out.push_str(&format!(
            "## Diagram {}\n\n```mermaid\nclassDiagram\n    direction TB\n",
            page_index + 1
        ));
        for (index, members) in page {
            let class = &classes[*index];
            let label = safe(&class.name);
            out.push_str(&format!("    class {}[\"{label}\"] {{\n", ids[*index]));
            out.push_str(&format!("        <<{}>>\n", safe(&class.kind)));
            for member in *members {
                let full = member_text(member, &mut aliases);
                let display = if full.chars().count() > 88 {
                    let key = abbreviation(&plain_member(member), "Signature", 0, &mut signatures);
                    let name = safe(&member.name);
                    if member.kind == "method" {
                        format!("{}{name}({key})", visibility(&member.visibility))
                    } else {
                        format!("{}{name}: {key}", visibility(&member.visibility))
                    }
                } else {
                    full
                };
                out.push_str(&format!("        {display}\n"));
            }
            out.push_str("    }\n");
        }
        for ((i, j, arrow), labels) in &edge_groups {
            if page.iter().any(|(index, _)| index == i) && page.iter().any(|(index, _)| index == j)
            {
                let label = if labels.len() == 1 {
                    labels[0].to_string()
                } else {
                    format!("{} relationships (see list)", labels.len())
                };
                let label = safe(&abbreviation(&label, "Relation", 65, &mut aliases));
                out.push_str(&format!("    {} {arrow} {} : {label}\n", ids[*i], ids[*j]));
            }
        }
        out.push_str("```\n\n");
    }
    out.push_str("## Source index\n\n");
    for (i, class) in classes.iter().enumerate() {
        out.push_str(&format!(
            "- {}: {} — {}:{}\n",
            ids[i],
            markdown_code(&class.name),
            markdown_code(&class.file),
            class.line
        ));
    }
    out.push_str("\n## Relationships\n\n");
    for (i, j, arrow, label) in &edges {
        out.push_str(&format!(
            "- {} {arrow} {}: {}\n",
            ids[*i],
            ids[*j],
            markdown_code(label)
        ));
    }
    if !aliases.is_empty() {
        out.push_str("\n## Type key\n\n");
        for (original, alias) in ordered_key(aliases) {
            out.push_str(&format!("- {alias}: {}\n", markdown_code(&original)));
        }
    }
    if !signatures.is_empty() {
        out.push_str("\n## Signature key\n\n");
        for (original, alias) in ordered_key(signatures) {
            out.push_str(&format!("- {alias}: {}\n", markdown_code(&original)));
        }
    }
    if !warnings.is_empty() {
        out.push_str("\n## Extraction warnings\n\n");
        for warning in warnings {
            out.push_str(&format!("- {}\n", markdown_code(&warning)));
        }
    }
    Ok(MermaidExport {
        markdown: wrap_prose(&out),
        notices: filter.notices.into_iter().collect(),
    })
}

type DiagramEdge = (usize, usize, String, String);

fn type_edges(
    classes: &[Class],
    all_classes: &[Class],
    hidden_ids: &BTreeSet<&str>,
    warnings: &mut BTreeSet<String>,
) -> BTreeSet<DiagramEdge> {
    let visible: BTreeMap<_, _> = classes
        .iter()
        .enumerate()
        .map(|(i, c)| (c.id.as_str(), i))
        .collect();
    let all_names = name_index(all_classes);
    let target = |file: &str, name: &str| {
        resolve(all_classes, &all_names, file, name)
            .and_then(|i| visible.get(all_classes[i].id.as_str()).copied())
    };
    let mut edges = BTreeSet::new();
    for (i, class) in classes.iter().enumerate() {
        for base in &class.bases {
            if let Some(j) = target(&class.file, &base.name) {
                let arrow = if base.relation == "implements"
                    || (class.kind != "interface" && classes[j].kind == "interface")
                {
                    "..|>"
                } else {
                    "--|>"
                };
                edges.insert((
                    i,
                    j,
                    arrow.to_string(),
                    if arrow == "..|>" {
                        "implements"
                    } else {
                        "inherits"
                    }
                    .into(),
                ));
            } else if !resolve(all_classes, &all_names, &class.file, &base.name)
                .is_some_and(|i| hidden_ids.contains(all_classes[i].id.as_str()))
            {
                warnings.insert(format!(
                    "Unresolved or out-of-scope base: {} -> {}",
                    class.name, base.name
                ));
            }
        }
        for member in &class.members {
            let types = member.value_type.iter().chain(
                member
                    .parameters
                    .iter()
                    .filter_map(|p| p.value_type.as_ref()),
            );
            for value_type in types {
                if class.kind == "type_alias" && member.name == "alias" {
                    continue;
                }
                for token in type_names(value_type, language_family(&class.file)) {
                    if let Some(j) = target(&class.file, &token) {
                        if i != j {
                            let (arrow, label) = if member.kind == "field" {
                                ("-->", format!("field {}", member.name))
                            } else {
                                ("..>", format!("type in {}", member.name))
                            };
                            edges.insert((i, j, arrow.into(), label));
                        }
                    } else if all_names
                        .get(&token)
                        .is_some_and(|matches| matches.len() > 1)
                    {
                        warnings.insert(format!("Ambiguous type: {} uses {token}", class.name));
                    }
                }
            }
        }
    }
    edges
}

// Legacy declaration IDs can collide for same-named types on one source line. Assign private
// occurrence IDs before selection, using complete records to break ties deterministically.
fn unique_occurrences(classes: &[Class]) -> Vec<Class> {
    let mut counts = BTreeMap::new();
    let mut used = BTreeSet::new();
    for class in classes {
        *counts.entry(class.id.as_str()).or_insert(0) += 1;
        used.insert(class.id.clone());
    }
    let mut raw = classes.to_vec();
    for class in &mut raw {
        if counts[class.id.as_str()] > 1 {
            class.members.sort();
            class.bases.sort();
        }
    }
    raw.sort();
    let mut next = 0;
    for class in &mut raw {
        if counts[class.id.as_str()] > 1 {
            loop {
                let candidate = format!("{}#occurrence-{next:020}", class.id);
                next += 1;
                if used.insert(candidate.clone()) {
                    class.id = candidate;
                    break;
                }
            }
        }
    }
    raw
}

// Resolve every impl against the full identity set, even when its owner is hidden.
fn merge_classes(
    raw: &[Class],
    identities: &[Class],
    warnings: &mut BTreeSet<String>,
) -> Vec<Class> {
    let mut owners: Vec<_> = identities
        .iter()
        .filter(|c| c.kind != "impl")
        .cloned()
        .collect();
    owners.sort_by(|a, b| a.id.cmp(&b.id));
    let names = name_index(&owners);
    let mut classes: Vec<_> = raw.iter().filter(|c| c.kind != "impl").cloned().collect();
    classes.sort_by(|a, b| a.id.cmp(&b.id));
    let mut implementations: Vec<_> = raw.iter().filter(|c| c.kind == "impl").cloned().collect();
    implementations.sort_by(|a, b| a.id.cmp(&b.id));
    for implementation in implementations {
        if let Some(index) = resolve(&owners, &names, &implementation.file, &implementation.name) {
            if !classes.iter().any(|c| c.id == owners[index].id) {
                // A kept source part still needs its owner, but must not restore excluded fields.
                let mut context = owners[index].clone();
                context.members.clear();
                context.bases.clear();
                warnings.insert(format!(
                    "Owner retained as context for kept implementation: {} in {}",
                    context.name, implementation.file
                ));
                classes.push(context);
            }
            if let Some(owner) = classes.iter_mut().find(|c| c.id == owners[index].id) {
                owner.members.extend(implementation.members);
                owner.bases.extend(implementation.bases);
            }
        } else {
            warnings.insert(format!(
                "Unresolved implementation: {} in {}",
                implementation.name, implementation.file
            ));
            classes.push(Class {
                kind: "unresolved_impl".into(),
                ..implementation
            });
        }
    }
    classes
}

fn name_index(classes: &[Class]) -> BTreeMap<String, Vec<usize>> {
    let mut index: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for (i, class) in classes
        .iter()
        .enumerate()
        .filter(|(_, c)| c.kind != "module")
    {
        index.entry(class.name.clone()).or_default().push(i);
    }
    index
}

fn resolve(
    classes: &[Class],
    index: &BTreeMap<String, Vec<usize>>,
    file: &str,
    name: &str,
) -> Option<usize> {
    let name = name.split(['<', '[', '(']).next()?.trim();
    let candidates: Vec<_> = index
        .get(name)?
        .iter()
        .copied()
        .filter(|i| language_family(&classes[*i].file) == language_family(file))
        .collect();
    for subset in [
        candidates
            .iter()
            .copied()
            .filter(|i| classes[*i].file == file)
            .collect::<Vec<_>>(),
        candidates
            .iter()
            .copied()
            .filter(|i| {
                std::path::Path::new(&classes[*i].file).parent()
                    == std::path::Path::new(file).parent()
            })
            .collect(),
        candidates,
    ] {
        if subset.len() == 1 {
            return Some(subset[0]);
        }
        if subset.len() > 1 {
            return None;
        }
    }
    None
}

fn member_text(member: &Member, aliases: &mut BTreeMap<String, String>) -> String {
    let visibility = visibility(&member.visibility);
    let kind = type_text(member.value_type.as_deref().unwrap_or("unknown"), aliases);
    if member.kind == "method" {
        let args = member
            .parameters
            .iter()
            .map(|p| {
                format!(
                    "{}: {}",
                    safe(&p.name),
                    type_text(p.value_type.as_deref().unwrap_or("unknown"), aliases)
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!("{}{}({args}) {kind}", visibility, safe(&member.name))
    } else {
        format!("{}{}: {kind}", visibility, safe(&member.name))
    }
}

fn type_text(value: &str, aliases: &mut BTreeMap<String, String>) -> String {
    // Mermaid supports nested single-argument generics, but not comma-separated ones.
    if value.len() <= 40
        && value
            .chars()
            .all(|c| c.is_alphanumeric() || "_ .:[]<>".contains(c))
    {
        return value.replace(['<', '>'], "~");
    }
    let next = format!("Type{}", aliases.len() + 1);
    aliases.entry(value.into()).or_insert(next).clone()
}

// Source text cannot introduce Mermaid statements, Markdown fences, or HTML.
fn safe(value: &str) -> String {
    value
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || "_ .:/()-".contains(c) {
                c.to_string()
            } else {
                format!("#{};", c as u32)
            }
        })
        .collect()
}

fn pages(classes: &[Class], maximum: usize) -> Vec<Vec<(usize, &[Member])>> {
    let mut pages = Vec::new();
    let mut current = Vec::new();
    for (index, class) in classes.iter().enumerate() {
        if class.members.len() > 40 {
            if !current.is_empty() {
                pages.push(std::mem::take(&mut current));
            }
            for part in class.members.chunks(40) {
                pages.push(vec![(index, part)]);
            }
        } else {
            current.push((index, class.members.as_slice()));
            if current.len() == maximum {
                pages.push(std::mem::take(&mut current));
            }
        }
    }
    if !current.is_empty() {
        pages.push(current);
    }
    pages
}

fn abbreviation(
    value: &str,
    prefix: &str,
    limit: usize,
    key: &mut BTreeMap<String, String>,
) -> String {
    if value.chars().count() <= limit {
        return value.into();
    }
    let next = format!("{prefix}{}", key.len() + 1);
    key.entry(value.into()).or_insert(next).clone()
}

fn wrap_prose(markdown: &str) -> String {
    let mut result = String::new();
    let mut in_diagram = false;
    for line in markdown.lines() {
        if line.starts_with("```") {
            in_diagram = !in_diagram;
        }
        if in_diagram || line.chars().count() <= 100 {
            result.push_str(line);
            result.push('\n');
            continue;
        }
        let mut length = 0;
        for word in line.split_whitespace() {
            let size = word.chars().count();
            if length > 0 && length + size + 1 > 100 {
                result.push_str("\n  ");
                length = 2;
            } else if length > 0 {
                result.push(' ');
                length += 1;
            }
            // Hard wrapping only applies to a single unusually long source token in the key.
            for ch in word.chars() {
                if length == 100 {
                    result.push_str("\n  ");
                    length = 2;
                }
                result.push(ch);
                length += 1;
            }
        }
        result.push('\n');
    }
    result
}

fn visibility(value: &str) -> &str {
    match value {
        "+" | "-" | "#" | "~" => value,
        _ => "",
    }
}

fn type_names(value: &str, language: &str) -> Vec<String> {
    // Quoted Python annotations are forward references, except Literal[...] values.
    // Rust apostrophes introduce lifetimes, not string literals.
    static LIFETIMES: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(r"'[A-Za-z_][A-Za-z0-9_]*")
            .expect("lifetime token pattern is a constant valid regex")
    });
    let value = if language == "Rust" {
        LIFETIMES.replace_all(value, " ")
    } else {
        std::borrow::Cow::Borrowed(value)
    };
    static NAMES: std::sync::LazyLock<regex::Regex> = std::sync::LazyLock::new(|| {
        regex::Regex::new(concat!(
            r#""(?:\\.|[^"\\])*"|'(?:\\.|[^'\\])*'|[\[\],]|"#,
            r"([\p{L}_][\p{L}\p{N}_]*(?:(?:::|\.)[\p{L}_][\p{L}\p{N}_]*)*)",
        ))
        .expect("type reference token pattern is a constant valid regex")
    });
    let mut names = Vec::new();
    // Each bracket scope records whether it contains values and whether it is Annotated.
    let mut scopes: Vec<(bool, bool)> = Vec::new();
    let mut previous_literal = false;
    let mut previous_annotated = false;
    for capture in NAMES.captures_iter(&value) {
        let token = capture
            .get(0)
            .expect("regex match has a full capture")
            .as_str();
        let in_literal = scopes.last().is_some_and(|(values, _)| *values);
        match token {
            "[" => scopes.push((in_literal || previous_literal, previous_annotated)),
            "]" => {
                scopes.pop();
            }
            "," => {
                if let Some((values, true)) = scopes.last_mut() {
                    *values = true;
                }
            }
            _ if capture.get(1).is_some() && !in_literal => names.push(token.to_string()),
            _ if language == "Python" && !in_literal && token.starts_with(['\'', '"']) => {
                names.extend(type_names(&token[1..token.len() - 1], language));
            }
            _ => (),
        }
        previous_literal = matches!(
            token,
            "Literal" | "typing.Literal" | "typing_extensions.Literal"
        );
        previous_annotated = language == "Python"
            && matches!(
                token,
                "Annotated" | "typing.Annotated" | "typing_extensions.Annotated"
            );
    }
    names
}

fn language_family(file: &str) -> &str {
    static REGISTRY: std::sync::LazyLock<crate::languages::AnalyserRegistry> =
        std::sync::LazyLock::new(crate::languages::AnalyserRegistry::new);
    let extension = std::path::Path::new(file)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    match REGISTRY.language_for_extension(extension) {
        Some("C" | "C++") => "C/C++",
        Some(language) => language,
        None => extension,
    }
}

fn ordered_key(key: BTreeMap<String, String>) -> Vec<(String, String)> {
    let mut entries: Vec<_> = key.into_iter().collect();
    entries.sort_by_key(|(_, alias)| {
        alias
            .trim_start_matches(char::is_alphabetic)
            .parse::<usize>()
            .unwrap_or(0)
    });
    entries
}

fn markdown_code(value: &str) -> String {
    let fence = "`".repeat(value.split(|c| c != '`').map(str::len).max().unwrap_or(0) + 1);
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    format!("{fence}{value}{fence}")
}

fn plain_member(member: &Member) -> String {
    let kind = member.value_type.as_deref().unwrap_or("unknown");
    if member.kind == "method" {
        let args = member
            .parameters
            .iter()
            .map(|p| {
                format!(
                    "{}: {}",
                    p.name,
                    p.value_type.as_deref().unwrap_or("unknown")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        format!(
            "{}{}({args}) {kind}",
            visibility(&member.visibility),
            member.name
        )
    } else {
        format!("{}{}: {kind}", visibility(&member.visibility), member.name)
    }
}
