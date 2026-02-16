//! Namespace-to-file index for namespace-aware import resolution.

use std::collections::HashMap;

/// Maps namespaces to files and tracks file imports.
pub struct NamespaceIndex {
    /// namespace → list of files that declare it
    ns_to_files: HashMap<String, Vec<String>>,
    /// file → list of namespaces declared in it
    file_to_ns: HashMap<String, Vec<String>>,
    /// file → list of namespace names it imports
    file_imports: HashMap<String, Vec<String>>,
}

impl NamespaceIndex {
    pub fn new() -> Self {
        Self {
            ns_to_files: HashMap::new(),
            file_to_ns: HashMap::new(),
            file_imports: HashMap::new(),
        }
    }

    /// Register that a file declares the given namespace.
    pub fn register(&mut self, namespace: &str, file_path: &str) {
        let files = self.ns_to_files.entry(namespace.to_string()).or_default();
        if !files.contains(&file_path.to_string()) {
            files.push(file_path.to_string());
        }

        let namespaces = self.file_to_ns.entry(file_path.to_string()).or_default();
        if !namespaces.contains(&namespace.to_string()) {
            namespaces.push(namespace.to_string());
        }
    }

    /// Get all files that declare the given namespace.
    pub fn get_files_for_namespace(&self, namespace: &str) -> &[String] {
        self.ns_to_files
            .get(namespace)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Record that a file imports the given namespace.
    pub fn register_file_import(&mut self, file_path: &str, namespace: &str) {
        let imports = self.file_imports.entry(file_path.to_string()).or_default();
        if !imports.contains(&namespace.to_string()) {
            imports.push(namespace.to_string());
        }
    }

    /// Get all namespace names imported by a file.
    pub fn get_imported_namespaces(&self, file_path: &str) -> &[String] {
        self.file_imports
            .get(file_path)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get namespaces declared in a file.
    pub fn get_namespaces_for_file(&self, file_path: &str) -> &[String] {
        self.file_to_ns
            .get(file_path)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }
}

impl Default for NamespaceIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_and_lookup() {
        let mut idx = NamespaceIndex::new();
        idx.register("MyApp.Services", "Services/UserService.cs");
        idx.register("MyApp.Services", "Services/OrderService.cs");

        let files = idx.get_files_for_namespace("MyApp.Services");
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"Services/UserService.cs".to_string()));
    }

    #[test]
    fn no_duplicates() {
        let mut idx = NamespaceIndex::new();
        idx.register("MyApp", "a.cs");
        idx.register("MyApp", "a.cs");
        assert_eq!(idx.get_files_for_namespace("MyApp").len(), 1);
    }

    #[test]
    fn file_imports() {
        let mut idx = NamespaceIndex::new();
        idx.register_file_import("main.cs", "MyApp.Services");
        idx.register_file_import("main.cs", "MyApp.Models");

        let imports = idx.get_imported_namespaces("main.cs");
        assert_eq!(imports.len(), 2);
    }
}
