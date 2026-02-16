//! Phase 3: Multi-language import resolution.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::config::{AnalysisConfig, ImportEdge, PackageReference, ProjectReference};
use crate::dotnet::assembly::AssemblyIndex;
use crate::dotnet::project::parse_project_file;
use crate::dotnet::solution::parse_solution;
use crate::graph::knowledge_graph::{KnowledgeGraph, NodeData};
use crate::graph::namespace_index::NamespaceIndex;
use crate::graph::symbol_table::SymbolTable;
use crate::languages::AnalyserRegistry;

/// Run the imports phase: resolve import statements to file edges.
pub fn run_imports_phase(
    config: &AnalysisConfig,
    kg: &mut KnowledgeGraph,
    st: &mut SymbolTable,
    ns_index: &mut NamespaceIndex,
) {
    let mut assembly_index = AssemblyIndex::new();

    // Process .NET project files (sln, csproj, vbproj)
    process_dotnet_projects(config, kg, &mut assembly_index);

    // Supplement assembly index with observed namespace declarations
    register_observed_namespaces(kg, &assembly_index);

    // Process source file imports
    process_source_imports(config, kg, st, &assembly_index, ns_index);
}

// ---------------------------------------------------------------------------
// .NET project processing
// ---------------------------------------------------------------------------

fn process_dotnet_projects(
    config: &AnalysisConfig,
    kg: &mut KnowledgeGraph,
    assembly_index: &mut AssemblyIndex,
) {
    let repo_root = &config.repo_path;

    // Collect file paths
    let files: Vec<(String, Option<String>)> = kg
        .get_files()
        .into_iter()
        .filter_map(|n| {
            if let NodeData::File { path, language, .. } = n {
                Some((path.clone(), language.clone()))
            } else {
                None
            }
        })
        .collect();

    let mut sln_files = Vec::new();
    let mut project_files = Vec::new();
    for (path, _) in &files {
        if path.ends_with(".sln") {
            sln_files.push(path.clone());
        } else if path.ends_with(".csproj") || path.ends_with(".vbproj") {
            project_files.push(path.clone());
        }
    }

    // Parse solutions (for discovery, not currently used beyond logging)
    for sln_path in &sln_files {
        let full_path = Path::new(repo_root).join(sln_path);
        if let Ok(content) = std::fs::read_to_string(&full_path) {
            let _projects = parse_solution(&content);
        }
    }

    // Parse each project file
    for proj_path in &project_files {
        let full_path = Path::new(repo_root).join(proj_path);
        let content = match std::fs::read_to_string(&full_path) {
            Ok(c) => c,
            Err(_) => continue,
        };

        let info = parse_project_file(&content, proj_path);

        // Register root namespace
        if let Some(ref root_ns) = info.root_namespace {
            assembly_index.register(root_ns, proj_path);
        }

        // Add project references
        for ref_path in &info.project_references {
            let proj_dir = Path::new(proj_path).parent().unwrap_or(Path::new(""));
            let full_ref = proj_dir.join(ref_path);
            let resolved = normalize_path(&full_ref.to_string_lossy());
            kg.add_project_reference(&ProjectReference {
                from_project: proj_path.clone(),
                to_project: resolved,
                ref_type: "ProjectReference".to_string(),
            });
        }

        // Add package references
        for (pkg_name, pkg_version) in &info.package_references {
            kg.add_package_reference(&PackageReference {
                project: proj_path.clone(),
                package: pkg_name.clone(),
                version: pkg_version.clone(),
            });
        }
    }
}

fn register_observed_namespaces(kg: &KnowledgeGraph, _assembly_index: &AssemblyIndex) {
    // Supplement assembly mapper with namespace declarations found during parsing
    let _symbols = kg.get_symbols();
    // For each namespace symbol, find which project file it belongs to and register
    // This is a minor supplementary step — the primary registration happens via csproj parsing
}

// ---------------------------------------------------------------------------
// Source file import resolution
// ---------------------------------------------------------------------------

