//! In-memory knowledge graph backed by petgraph::DiGraph.

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::visit::EdgeRef;
use std::collections::HashMap;

use crate::config::{
    CallEdge, Community, FileNode, FolderNode, ImportEdge, PackageReference, Process,
    ProjectReference, Symbol,
};

/// Node data stored in the graph.
#[derive(Debug, Clone)]
pub enum NodeData {
    File {
        path: String,
        language: Option<String>,
        size: u64,
        lines: usize,
    },
    Folder {
        path: String,
        file_count: usize,
    },
    Symbol {
        id: String,
        name: String,
        symbol_type: String,
        file: String,
        line: usize,
        visibility: String,
        exported: bool,
        parent: Option<String>,
        language: Option<String>,
        parameter_types: Option<Vec<(String, String)>>,
    },
    Community {
        id: String,
        label: String,
        cohesion: f64,
        primary_language: String,
    },
    Process {
        id: String,
        entry: String,
        terminal: String,
        process_type: String,
        total_confidence: f64,
    },
    Package {
        name: String,
    },
    Project {
        name: String,
    },
}

impl NodeData {
    pub fn node_type(&self) -> &'static str {
        match self {
            NodeData::File { .. } => "file",
            NodeData::Folder { .. } => "folder",
            NodeData::Symbol { .. } => "symbol",
            NodeData::Community { .. } => "community",
            NodeData::Process { .. } => "process",
            NodeData::Package { .. } => "package",
            NodeData::Project { .. } => "project",
        }
    }
}

/// Edge data stored in the graph.
#[derive(Debug, Clone)]
pub enum EdgeData {
    Defines,
    Imports {
        statement: String,
    },
    Calls {
        confidence: f64,
        tier: String,
        reason: String,
        line: usize,
    },
    ProjectReference {
        ref_type: String,
    },
    PackageReference {
        version: String,
    },
    MemberOf,
    Step {
        order: usize,
    },
    Contains,
}

impl EdgeData {
    pub fn edge_type(&self) -> &'static str {
        match self {
            EdgeData::Defines => "DEFINES",
            EdgeData::Imports { .. } => "IMPORTS",
            EdgeData::Calls { .. } => "CALLS",
            EdgeData::ProjectReference { .. } => "PROJECT_REFERENCE",
            EdgeData::PackageReference { .. } => "PACKAGE_REFERENCE",
            EdgeData::MemberOf => "MEMBER_OF",
            EdgeData::Step { .. } => "STEP",
            EdgeData::Contains => "CONTAINS",
        }
    }
}

/// Wrapper around petgraph::DiGraph with typed node/edge methods.
pub struct KnowledgeGraph {
    graph: DiGraph<NodeData, EdgeData>,
    /// O(1) string ID → NodeIndex lookup.
    id_index: HashMap<String, NodeIndex>,
}

/// A flat dict-like representation of a symbol for queries.
#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub id: String,
    pub name: String,
    pub symbol_type: String,
    pub file: String,
    pub line: usize,
    pub visibility: String,
    pub exported: bool,
    pub parent: Option<String>,
    pub language: Option<String>,
    pub parameter_types: Option<Vec<(String, String)>>,
}

