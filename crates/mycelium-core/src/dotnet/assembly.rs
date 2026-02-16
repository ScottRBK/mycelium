//! Namespace-to-project mapping for .NET assemblies.

use std::collections::HashMap;

/// Maps namespaces to the projects that define them.
///
/// Seeded from `<RootNamespace>` in .csproj/.vbproj files, then supplemented
/// by observed namespace declarations in source files.
pub struct AssemblyIndex {
    ns_to_project: HashMap<String, String>,
}

impl AssemblyIndex {
    pub fn new() -> Self {
        Self {
            ns_to_project: HashMap::new(),
        }
    }

    /// Register a namespace as belonging to a project.
    pub fn register(&mut self, namespace: &str, project: &str) {
        self.ns_to_project
            .insert(namespace.to_string(), project.to_string());
    }

    /// Resolve a namespace to the project that owns it.
    ///
    /// Tries exact match first, then prefix matching (e.g.,
    /// `Absence.Services.Internal` matches `Absence.Services` if
    /// `Absence` project registered `Absence` as root namespace).
    pub fn resolve_namespace(&self, namespace: &str) -> Option<&str> {
        // Exact match
        if let Some(project) = self.ns_to_project.get(namespace) {
            return Some(project.as_str());
        }

        // Prefix match: find the longest matching namespace prefix
        let mut best_match: Option<&str> = None;
        let mut best_len = 0;

        for (ns, project) in &self.ns_to_project {
            if namespace.starts_with(ns.as_str()) && ns.len() > best_len {
                // Ensure it matches at a dot boundary
                if namespace.len() == ns.len() || namespace.as_bytes()[ns.len()] == b'.' {
                    best_match = Some(project.as_str());
                    best_len = ns.len();
                }
            }
        }

        best_match
    }

    /// Return the full namespace-to-project mapping.
    pub fn get_all_namespaces(&self) -> &HashMap<String, String> {
        &self.ns_to_project
    }
}

impl Default for AssemblyIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_match() {
        let mut idx = AssemblyIndex::new();
        idx.register("Absence.Services", "Services.csproj");
        assert_eq!(
            idx.resolve_namespace("Absence.Services"),
            Some("Services.csproj")
        );
    }

    #[test]
    fn prefix_match() {
        let mut idx = AssemblyIndex::new();
        idx.register("Absence", "Core.csproj");
        assert_eq!(
            idx.resolve_namespace("Absence.Services.Internal"),
            Some("Core.csproj")
        );
    }

    #[test]
    fn longest_prefix_wins() {
        let mut idx = AssemblyIndex::new();
        idx.register("Absence", "Core.csproj");
        idx.register("Absence.Services", "Services.csproj");
        assert_eq!(
            idx.resolve_namespace("Absence.Services.Internal"),
            Some("Services.csproj")
        );
    }

    #[test]
    fn no_match() {
        let mut idx = AssemblyIndex::new();
        idx.register("Absence", "Core.csproj");
        assert_eq!(idx.resolve_namespace("Completely.Different"), None);
    }
}
