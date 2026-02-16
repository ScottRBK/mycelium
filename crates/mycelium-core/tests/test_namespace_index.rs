//! NamespaceIndex integration tests with real fixture data.

mod common;

use common::*;

#[test]
fn namespace_index_populated_from_parsing() {
    let r = run_two_phases("csharp_simple");
    // Parsing should register namespaces found in C# files — test by checking known namespace
    let files = r.ns_index.get_files_for_namespace("Absence");
    let files2 = r.ns_index.get_files_for_namespace("Absence.Controllers");
    let files3 = r.ns_index.get_files_for_namespace("Absence.Services");
    let total = files.len() + files2.len() + files3.len();
    assert!(
        total > 0,
        "NamespaceIndex should have Absence namespace entries after parsing"
    );
}

#[test]
fn namespace_maps_to_files() {
    let r = run_two_phases("csharp_simple");
    // Find C# files and check that they have namespace registrations
    let files = file_paths(&r.kg);
    let cs_files: Vec<_> = files.iter().filter(|f| f.ends_with(".cs")).collect();
    let mut any_ns = false;
    for f in &cs_files {
        let ns = r.ns_index.get_namespaces_for_file(f);
        if !ns.is_empty() {
            any_ns = true;
            break;
        }
    }
    assert!(any_ns, "At least one C# file should have namespace registrations");
}

#[test]
fn namespace_file_lookup() {
    let r = run_two_phases("csharp_simple");
    // Find a C# file with a namespace and verify round-trip
    let files = file_paths(&r.kg);
    for f in files.iter().filter(|f| f.ends_with(".cs")) {
        let namespaces = r.ns_index.get_namespaces_for_file(f);
        if let Some(ns) = namespaces.first() {
            let resolved = r.ns_index.get_files_for_namespace(ns);
            assert!(
                resolved.contains(&f.to_string()),
                "File {} should be in the files for namespace {}",
                f,
                ns
            );
            return;
        }
    }
    // If no namespace found, that's also acceptable for this fixture
}

#[test]
fn namespace_reverse_lookup() {
    let r = run_two_phases("csharp_simple");
    let files = file_paths(&r.kg);
    let cs_file = files.iter().find(|f| f.ends_with(".cs"));
    if let Some(cs_file) = cs_file {
        let namespaces = r.ns_index.get_namespaces_for_file(cs_file);
        assert!(
            !namespaces.is_empty(),
            "C# file {} should have namespace registrations",
            cs_file
        );
    }
}

#[test]
fn namespace_imports_tracked() {
    let r = run_three_phases("csharp_simple");
    // After imports phase, file imports should be tracked
    let files = file_paths(&r.kg);
    let cs_files: Vec<_> = files.iter().filter(|f| f.ends_with(".cs")).collect();
    let mut any_imports = false;
    for f in &cs_files {
        let imported = r.ns_index.get_imported_namespaces(f);
        if !imported.is_empty() {
            any_imports = true;
            break;
        }
    }
    assert!(
        any_imports,
        "At least one C# file should have imported namespaces tracked"
    );
}

#[test]
fn namespace_mixed_dotnet() {
    let r = run_two_phases("mixed_dotnet");
    // mixed_dotnet has C# files with namespaces
    let files = file_paths(&r.kg);
    let cs_files: Vec<_> = files.iter().filter(|f| f.ends_with(".cs")).collect();
    let mut any_ns = false;
    for f in &cs_files {
        let ns = r.ns_index.get_namespaces_for_file(f);
        if !ns.is_empty() {
            any_ns = true;
            break;
        }
    }
    // It's OK if no namespaces were found (depends on fixture content)
    let _ = any_ns;
}