/// A flat representation of a caller/callee query result.
#[derive(Debug, Clone)]
pub struct CallInfo {
    pub id: String,
    pub confidence: f64,
    pub tier: String,
    pub reason: String,
    pub line: usize,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self {
            graph: DiGraph::new(),
            id_index: HashMap::new(),
        }
    }

    /// Get or create a node by string ID.
    fn ensure_node(&mut self, id: &str, data: NodeData) -> NodeIndex {
        if let Some(&idx) = self.id_index.get(id) {
            idx
        } else {
            let idx = self.graph.add_node(data);
            self.id_index.insert(id.to_string(), idx);
            idx
        }
    }

    /// Get node index by ID, or None.
    pub fn get_node_index(&self, id: &str) -> Option<NodeIndex> {
        self.id_index.get(id).copied()
    }

    /// Get node data by ID.
    pub fn get_node_data(&self, id: &str) -> Option<&NodeData> {
        self.id_index
            .get(id)
            .and_then(|&idx| self.graph.node_weight(idx))
    }

    /// Check if a node exists.
    pub fn has_node(&self, id: &str) -> bool {
        self.id_index.contains_key(id)
    }

    // --- Node addition ---

    pub fn add_file(&mut self, node: &FileNode) {
        let id = format!("file:{}", node.path);
        self.ensure_node(
            &id,
            NodeData::File {
                path: node.path.clone(),
                language: node.language.clone(),
                size: node.size,
                lines: node.lines,
            },
        );
    }

    pub fn add_folder(&mut self, node: &FolderNode) {
        let id = format!("folder:{}", node.path);
        self.ensure_node(
            &id,
            NodeData::Folder {
                path: node.path.clone(),
                file_count: node.file_count,
            },
        );
    }

    pub fn add_symbol(&mut self, symbol: &Symbol) {
        let sym_idx = self.ensure_node(
            &symbol.id,
            NodeData::Symbol {
                id: symbol.id.clone(),
                name: symbol.name.clone(),
                symbol_type: symbol.symbol_type.as_str().to_string(),
                file: symbol.file.clone(),
                line: symbol.line,
                visibility: symbol.visibility.as_str().to_string(),
                exported: symbol.exported,
                parent: symbol.parent.clone(),
                language: symbol.language.clone(),
                parameter_types: symbol.parameter_types.clone(),
            },
        );

        // DEFINES edge: file -> symbol
        let file_id = format!("file:{}", symbol.file);
        let file_idx = self.ensure_node(
            &file_id,
            NodeData::File {
                path: symbol.file.clone(),
                language: symbol.language.clone(),
                size: 0,
                lines: 0,
            },
        );
        self.graph.add_edge(file_idx, sym_idx, EdgeData::Defines);
    }

    pub fn add_call(&mut self, edge: &CallEdge) {
        if let (Some(&from_idx), Some(&to_idx)) = (
            self.id_index.get(&edge.from_symbol),
            self.id_index.get(&edge.to_symbol),
        ) {
            self.graph.add_edge(
                from_idx,
                to_idx,
                EdgeData::Calls {
                    confidence: edge.confidence,
                    tier: edge.tier.clone(),
                    reason: edge.reason.clone(),
                    line: edge.line,
                },
            );
        }
    }

    pub fn add_import(&mut self, edge: &ImportEdge) {
        let from_id = format!("file:{}", edge.from_file);
        let to_id = format!("file:{}", edge.to_file);
        let from_idx = self.ensure_node(
            &from_id,
            NodeData::File {
                path: edge.from_file.clone(),
                language: None,
                size: 0,
                lines: 0,
            },
        );
        let to_idx = self.ensure_node(
            &to_id,
            NodeData::File {
                path: edge.to_file.clone(),
                language: None,
                size: 0,
                lines: 0,
            },
        );
        self.graph.add_edge(
            from_idx,
            to_idx,
            EdgeData::Imports {
                statement: edge.statement.clone(),
            },
        );
    }

    pub fn add_project_reference(&mut self, reference: &ProjectReference) {
        let from_id = format!("project:{}", reference.from_project);
        let to_id = format!("project:{}", reference.to_project);
        let from_idx = self.ensure_node(
            &from_id,
            NodeData::Project {
                name: reference.from_project.clone(),
            },
        );
        let to_idx = self.ensure_node(
            &to_id,
            NodeData::Project {
                name: reference.to_project.clone(),
            },
        );
        self.graph.add_edge(
            from_idx,
            to_idx,
            EdgeData::ProjectReference {
                ref_type: reference.ref_type.clone(),
            },
        );
    }

    pub fn add_package_reference(&mut self, reference: &PackageReference) {
        let pkg_id = format!("package:{}", reference.package);
        let proj_id = format!("project:{}", reference.project);
        let proj_idx = self.ensure_node(
            &proj_id,
            NodeData::Project {
                name: reference.project.clone(),
            },
        );
        let pkg_idx = self.ensure_node(
            &pkg_id,
            NodeData::Package {
                name: reference.package.clone(),
            },
        );
        self.graph.add_edge(
            proj_idx,
            pkg_idx,
            EdgeData::PackageReference {
                version: reference.version.clone(),
            },
        );
    }

    pub fn add_community(&mut self, community: &Community) {
        let comm_idx = self.ensure_node(
            &community.id,
            NodeData::Community {
                id: community.id.clone(),
                label: community.label.clone(),
                cohesion: community.cohesion,
                primary_language: community.primary_language.clone(),
            },
        );
        for member in &community.members {
            if let Some(&member_idx) = self.id_index.get(member) {
                self.graph
                    .add_edge(member_idx, comm_idx, EdgeData::MemberOf);
            }
        }
    }

    pub fn add_process(&mut self, process: &Process) {
        let proc_idx = self.ensure_node(
            &process.id,
            NodeData::Process {
                id: process.id.clone(),
                entry: process.entry.clone(),
                terminal: process.terminal.clone(),
                process_type: process.process_type.clone(),
                total_confidence: process.total_confidence,
            },
        );
        for (i, step) in process.steps.iter().enumerate() {
            if let Some(&step_idx) = self.id_index.get(step) {
                self.graph
                    .add_edge(proc_idx, step_idx, EdgeData::Step { order: i });
            }
        }
    }

    // --- Queries ---

    pub fn get_files(&self) -> Vec<&NodeData> {
        self.graph
            .node_weights()
            .filter(|n| matches!(n, NodeData::File { .. }))
            .collect()
    }

    pub fn get_folders(&self) -> Vec<&NodeData> {
        self.graph
            .node_weights()
            .filter(|n| matches!(n, NodeData::Folder { .. }))
            .collect()
    }

    pub fn get_symbols(&self) -> Vec<SymbolInfo> {
        self.graph
            .node_weights()
            .filter_map(|n| {
                if let NodeData::Symbol {
                    id,
                    name,
                    symbol_type,
                    file,
                    line,
                    visibility,
                    exported,
                    parent,
                    language,
                    parameter_types,
                } = n
                {
                    Some(SymbolInfo {
                        id: id.clone(),
                        name: name.clone(),
                        symbol_type: symbol_type.clone(),
                        file: file.clone(),
                        line: *line,
                        visibility: visibility.clone(),
                        exported: *exported,
                        parent: parent.clone(),
                        language: language.clone(),
                        parameter_types: parameter_types.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn get_symbols_in_file(&self, path: &str) -> Vec<SymbolInfo> {
        let file_id = format!("file:{path}");
        let Some(&file_idx) = self.id_index.get(&file_id) else {
            return Vec::new();
        };
        let mut result = Vec::new();
        for edge_idx in self.graph.edges(file_idx) {
            if matches!(edge_idx.weight(), EdgeData::Defines) {
                let target_idx = edge_idx.target();
                if let Some(NodeData::Symbol {
                    id,
                    name,
                    symbol_type,
                    file,
                    line,
                    visibility,
                    exported,
                    parent,
                    language,
                    parameter_types,
                }) = self.graph.node_weight(target_idx)
                {
                    result.push(SymbolInfo {
                        id: id.clone(),
                        name: name.clone(),
                        symbol_type: symbol_type.clone(),
                        file: file.clone(),
                        line: *line,
                        visibility: visibility.clone(),
                        exported: *exported,
                        parent: parent.clone(),
                        language: language.clone(),
                        parameter_types: parameter_types.clone(),
                    });
                }
            }
        }
        result
    }

    pub fn get_callers(&self, symbol_id: &str) -> Vec<CallInfo> {
        let Some(&sym_idx) = self.id_index.get(symbol_id) else {
            return Vec::new();
        };
        let mut result = Vec::new();
        for edge in self
            .graph
            .edges_directed(sym_idx, petgraph::Direction::Incoming)
        {
            if let EdgeData::Calls {
                confidence,
                tier,
                reason,
                line,
            } = edge.weight()
            {
                let source_idx = edge.source();
                // Find the source ID from id_index (reverse lookup)
                if let Some(source_id) = self.node_id(source_idx) {
                    result.push(CallInfo {
                        id: source_id,
                        confidence: *confidence,
                        tier: tier.clone(),
                        reason: reason.clone(),
                        line: *line,
                    });
                }
            }
        }
        result
    }

    pub fn get_callees(&self, symbol_id: &str) -> Vec<CallInfo> {
        let Some(&sym_idx) = self.id_index.get(symbol_id) else {
            return Vec::new();
        };
        let mut result = Vec::new();
        for edge in self
            .graph
            .edges_directed(sym_idx, petgraph::Direction::Outgoing)
        {
            if let EdgeData::Calls {
                confidence,
                tier,
                reason,
                line,
            } = edge.weight()
            {
                let target_idx = edge.target();
                if let Some(target_id) = self.node_id(target_idx) {
                    result.push(CallInfo {
                        id: target_id,
                        confidence: *confidence,
                        tier: tier.clone(),
                        reason: reason.clone(),
                        line: *line,
                    });
                }
            }
        }
        result
    }

    pub fn get_call_edges(&self) -> Vec<(String, String, f64, String, String, usize)> {
        let mut result = Vec::new();
        for edge in self.graph.edge_indices() {
            if let Some(EdgeData::Calls {
                confidence,
                tier,
                reason,
                line,
            }) = self.graph.edge_weight(edge)
            {
                let (src_idx, tgt_idx) = self.graph.edge_endpoints(edge).unwrap();
                if let (Some(src_id), Some(tgt_id)) = (self.node_id(src_idx), self.node_id(tgt_idx))
                {
                    result.push((
                        src_id,
                        tgt_id,
                        *confidence,
                        tier.clone(),
                        reason.clone(),
                        *line,
                    ));
                }
            }
        }
        result
    }

    pub fn get_import_edges(&self) -> Vec<(String, String, String)> {
        let mut result = Vec::new();
        for edge in self.graph.edge_indices() {
            if let Some(EdgeData::Imports { statement }) = self.graph.edge_weight(edge) {
                let (src_idx, tgt_idx) = self.graph.edge_endpoints(edge).unwrap();
                if let (Some(src_id), Some(tgt_id)) = (self.node_id(src_idx), self.node_id(tgt_idx))
                {
                    let src_path = src_id.strip_prefix("file:").unwrap_or(&src_id);
                    let tgt_path = tgt_id.strip_prefix("file:").unwrap_or(&tgt_id);
                    result.push((
                        src_path.to_string(),
                        tgt_path.to_string(),
                        statement.clone(),
                    ));
                }
            }
        }
        result
    }

    pub fn get_project_references(&self) -> Vec<(String, String, String)> {
        let mut result = Vec::new();
        for edge in self.graph.edge_indices() {
            if let Some(EdgeData::ProjectReference { ref_type }) = self.graph.edge_weight(edge) {
                let (src_idx, tgt_idx) = self.graph.edge_endpoints(edge).unwrap();
                if let (Some(src_id), Some(tgt_id)) = (self.node_id(src_idx), self.node_id(tgt_idx))
                {
                    let src_name = src_id.strip_prefix("project:").unwrap_or(&src_id);
                    let tgt_name = tgt_id.strip_prefix("project:").unwrap_or(&tgt_id);
                    result.push((src_name.to_string(), tgt_name.to_string(), ref_type.clone()));
                }
            }
        }
        result
    }

    pub fn get_package_references(&self) -> Vec<(String, String, String)> {
        let mut result = Vec::new();
        for edge in self.graph.edge_indices() {
            if let Some(EdgeData::PackageReference { version }) = self.graph.edge_weight(edge) {
                let (src_idx, tgt_idx) = self.graph.edge_endpoints(edge).unwrap();
                if let (Some(src_id), Some(tgt_id)) = (self.node_id(src_idx), self.node_id(tgt_idx))
                {
                    let proj = src_id.strip_prefix("project:").unwrap_or(&src_id);
                    let pkg_name = match self.graph.node_weight(tgt_idx) {
                        Some(NodeData::Package { name }) => name.clone(),
                        _ => tgt_id
                            .strip_prefix("package:")
                            .unwrap_or(&tgt_id)
                            .to_string(),
                    };
                    result.push((proj.to_string(), pkg_name, version.clone()));
                }
            }
        }
        result
    }

    pub fn get_communities(&self) -> Vec<(String, String, Vec<String>, f64, String)> {
        let mut results = Vec::new();
        for &node_idx in self.id_index.values() {
            if let Some(NodeData::Community {
                id,
                label,
                cohesion,
                primary_language,
            }) = self.graph.node_weight(node_idx)
            {
                let members: Vec<String> = self
                    .graph
                    .edges_directed(node_idx, petgraph::Direction::Incoming)
                    .filter(|e| matches!(e.weight(), EdgeData::MemberOf))
                    .filter_map(|e| self.node_id(e.source()))
                    .collect();
                results.push((
                    id.clone(),
                    label.clone(),
                    members,
                    *cohesion,
                    primary_language.clone(),
                ));
            }
        }
        results
    }

    pub fn get_processes(&self) -> Vec<(String, String, String, Vec<String>, String, f64)> {
        let mut results = Vec::new();
        for &node_idx in self.id_index.values() {
            if let Some(NodeData::Process {
                id,
                entry,
                terminal,
                process_type,
                total_confidence,
            }) = self.graph.node_weight(node_idx)
            {
                let mut steps: Vec<(usize, String)> = self
                    .graph
                    .edges_directed(node_idx, petgraph::Direction::Outgoing)
                    .filter_map(|e| {
                        if let EdgeData::Step { order } = e.weight() {
                            self.node_id(e.target()).map(|id| (*order, id))
                        } else {
                            None
                        }
                    })
                    .collect();
                steps.sort_by_key(|(order, _)| *order);
                let step_ids: Vec<String> = steps.into_iter().map(|(_, id)| id).collect();
                results.push((
                    id.clone(),
                    entry.clone(),
                    terminal.clone(),
                    step_ids,
                    process_type.clone(),
                    *total_confidence,
                ));
            }
        }
        results
    }

    // --- Counts ---

    pub fn symbol_count(&self) -> usize {
        self.graph
            .node_weights()
            .filter(|n| matches!(n, NodeData::Symbol { .. }))
            .count()
    }

    pub fn file_count(&self) -> usize {
        self.graph
            .node_weights()
            .filter(|n| matches!(n, NodeData::File { .. }))
            .count()
    }

    pub fn folder_count(&self) -> usize {
        self.graph
            .node_weights()
            .filter(|n| matches!(n, NodeData::Folder { .. }))
            .count()
    }

    /// Reverse lookup: NodeIndex → String ID.
    fn node_id(&self, idx: NodeIndex) -> Option<String> {
        // Build reverse lookup from id_index
        for (id, &node_idx) in &self.id_index {
            if node_idx == idx {
                return Some(id.clone());
            }
        }
        None
    }

    /// Access the underlying petgraph for algorithms that need it.
    pub fn inner_graph(&self) -> &DiGraph<NodeData, EdgeData> {
        &self.graph
    }

    /// Access the ID index for external algorithms.
    pub fn id_index(&self) -> &HashMap<String, NodeIndex> {
        &self.id_index
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{SymbolType, Visibility};

    #[test]
    fn add_file_and_query() {
        let mut kg = KnowledgeGraph::new();
        kg.add_file(&FileNode {
            path: "src/main.cs".to_string(),
            language: Some("C#".to_string()),
            size: 1024,
            lines: 50,
        });
        let files = kg.get_files();
        assert_eq!(files.len(), 1);
        assert!(kg.has_node("file:src/main.cs"));
    }

    #[test]
    fn add_symbol_creates_defines_edge() {
        let mut kg = KnowledgeGraph::new();
        kg.add_file(&FileNode {
            path: "src/main.cs".to_string(),
            language: Some("C#".to_string()),
            size: 1024,
            lines: 50,
        });
        kg.add_symbol(&Symbol {
            id: "sym:MyClass.Run".to_string(),
            name: "Run".to_string(),
            symbol_type: SymbolType::Method,
            file: "src/main.cs".to_string(),
            line: 10,
            visibility: Visibility::Public,
            exported: true,
            parent: Some("MyClass".to_string()),
            language: Some("C#".to_string()),
            byte_range: None,
            parameter_types: None,
        });
        assert_eq!(kg.symbol_count(), 1);
        let syms = kg.get_symbols_in_file("src/main.cs");
        assert_eq!(syms.len(), 1);
        assert_eq!(syms[0].name, "Run");
    }

    #[test]
    fn add_call_and_query() {
        let mut kg = KnowledgeGraph::new();
        let sym_a = Symbol {
            id: "sym:A".to_string(),
            name: "A".to_string(),
            symbol_type: SymbolType::Method,
            file: "a.cs".to_string(),
            line: 1,
            visibility: Visibility::Public,
            exported: true,
            parent: None,
            language: None,
            byte_range: None,
            parameter_types: None,
        };
        let sym_b = Symbol {
            id: "sym:B".to_string(),
            name: "B".to_string(),
            symbol_type: SymbolType::Method,
            file: "b.cs".to_string(),
            line: 1,
            visibility: Visibility::Public,
            exported: true,
            parent: None,
            language: None,
            byte_range: None,
            parameter_types: None,
        };
        kg.add_symbol(&sym_a);
        kg.add_symbol(&sym_b);
        kg.add_call(&CallEdge {
            from_symbol: "sym:A".to_string(),
            to_symbol: "sym:B".to_string(),
            confidence: 0.85,
            tier: "A".to_string(),
            reason: "import-resolved".to_string(),
            line: 5,
        });
        let callees = kg.get_callees("sym:A");
        assert_eq!(callees.len(), 1);
        assert_eq!(callees[0].id, "sym:B");

        let callers = kg.get_callers("sym:B");
        assert_eq!(callers.len(), 1);
        assert_eq!(callers[0].id, "sym:A");
    }

    #[test]
    fn add_folder_and_query() {
        let mut kg = KnowledgeGraph::new();
        kg.add_folder(&crate::config::FolderNode {
            path: "src/services".to_string(),
            file_count: 3,
        });
        let folders = kg.get_folders();
        assert_eq!(folders.len(), 1);
        assert!(kg.has_node("folder:src/services"));
        assert_eq!(kg.folder_count(), 1);
    }

    #[test]
    fn add_import_and_query() {
        let mut kg = KnowledgeGraph::new();
        kg.add_import(&crate::config::ImportEdge {
            from_file: "a.cs".to_string(),
            to_file: "b.cs".to_string(),
            statement: "using B".to_string(),
        });
        let edges = kg.get_import_edges();
        assert_eq!(edges.len(), 1);
        assert_eq!(edges[0].0, "a.cs");
        assert_eq!(edges[0].1, "b.cs");
    }

    #[test]
    fn add_project_reference_and_query() {
        let mut kg = KnowledgeGraph::new();
        kg.add_project_reference(&crate::config::ProjectReference {
            from_project: "Web.csproj".to_string(),
            to_project: "Core.csproj".to_string(),
            ref_type: "ProjectReference".to_string(),
        });
        let refs = kg.get_project_references();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].0, "Web.csproj");
        assert_eq!(refs[0].1, "Core.csproj");
    }

    #[test]
    fn add_package_reference_and_query() {
        let mut kg = KnowledgeGraph::new();
        kg.add_package_reference(&crate::config::PackageReference {
            project: "Web.csproj".to_string(),
            package: "Newtonsoft.Json".to_string(),
            version: "13.0.1".to_string(),
        });
        let refs = kg.get_package_references();
        assert_eq!(refs.len(), 1);
        assert_eq!(refs[0].1, "Newtonsoft.Json");
        assert_eq!(refs[0].2, "13.0.1");
    }

    #[test]
    fn add_community_and_query() {
        let mut kg = KnowledgeGraph::new();
        // Add symbols first
        kg.add_symbol(&Symbol {
            id: "sym:X".to_string(),
            name: "X".to_string(),
            symbol_type: SymbolType::Method,
            file: "x.cs".to_string(),
            line: 1,
            visibility: Visibility::Public,
            exported: true,
            parent: None,
            language: None,
            byte_range: None,
            parameter_types: None,
        });
        kg.add_symbol(&Symbol {
            id: "sym:Y".to_string(),
            name: "Y".to_string(),
            symbol_type: SymbolType::Method,
            file: "y.cs".to_string(),
            line: 1,
            visibility: Visibility::Public,
            exported: true,
            parent: None,
            language: None,
            byte_range: None,
            parameter_types: None,
        });
        kg.add_community(&crate::config::Community {
            id: "community_0".to_string(),
            label: "TestCommunity".to_string(),
            members: vec!["sym:X".to_string(), "sym:Y".to_string()],
            cohesion: 0.5,
            primary_language: "C#".to_string(),
        });
        let communities = kg.get_communities();
        assert_eq!(communities.len(), 1);
        assert_eq!(communities[0].1, "TestCommunity");
        assert_eq!(communities[0].2.len(), 2);
    }
}
