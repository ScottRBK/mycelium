//! Dual HashMap symbol table: file-scoped index + global index.

use std::collections::HashMap;

use crate::config::Symbol;

/// Lightweight record for the global index.
#[derive(Debug, Clone)]
pub struct SymbolDefinition {
    pub symbol_id: String,
    pub name: String,
    pub file: String,
    pub symbol_type: String,
    pub language: Option<String>,
}

/// Dual HashMap for symbol lookups.
///
/// - `file_index`: file_path → symbol_name → symbol_id
/// - `global_index`: symbol_name → Vec<SymbolDefinition>
pub struct SymbolTable {
    file_index: HashMap<String, HashMap<String, String>>,
    global_index: HashMap<String, Vec<SymbolDefinition>>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            file_index: HashMap::new(),
            global_index: HashMap::new(),
        }
    }

    pub fn add(&mut self, symbol: &Symbol) {
        // File index
        self.file_index
            .entry(symbol.file.clone())
            .or_default()
            .insert(symbol.name.clone(), symbol.id.clone());

        // Global index
        let defn = SymbolDefinition {
            symbol_id: symbol.id.clone(),
            name: symbol.name.clone(),
            file: symbol.file.clone(),
            symbol_type: symbol.symbol_type.as_str().to_string(),
            language: symbol.language.clone(),
        };
        self.global_index
            .entry(symbol.name.clone())
            .or_default()
            .push(defn);
    }

    /// Look up a symbol by file path and name. Returns symbol_id or None.
    pub fn lookup_exact(&self, file_path: &str, name: &str) -> Option<&str> {
        self.file_index
            .get(file_path)
            .and_then(|syms| syms.get(name))
            .map(|s| s.as_str())
    }

    /// Look up a symbol name in the global index. Returns all matching definitions.
    pub fn lookup_fuzzy(&self, name: &str) -> &[SymbolDefinition] {
        self.global_index
            .get(name)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Return all symbol_name → symbol_id mappings for a file.
    pub fn get_symbols_in_file(&self, file_path: &str) -> Option<&HashMap<String, String>> {
        self.file_index.get(file_path)
    }

    /// Access the file index directly.
    pub fn file_index(&self) -> &HashMap<String, HashMap<String, String>> {
        &self.file_index
    }

    /// Access the global index directly.
    pub fn global_index(&self) -> &HashMap<String, Vec<SymbolDefinition>> {
        &self.global_index
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{SymbolType, Visibility};

    fn make_symbol(id: &str, name: &str, file: &str) -> Symbol {
        Symbol {
            id: id.to_string(),
            name: name.to_string(),
            symbol_type: SymbolType::Method,
            file: file.to_string(),
            line: 1,
            visibility: Visibility::Public,
            exported: true,
            parent: None,
            language: Some("C#".to_string()),
            byte_range: None,
            parameter_types: None,
        }
    }

    #[test]
    fn exact_lookup() {
        let mut st = SymbolTable::new();
        st.add(&make_symbol("sym:Foo.Bar", "Bar", "foo.cs"));
        assert_eq!(st.lookup_exact("foo.cs", "Bar"), Some("sym:Foo.Bar"));
        assert_eq!(st.lookup_exact("other.cs", "Bar"), None);
    }

    #[test]
    fn fuzzy_lookup() {
        let mut st = SymbolTable::new();
        st.add(&make_symbol("sym:A.Run", "Run", "a.cs"));
        st.add(&make_symbol("sym:B.Run", "Run", "b.cs"));
        let results = st.lookup_fuzzy("Run");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn symbols_in_file() {
        let mut st = SymbolTable::new();
        st.add(&make_symbol("sym:A.X", "X", "a.cs"));
        st.add(&make_symbol("sym:A.Y", "Y", "a.cs"));
        let syms = st.get_symbols_in_file("a.cs").unwrap();
        assert_eq!(syms.len(), 2);
        assert!(syms.contains_key("X"));
        assert!(syms.contains_key("Y"));
    }
}
