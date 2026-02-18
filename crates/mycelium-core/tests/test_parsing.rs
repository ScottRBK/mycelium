//! Phase 2: Parsing phase integration tests.

mod common;

use common::*;

// ---------------------------------------------------------------------------
// C# symbol extraction (17 tests)
// ---------------------------------------------------------------------------

#[test]
fn csharp_extracts_namespace() {
    let r = run_two_phases("csharp_simple");
    let names = symbol_names(&r.kg);
    assert!(
        names.iter().any(|n| n.contains("Absence")),
        "Should extract namespace symbols"
    );
}

#[test]
fn csharp_extracts_classes() {
    let r = run_two_phases("csharp_simple");
    let names = symbol_names(&r.kg);
    assert!(names.contains(&"AbsenceController".to_string()));
    assert!(names.contains(&"AbsenceService".to_string()));
}

#[test]
fn csharp_extracts_interfaces() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let interfaces: Vec<_> = syms.iter().filter(|s| s.symbol_type == "Interface").collect();
    assert!(
        !interfaces.is_empty(),
        "Should extract interface declarations"
    );
    assert!(interfaces
        .iter()
        .any(|s| s.name == "IAbsenceService" || s.name == "IAbsenceRepository"));
}

#[test]
fn csharp_extracts_methods() {
    let r = run_two_phases("csharp_simple");
    let names = symbol_names(&r.kg);
    assert!(names.contains(&"GetEntitlement".to_string()));
    assert!(names.contains(&"CalculateEntitlement".to_string()));
}

#[test]
fn csharp_extracts_constructors() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let constructors: Vec<_> = syms
        .iter()
        .filter(|s| s.symbol_type == "Constructor")
        .collect();
    assert!(
        !constructors.is_empty(),
        "Should extract constructor declarations"
    );
}

#[test]
fn csharp_extracts_properties() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let props: Vec<_> = syms
        .iter()
        .filter(|s| s.symbol_type == "Property")
        .collect();
    assert!(!props.is_empty(), "Should extract property declarations");
}

#[test]
fn csharp_extracts_enums() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let enums: Vec<_> = syms.iter().filter(|s| s.symbol_type == "Enum").collect();
    assert!(!enums.is_empty(), "Should extract enum declarations");
    assert!(enums.iter().any(|s| s.name == "LeaveType"));
}

#[test]
fn csharp_extracts_structs() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let structs: Vec<_> = syms.iter().filter(|s| s.symbol_type == "Struct").collect();
    assert!(!structs.is_empty(), "Should extract struct declarations");
    assert!(structs.iter().any(|s| s.name == "DateRange"));
}

#[test]
fn csharp_visibility_public() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let controller = syms
        .iter()
        .find(|s| s.name == "AbsenceController")
        .unwrap();
    assert_eq!(controller.visibility, "public");
    assert!(controller.exported);
}

#[test]
fn csharp_visibility_internal() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let model = syms.iter().find(|s| s.name == "AbsenceModel").unwrap();
    assert_eq!(model.visibility, "internal");
    assert!(!model.exported);
}

#[test]
fn csharp_visibility_private_method() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let private_methods: Vec<_> = syms
        .iter()
        .filter(|s| s.visibility == "private" && s.symbol_type == "Method")
        .collect();
    assert!(!private_methods.is_empty(), "Should have private methods");
}

#[test]
fn csharp_parent_tracking() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let methods_with_parent: Vec<_> = syms
        .iter()
        .filter(|s| s.symbol_type == "Method" && s.parent.is_some())
        .collect();
    assert!(!methods_with_parent.is_empty(), "Methods should have parent");
}

#[test]
fn csharp_line_numbers() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    for sym in &syms {
        assert!(sym.line > 0, "Line numbers should be > 0");
    }
}

#[test]
fn csharp_language_tag() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    for sym in &syms {
        assert_eq!(
            sym.language.as_deref(),
            Some("C#"),
            "C# symbols should have language tag"
        );
    }
}

