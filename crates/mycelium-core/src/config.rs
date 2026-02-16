//! Core data types and configuration for Mycelium analysis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Type of symbol extracted from source code.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SymbolType {
    Class,
    Function,
    Method,
    Interface,
    Struct,
    Enum,
    Namespace,
    Property,
    Constructor,
    Module,
    Record,
    Delegate,
    TypeAlias,
    Constant,
    Variable,
    Trait,
    Impl,
    Macro,
    Template,
    Typedef,
    Annotation,
    Static,
}

impl SymbolType {
    /// Returns the string representation matching Python's enum value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Class => "Class",
            Self::Function => "Function",
            Self::Method => "Method",
            Self::Interface => "Interface",
            Self::Struct => "Struct",
            Self::Enum => "Enum",
            Self::Namespace => "Namespace",
            Self::Property => "Property",
            Self::Constructor => "Constructor",
            Self::Module => "Module",
            Self::Record => "Record",
            Self::Delegate => "Delegate",
            Self::TypeAlias => "TypeAlias",
            Self::Constant => "Constant",
            Self::Variable => "Variable",
            Self::Trait => "Trait",
            Self::Impl => "Impl",
            Self::Macro => "Macro",
            Self::Template => "Template",
            Self::Typedef => "Typedef",
            Self::Annotation => "Annotation",
            Self::Static => "Static",
        }
    }

    /// Parse from string (matching Python enum values).
    pub fn from_str_value(s: &str) -> Option<Self> {
        match s {
            "Class" => Some(Self::Class),
            "Function" => Some(Self::Function),
            "Method" => Some(Self::Method),
            "Interface" => Some(Self::Interface),
            "Struct" => Some(Self::Struct),
            "Enum" => Some(Self::Enum),
            "Namespace" => Some(Self::Namespace),
            "Property" => Some(Self::Property),
            "Constructor" => Some(Self::Constructor),
            "Module" => Some(Self::Module),
            "Record" => Some(Self::Record),
            "Delegate" => Some(Self::Delegate),
            "TypeAlias" => Some(Self::TypeAlias),
            "Constant" => Some(Self::Constant),
            "Variable" => Some(Self::Variable),
            "Trait" => Some(Self::Trait),
            "Impl" => Some(Self::Impl),
            "Macro" => Some(Self::Macro),
            "Template" => Some(Self::Template),
            "Typedef" => Some(Self::Typedef),
            "Annotation" => Some(Self::Annotation),
            "Static" => Some(Self::Static),
            _ => None,
        }
    }
}

impl std::fmt::Display for SymbolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Visibility level of a symbol.
#[derive(Debug, Clone, Copy, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Private,
    Internal,
    Protected,
    Friend,
    #[default]
    Unknown,
}

impl Visibility {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Private => "private",
            Self::Internal => "internal",
            Self::Protected => "protected",
            Self::Friend => "friend",
            Self::Unknown => "unknown",
        }
    }
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A source file in the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileNode {
    pub path: String,
    pub language: Option<String>,
    pub size: u64,
    pub lines: usize,
}

/// A directory in the repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderNode {
    pub path: String,
    pub file_count: usize,
}

/// A symbol extracted from source code (class, method, function, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Symbol {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub symbol_type: SymbolType,
    pub file: String,
    pub line: usize,
    #[serde(default)]
    pub visibility: Visibility,
    #[serde(default)]
    pub exported: bool,
    pub parent: Option<String>,
    pub language: Option<String>,
    pub byte_range: Option<(usize, usize)>,
    /// Parameter types for DI tracking: Vec<(param_name, type_name)>.
    pub parameter_types: Option<Vec<(String, String)>>,
}

/// Raw import statement extracted from source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatement {
    pub file: String,
    pub statement: String,
    pub target_name: String,
    pub line: usize,
}

/// Raw call site extracted from source.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCall {
    pub caller_file: String,
    pub caller_name: String,
    pub callee_name: String,
    pub line: usize,
    pub qualifier: Option<String>,
}

/// A resolved call edge between two symbols.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEdge {
    pub from_symbol: String,
    pub to_symbol: String,
    pub confidence: f64,
    pub tier: String,
    pub reason: String,
    pub line: usize,
}