fn process_source_imports(
    config: &AnalysisConfig,
    kg: &mut KnowledgeGraph,
    st: &mut SymbolTable,
    assembly_index: &AssemblyIndex,
    ns_index: &mut NamespaceIndex,
) {
    let repo_root = &config.repo_path;
    let registry = AnalyserRegistry::new();

    // Build file set once for O(1) lookups
    let file_set: HashSet<String> = kg
        .get_files()
        .into_iter()
        .filter_map(|n| {
            if let NodeData::File { path, .. } = n {
                Some(path.clone())
            } else {
                None
            }
        })
        .collect();

    // Collect file info for iteration
    let files: Vec<(String, Option<String>)> = kg
        .get_files()
        .into_iter()
        .filter_map(|n| {
            if let NodeData::File { path, language, .. } = n {
                Some((path.clone(), language.clone()))
            } else {
                None
            }
        })
        .collect();

    // --- Pre-processing: build language-specific indexes ---

    // Go: parse go.mod and build directory index
    let go_module = parse_go_mod(&file_set, repo_root);
    let go_dir_index = if go_module.is_some() {
        build_go_dir_index(&file_set)
    } else {
        HashMap::new()
    };

    // Java: build basename index for class-name fallback resolution
    let mut java_basename_index: HashMap<String, Vec<String>> = HashMap::new();
    for path in &file_set {
        if path.ends_with(".java") {
            let basename = Path::new(path)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_default();
            java_basename_index
                .entry(basename)
                .or_default()
                .push(path.clone());
        }
    }

    // Process each file's imports
    for (file_path, language) in &files {
        let lang = match language {
            Some(l) => l.as_str(),
            None => continue,
        };

        let ext = Path::new(file_path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default();

        let analyser = match registry.get_by_extension(&ext) {
            Some(a) => a,
            None => continue,
        };

        if !analyser.is_available() {
            continue;
        }

        let abs_path = Path::new(repo_root).join(file_path);
        let source = match std::fs::read(&abs_path) {
            Ok(s) => s,
            Err(_) => continue,
        };

        // Parse with tree-sitter
        let mut parser = tree_sitter::Parser::new();
        let ts_language = analyser.get_language_for_ext(&ext);
        if parser.set_language(&ts_language).is_err() {
            continue;
        }
        let tree = match parser.parse(&source, None) {
            Some(t) => t,
            None => continue,
        };

        // Extract imports
        let imports = analyser.extract_imports(&tree, &source, file_path);

        // Resolve each import based on language
        for imp in &imports {
            // C#/VB.NET: namespace index
            if lang == "C#" || lang == "VB.NET" {
                let ns_files = ns_index.get_files_for_namespace(&imp.target_name).to_vec();
                if !ns_files.is_empty() {
                    ns_index.register_file_import(file_path, &imp.target_name);
                    for target in &ns_files {
                        if target != file_path {
                            kg.add_import(&ImportEdge {
                                from_file: file_path.to_string(),
                                to_file: target.to_string(),
                                statement: imp.statement.clone(),
                            });
                        }
                    }
                    continue;
                }
                // Fall through to fallback resolver
                if let Some(target) =
                    resolve_fallback(&imp.target_name, file_path, st, assembly_index, kg)
                {
                    if target != *file_path {
                        kg.add_import(&ImportEdge {
                            from_file: file_path.clone(),
                            to_file: target,
                            statement: imp.statement.clone(),
                        });
                        ns_index.register_file_import(file_path, &imp.target_name);
                    }
                }
                continue;
            }

            // Python: dotted module paths
            if lang == "Python" {
                if let Some(target) = resolve_python_import(&imp.target_name, file_path, &file_set)
                {
                    if target != *file_path {
                        kg.add_import(&ImportEdge {
                            from_file: file_path.clone(),
                            to_file: target,
                            statement: imp.statement.clone(),
                        });
                    }
                }
                continue;
            }

            // TypeScript/JavaScript: relative paths + extension probing
            if lang == "TypeScript" || lang == "JavaScript" {
                if let Some(target) = resolve_ts_import(&imp.target_name, file_path, &file_set) {
                    if target != *file_path {
                        kg.add_import(&ImportEdge {
                            from_file: file_path.clone(),
                            to_file: target,
                            statement: imp.statement.clone(),
                        });
                    }
                }
                continue;
            }

            // Java: dotted path + class-name fallback
            if lang == "Java" {
                if let Some(target) = resolve_java_import(
                    &imp.target_name,
                    file_path,
                    &file_set,
                    &java_basename_index,
                ) {
                    if target != *file_path {
                        kg.add_import(&ImportEdge {
                            from_file: file_path.clone(),
                            to_file: target,
                            statement: imp.statement.clone(),
                        });
                    }
                }
                continue;
            }

            // Go: package-level directory resolution
            if lang == "Go" {
                let targets =
                    resolve_go_import(&imp.target_name, go_module.as_deref(), &go_dir_index);
                for target in &targets {
                    if target != file_path {
                        kg.add_import(&ImportEdge {
                            from_file: file_path.clone(),
                            to_file: target.clone(),
                            statement: imp.statement.clone(),
                        });
                    }
                }
                continue;
            }

            // Rust: crate/super/self prefix + progressive shortening
            if lang == "Rust" {
                if let Some(target) = resolve_rust_import(&imp.target_name, file_path, &file_set) {
                    if target != *file_path {
                        kg.add_import(&ImportEdge {
                            from_file: file_path.clone(),
                            to_file: target,
                            statement: imp.statement.clone(),
                        });
                    }
                }
                continue;
            }

            // C/C++: relative include resolution
            if lang == "C" || lang == "C++" {
                if let Some(target) =
                    resolve_c_include(&imp.target_name, &imp.statement, file_path, &file_set)
                {
                    if target != *file_path {
                        kg.add_import(&ImportEdge {
                            from_file: file_path.clone(),
                            to_file: target,
                            statement: imp.statement.clone(),
                        });
                    }
                }
                continue;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Python resolver
// ---------------------------------------------------------------------------

fn resolve_python_import(
    target_name: &str,
    source_file: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
    if target_name.starts_with('.') {
        return resolve_python_relative(target_name, source_file, file_set);
    }

    let path = target_name.replace('.', "/");

    let candidate = format!("{}.py", path);
    if file_set.contains(&candidate) {
        return Some(candidate);
    }

    let candidate = format!("{}/__init__.py", path);
    if file_set.contains(&candidate) {
        return Some(candidate);
    }

    None
}

fn resolve_python_relative(
    target_name: &str,
    source_file: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
    let dots = target_name.chars().take_while(|&c| c == '.').count();
    let remainder = &target_name[dots..];

    let mut base = Path::new(source_file)
        .parent()
        .unwrap_or(Path::new(""))
        .to_string_lossy()
        .to_string();

    for _ in 0..dots.saturating_sub(1) {
        base = Path::new(&base)
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .to_string();
    }

    let path = if remainder.is_empty() {
        if base.is_empty() {
            return None;
        }
        let candidate = format!("{}/__init__.py", base);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
        return None;
    } else {
        let rel = remainder.replace('.', "/");
        if base.is_empty() {
            rel
        } else {
            format!("{}/{}", base, rel)
        }
    };

    let candidate = format!("{}.py", path);
    if file_set.contains(&candidate) {
        return Some(candidate);
    }

    let candidate = format!("{}/__init__.py", path);
    if file_set.contains(&candidate) {
        return Some(candidate);
    }

    None
}

// ---------------------------------------------------------------------------
// TypeScript/JavaScript resolver
// ---------------------------------------------------------------------------

fn resolve_ts_import(
    target_name: &str,
    source_file: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
    // Bare specifiers (no ./ or ../ prefix) are external packages
    if !target_name.starts_with("./") && !target_name.starts_with("../") {
        return None;
    }

    let source_dir = Path::new(source_file)
        .parent()
        .unwrap_or(Path::new(""))
        .to_string_lossy()
        .to_string();

    let resolved = normalize_path(&format!("{}/{}", source_dir, target_name));

    // Direct match
    if file_set.contains(&resolved) {
        return Some(resolved);
    }

    // Extension probing
    for ext in &[".ts", ".tsx", ".js", ".jsx"] {
        let candidate = format!("{}{}", resolved, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }

    // Index file probing
    for ext in &[".ts", ".tsx", ".js", ".jsx"] {
        let candidate = format!("{}/index{}", resolved, ext);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Java resolver
// ---------------------------------------------------------------------------

fn resolve_java_import(
    target_name: &str,
    source_file: &str,
    file_set: &HashSet<String>,
    basename_index: &HashMap<String, Vec<String>>,
) -> Option<String> {
    // Primary: path-based resolution
    let path = format!("{}.java", target_name.replace('.', "/"));
    if file_set.contains(&path) {
        return Some(path);
    }

    // Fallback: class-name basename lookup
    let class_name = target_name.rsplit('.').next().unwrap_or(target_name);
    let basename = format!("{}.java", class_name);
    if let Some(candidates) = basename_index.get(&basename) {
        for candidate in candidates {
            if candidate != source_file {
                return Some(candidate.clone());
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Go resolver
// ---------------------------------------------------------------------------

fn parse_go_mod(file_set: &HashSet<String>, repo_root: &str) -> Option<String> {
    for path in file_set {
        if Path::new(path)
            .file_name()
            .map(|f| f == "go.mod")
            .unwrap_or(false)
        {
            let full = Path::new(repo_root).join(path);
            if let Ok(content) = std::fs::read_to_string(&full) {
                for line in content.lines() {
                    let line = line.trim();
                    if let Some(module) = line.strip_prefix("module ") {
                        return Some(module.trim().to_string());
                    }
                }
            }
        }
    }
    None
}

fn build_go_dir_index(file_set: &HashSet<String>) -> HashMap<String, Vec<String>> {
    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    for path in file_set {
        if path.ends_with(".go") {
            let dir = Path::new(path)
                .parent()
                .unwrap_or(Path::new(""))
                .to_string_lossy()
                .to_string();
            index.entry(dir).or_default().push(path.clone());
        }
    }
    index
}

fn resolve_go_import(
    target_name: &str,
    go_module: Option<&str>,
    go_dir_index: &HashMap<String, Vec<String>>,
) -> Vec<String> {
    let go_module = match go_module {
        Some(m) => m,
        None => return Vec::new(),
    };

    // Stdlib imports have no slash
    if !target_name.contains('/') {
        return Vec::new();
    }

    // Must be part of this module
    if !target_name.starts_with(go_module) {
        return Vec::new();
    }

    // Strip module prefix to get relative directory
    let rel_dir = &target_name[go_module.len()..];
    let rel_dir = rel_dir.strip_prefix('/').unwrap_or(rel_dir);

    go_dir_index.get(rel_dir).cloned().unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Rust resolver
// ---------------------------------------------------------------------------

const RUST_EXTERNAL_PREFIXES: &[&str] = &["std::", "core::", "alloc::"];

fn resolve_rust_import(
    target_name: &str,
    source_file: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
    // External crates
    for prefix in RUST_EXTERNAL_PREFIXES {
        if target_name.starts_with(prefix) {
            return None;
        }
    }

    let source_dir = Path::new(source_file)
        .parent()
        .unwrap_or(Path::new(""))
        .to_string_lossy()
        .to_string();

    let (base, remainder) = if let Some(stripped) = target_name.strip_prefix("crate::") {
        (String::new(), stripped)
    } else if target_name.starts_with("super::") {
        // Count consecutive super:: prefixes
        let mut rem = target_name;
        let mut b = source_dir.clone();
        while let Some(stripped) = rem.strip_prefix("super::") {
            rem = stripped;
            b = Path::new(&b)
                .parent()
                .unwrap_or(Path::new(""))
                .to_string_lossy()
                .to_string();
        }
        (b, rem)
    } else if let Some(stripped) = target_name.strip_prefix("self::") {
        (source_dir.clone(), stripped)
    } else {
        // Bare path
        (source_dir.clone(), target_name)
    };

    // Split on :: and try progressive shortening
    let segments: Vec<&str> = remainder.split("::").collect();

    for end in (1..=segments.len()).rev() {
        let path_segments = &segments[..end];
        let rel_path = path_segments.join("/");
        let full_rel = if base.is_empty() {
            rel_path
        } else {
            format!("{}/{}", base, rel_path)
        };

        // Try as {path}.rs
        let candidate = format!("{}.rs", full_rel);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }

        // Try as {path}/mod.rs
        let candidate = format!("{}/mod.rs", full_rel);
        if file_set.contains(&candidate) {
            return Some(candidate);
        }
    }

    None
}

// ---------------------------------------------------------------------------
// C/C++ resolver
// ---------------------------------------------------------------------------

fn resolve_c_include(
    target_name: &str,
    statement: &str,
    source_file: &str,
    file_set: &HashSet<String>,
) -> Option<String> {
    // System includes contain < in the statement text
    if statement.contains('<') {
        return None;
    }

    let source_dir = Path::new(source_file)
        .parent()
        .unwrap_or(Path::new(""))
        .to_string_lossy()
        .to_string();

    // Resolve relative to source file directory
    let candidate = if source_dir.is_empty() {
        target_name.to_string()
    } else {
        normalize_path(&format!("{}/{}", source_dir, target_name))
    };
    if file_set.contains(&candidate) {
        return Some(candidate);
    }

    // Fallback: resolve from repo root
    let candidate = normalize_path(target_name);
    if file_set.contains(&candidate) {
        return Some(candidate);
    }

    None
}

// ---------------------------------------------------------------------------
// Fallback resolver (for C#/VB.NET when namespace index fails)
// ---------------------------------------------------------------------------

fn resolve_fallback(
    target_name: &str,
    _source_file: &str,
    st: &SymbolTable,
    assembly_index: &AssemblyIndex,
    kg: &KnowledgeGraph,
) -> Option<String> {
    // Try fuzzy symbol lookup
    let matches = st.lookup_fuzzy(target_name);
    if !matches.is_empty() {
        return Some(matches[0].file.clone());
    }

    // Try assembly mapper
    if let Some(project) = assembly_index.resolve_namespace(target_name) {
        let proj_dir = Path::new(project)
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .to_string();

        for file_data in kg.get_files() {
            if let NodeData::File { path, .. } = file_data {
                if (path.ends_with(".cs") || path.ends_with(".vb"))
                    && (path.starts_with(&proj_dir) || proj_dir.is_empty())
                {
                    let file_syms = st.get_symbols_in_file(path);
                    if file_syms.is_some_and(|m| !m.is_empty()) {
                        return Some(path.clone());
                    }
                }
            }
        }
    }

    None
}

// ---------------------------------------------------------------------------
// Path normalization (replace backslashes + resolve .. and .)
// ---------------------------------------------------------------------------

fn normalize_path(path: &str) -> String {
    let path = path.replace('\\', "/");
    let mut parts: Vec<&str> = Vec::new();
    for segment in path.split('/') {
        match segment {
            "." | "" => {
                // Skip (unless it's the very first empty for absolute path)
                if parts.is_empty() && segment.is_empty() && path.starts_with('/') {
                    parts.push("");
                }
            }
            ".." => {
                if !parts.is_empty() && parts.last() != Some(&"..") {
                    parts.pop();
                }
            }
            _ => parts.push(segment),
        }
    }
    parts.join("/")
}