#[test]
fn csharp_constructor_parameter_types() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let constructors: Vec<_> = syms
        .iter()
        .filter(|s| s.symbol_type == "Constructor" && s.parameter_types.is_some())
        .collect();
    assert!(
        !constructors.is_empty(),
        "Constructors should have parameter types for DI tracking"
    );
}

#[test]
fn csharp_symbol_count() {
    let r = run_two_phases("csharp_simple");
    let count = r.kg.symbol_count();
    assert!(
        count >= 20,
        "csharp_simple should have at least 20 symbols, got {count}"
    );
}

#[test]
fn csharp_defines_edges() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols_in_file("AbsenceController.cs");
    assert!(!syms.is_empty(), "Should have DEFINES edges");
}

// ---------------------------------------------------------------------------
// VB.NET parsing (1 test)
// ---------------------------------------------------------------------------

#[test]
fn vbnet_files_parsed_in_mixed_dotnet() {
    // VB.NET grammar is now available; parsing should extract .vb symbols
    let r = run_two_phases("mixed_dotnet");
    let syms = r.kg.get_symbols();
    let vb_syms: Vec<_> = syms
        .iter()
        .filter(|s| s.language.as_deref() == Some("VB.NET"))
        .collect();
    assert!(
        !vb_syms.is_empty(),
        "VB.NET symbols should be extracted from mixed_dotnet fixture"
    );
}

// ---------------------------------------------------------------------------
// Mixed .NET (1 test)
// ---------------------------------------------------------------------------

#[test]
fn mixed_dotnet_extracts_csharp() {
    let r = run_two_phases("mixed_dotnet");
    let syms = r.kg.get_symbols();
    let cs_syms: Vec<_> = syms
        .iter()
        .filter(|s| s.language.as_deref() == Some("C#"))
        .collect();
    assert!(!cs_syms.is_empty(), "Should extract C# symbols from mixed .NET project");
}

// ---------------------------------------------------------------------------
// Per-language basics (8 tests)
// ---------------------------------------------------------------------------

#[test]
fn python_parsing_basic() {
    let r = run_two_phases("python_simple");
    let names = symbol_names(&r.kg);
    assert!(
        names.iter().any(|n| n.contains("Service") || n.contains("Handler")),
        "Should extract Python class/function names"
    );
}

#[test]
fn typescript_parsing_basic() {
    let r = run_two_phases("typescript_simple");
    let names = symbol_names(&r.kg);
    assert!(!names.is_empty(), "Should extract TypeScript symbols");
}

#[test]
fn java_parsing_basic() {
    let r = run_two_phases("java_simple");
    let names = symbol_names(&r.kg);
    assert!(names.contains(&"UserController".to_string()));
}

#[test]
fn go_parsing_basic() {
    let r = run_two_phases("go_simple");
    let names = symbol_names(&r.kg);
    assert!(!names.is_empty(), "Should extract Go symbols");
}

#[test]
fn rust_parsing_basic() {
    let r = run_two_phases("rust_simple");
    let names = symbol_names(&r.kg);
    assert!(!names.is_empty(), "Should extract Rust symbols");
}

#[test]
fn c_parsing_basic() {
    let r = run_two_phases("c_simple");
    let names = symbol_names(&r.kg);
    assert!(!names.is_empty(), "Should extract C symbols");
}

#[test]
fn cpp_parsing_basic() {
    let r = run_two_phases("cpp_simple");
    let names = symbol_names(&r.kg);
    assert!(!names.is_empty(), "Should extract C++ symbols");
}

#[test]
fn symbol_ids_unique() {
    let r = run_two_phases("csharp_simple");
    let syms = r.kg.get_symbols();
    let mut seen = std::collections::HashSet::new();
    for sym in &syms {
        assert!(seen.insert(&sym.id), "Duplicate symbol ID: {}", sym.id);
    }
}