/// A resolved import edge between two files.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEdge {
    pub from_file: String,
    pub to_file: String,
    pub statement: String,
}

/// A project-to-project reference (.NET).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectReference {
    pub from_project: String,
    pub to_project: String,
    #[serde(default = "default_project_ref_type")]
    pub ref_type: String,
}

fn default_project_ref_type() -> String {
    "ProjectReference".to_string()
}

/// A project-to-package reference (.NET NuGet).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageReference {
    pub project: String,
    pub package: String,
    #[serde(default)]
    pub version: String,
}

/// A detected community of related symbols.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub cohesion: f64,
    #[serde(default)]
    pub primary_language: String,
}

/// A detected execution flow / process.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub id: String,
    pub entry: String,
    pub terminal: String,
    #[serde(default)]
    pub steps: Vec<String>,
    #[serde(default = "default_process_type")]
    #[serde(rename = "type")]
    pub process_type: String,
    #[serde(default)]
    pub total_confidence: f64,
}

fn default_process_type() -> String {
    "intra_community".to_string()
}

/// Configuration for an analysis run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisConfig {
    #[serde(default)]
    pub repo_path: String,
    pub output_path: Option<String>,
    pub languages: Option<Vec<String>>,
    #[serde(default = "default_resolution")]
    pub resolution: f64,
    #[serde(default = "default_max_processes")]
    pub max_processes: usize,
    #[serde(default = "default_max_depth")]
    pub max_depth: usize,
    #[serde(default = "default_max_branching")]
    pub max_branching: usize,
    #[serde(default = "default_min_steps")]
    pub min_steps: usize,
    #[serde(default)]
    pub exclude_patterns: Vec<String>,
    #[serde(default)]
    pub verbose: bool,
    #[serde(default)]
    pub quiet: bool,
    #[serde(default = "default_max_file_size")]
    pub max_file_size: u64,
    #[serde(default = "default_max_community_size")]
    pub max_community_size: usize,
}

fn default_resolution() -> f64 {
    1.0
}
fn default_max_processes() -> usize {
    75
}
fn default_max_depth() -> usize {
    10
}
fn default_max_branching() -> usize {
    4
}
fn default_min_steps() -> usize {
    2
}
fn default_max_file_size() -> u64 {
    1_000_000
}
fn default_max_community_size() -> usize {
    50
}

impl Default for AnalysisConfig {
    fn default() -> Self {
        Self {
            repo_path: String::new(),
            output_path: None,
            languages: None,
            resolution: default_resolution(),
            max_processes: default_max_processes(),
            max_depth: default_max_depth(),
            max_branching: default_max_branching(),
            min_steps: default_min_steps(),
            exclude_patterns: Vec::new(),
            verbose: false,
            quiet: false,
            max_file_size: default_max_file_size(),
            max_community_size: default_max_community_size(),
        }
    }
}

/// Result of an analysis run — matches the JSON output schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisResult {
    #[serde(default = "default_version")]
    pub version: String,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub stats: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub structure: StructureOutput,
    #[serde(default)]
    pub symbols: Vec<SymbolOutput>,
    #[serde(default)]
    pub imports: ImportsOutput,
    #[serde(default)]
    pub calls: Vec<CallOutput>,
    #[serde(default)]
    pub communities: Vec<CommunityOutput>,
    #[serde(default)]
    pub processes: Vec<ProcessOutput>,
}

fn default_version() -> String {
    "1.0".to_string()
}

impl Default for AnalysisResult {
    fn default() -> Self {
        Self {
            version: default_version(),
            metadata: HashMap::new(),
            stats: HashMap::new(),
            structure: StructureOutput::default(),
            symbols: Vec::new(),
            imports: ImportsOutput::default(),
            calls: Vec::new(),
            communities: Vec::new(),
            processes: Vec::new(),
        }
    }
}

