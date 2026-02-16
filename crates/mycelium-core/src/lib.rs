//! Mycelium Core — Static analysis engine for mapping codebase connections.
//!
//! This crate contains all analysis logic: tree-sitter parsing, graph construction,
//! import resolution, call graph building, community detection, and process tracing.

pub mod config;
pub mod dotnet;
pub mod graph;
pub mod languages;
pub mod output;
pub mod phases;
pub mod pipeline;