/// Structure section of the output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StructureOutput {
    #[serde(default)]
    pub files: Vec<FileOutput>,
    #[serde(default)]
    pub folders: Vec<FolderOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileOutput {
    pub path: String,
    pub language: Option<String>,
    pub size: u64,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderOutput {
    pub path: String,
    pub file_count: usize,
}

/// Symbol in the output JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolOutput {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub symbol_type: String,
    pub file: String,
    pub line: usize,
    pub visibility: String,
    pub exported: bool,
    pub parent: Option<String>,
    pub language: Option<String>,
}

/// Imports section of the output.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ImportsOutput {
    #[serde(default)]
    pub file_imports: Vec<ImportOutput>,
    #[serde(default)]
    pub project_references: Vec<ProjectRefOutput>,
    #[serde(default)]
    pub package_references: Vec<PackageRefOutput>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOutput {
    pub from: String,
    pub to: String,
    pub statement: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRefOutput {
    pub from: String,
    pub to: String,
    #[serde(rename = "type")]
    pub ref_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageRefOutput {
    pub project: String,
    pub package: String,
    pub version: String,
}

/// Call in the output JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallOutput {
    pub from: String,
    pub to: String,
    pub confidence: f64,
    pub tier: String,
    pub reason: String,
    pub line: usize,
}

/// Community in the output JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityOutput {
    pub id: String,
    pub label: String,
    pub members: Vec<String>,
    pub cohesion: f64,
    pub primary_language: String,
}

/// Process in the output JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessOutput {
    pub id: String,
    pub entry: String,
    pub terminal: String,
    pub steps: Vec<String>,
    #[serde(rename = "type")]
    pub process_type: String,
    pub total_confidence: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symbol_type_roundtrip() {
        for st in [
            SymbolType::Class,
            SymbolType::Function,
            SymbolType::Method,
            SymbolType::Interface,
            SymbolType::Namespace,
            SymbolType::Property,
        ] {
            let s = st.as_str();
            assert_eq!(SymbolType::from_str_value(s), Some(st));
        }
    }

    #[test]
    fn visibility_default_is_unknown() {
        assert_eq!(Visibility::default(), Visibility::Unknown);
    }

    #[test]
    fn analysis_config_defaults() {
        let cfg = AnalysisConfig::default();
        assert_eq!(cfg.resolution, 1.0);
        assert_eq!(cfg.max_processes, 75);
        assert_eq!(cfg.max_depth, 10);
        assert_eq!(cfg.max_community_size, 50);
        assert_eq!(cfg.max_file_size, 1_000_000);
    }

    #[test]
    fn symbol_serialization() {
        let sym = Symbol {
            id: "sym:test".to_string(),
            name: "TestMethod".to_string(),
            symbol_type: SymbolType::Method,
            file: "test.cs".to_string(),
            line: 10,
            visibility: Visibility::Public,
            exported: true,
            parent: Some("TestClass".to_string()),
            language: Some("C#".to_string()),
            byte_range: Some((100, 200)),
            parameter_types: None,
        };
        let json = serde_json::to_string(&sym).unwrap();
        assert!(json.contains("\"type\":\"Method\""));
        assert!(json.contains("\"visibility\":\"public\""));
    }

    #[test]
    fn unknown_symbol_type_from_str() {
        assert_eq!(SymbolType::from_str_value("NotAType"), None);
        assert_eq!(SymbolType::from_str_value(""), None);
        assert_eq!(SymbolType::from_str_value("class"), None); // case-sensitive
    }

    #[test]
    fn symbol_type_display() {
        assert_eq!(format!("{}", SymbolType::Class), "Class");
        assert_eq!(format!("{}", SymbolType::Macro), "Macro");
        assert_eq!(format!("{}", SymbolType::Impl), "Impl");
    }

    #[test]
    fn visibility_display() {
        assert_eq!(format!("{}", Visibility::Public), "public");
        assert_eq!(format!("{}", Visibility::Private), "private");
        assert_eq!(format!("{}", Visibility::Unknown), "unknown");
    }

    #[test]
    fn analysis_result_default() {
        let result = AnalysisResult::default();
        assert_eq!(result.version, "1.0");
        assert!(result.symbols.is_empty());
        assert!(result.calls.is_empty());
        assert!(result.communities.is_empty());
        assert!(result.processes.is_empty());
    }
}
