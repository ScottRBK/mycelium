# Mermaid class diagrams

Declared types and signatures; unknown types are `unknown`. Receivers are omitted.
Fields are associations, not lifetime ownership. Calls are static heuristic estimates.
Members and connections within each view are uncapped.
Parallel arrows are summarized.
Cross-diagram relationships are retained in the complete relationship list.

Included: 125 boxes. Calls without in-scope endpoints: 0.

## Test filtering

Mode: exclude.
Saved detector version: 2.
Test paths: `crates/mycelium-cli/tests`, `crates/mycelium-core/tests`, `tests`.
Keep paths: none.
Calls removed by test filtering: 1335. Type relationships removed: 195.

- `crates/mycelium-cli/tests/mermaid.rs`: 3 source occurrences (test-path).
- `crates/mycelium-core/src/config.rs`: 8 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/dotnet/assembly.rs`: 4 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/dotnet/project.rs`: 4 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/dotnet/solution.rs`: 4 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/graph/knowledge_graph.rs`: 8 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/graph/namespace_index.rs`: 3 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/graph/scoring.rs`: 11 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/graph/symbol_table.rs`: 4 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/output.rs`: 3 source occurrences (rust.cfg-test).
- `crates/mycelium-core/src/phases/communities.rs`: 12 source occurrences (rust.cfg-test).
- `crates/mycelium-core/tests/common/mod.rs`: 22 source occurrences (test-path).
- `crates/mycelium-core/tests/test_calls.rs`: 19 source occurrences (test-path).
- `crates/mycelium-core/tests/test_communities.rs`: 12 source occurrences (test-path).
- `crates/mycelium-core/tests/test_construction_calls.rs`: 16 source occurrences (test-path).
- `crates/mycelium-core/tests/test_framework_filtering.rs`: 24 source occurrences (test-path).
- `crates/mycelium-core/tests/test_imports.rs`: 56 source occurrences (test-path).
- `crates/mycelium-core/tests/test_languages.rs`: 196 source occurrences (test-path).
- `crates/mycelium-core/tests/test_mermaid.rs`: 40 source occurrences (test-path).
- `crates/mycelium-core/tests/test_namespace_index.rs`: 7 source occurrences (test-path).
- `crates/mycelium-core/tests/test_parsing.rs`: 28 source occurrences (test-path).
- `crates/mycelium-core/tests/test_pipeline.rs`: 14 source occurrences (test-path).
- `crates/mycelium-core/tests/test_processes.rs`: 19 source occurrences (test-path).
- `crates/mycelium-core/tests/test_structure.rs`: 9 source occurrences (test-path).
- `crates/mycelium-core/tests/test_test_filtering.rs`: 20 source occurrences (test-path).
- `tests/check_wheel.py`: 2 source occurrences (test-path).
- `tests/checkpoints/check_mermaid.py`: 3 source occurrences (test-path).
- `tests/fixtures/c_simple/main.c`: 9 source occurrences (test-path).
- `tests/fixtures/c_simple/repository.c`: 7 source occurrences (test-path).
- `tests/fixtures/c_simple/repository.h`: 11 source occurrences (test-path).
- `tests/fixtures/c_simple/service.c`: 9 source occurrences (test-path).
- `tests/fixtures/c_simple/service.h`: 15 source occurrences (test-path).
- `tests/fixtures/c_simple/types.c`: 4 source occurrences (test-path).
- `tests/fixtures/c_simple/types.h`: 12 source occurrences (test-path).
- `tests/fixtures/cpp_simple/handler.cpp`: 12 source occurrences (test-path).
- `tests/fixtures/cpp_simple/main.cpp`: 3 source occurrences (test-path).
- `tests/fixtures/cpp_simple/models.hpp`: 13 source occurrences (test-path).
- `tests/fixtures/cpp_simple/repository.cpp`: 14 source occurrences (test-path).
- `tests/fixtures/cpp_simple/repository.hpp`: 10 source occurrences (test-path).
- `tests/fixtures/cpp_simple/service.cpp`: 16 source occurrences (test-path).
- `tests/fixtures/cpp_simple/service.hpp`: 20 source occurrences (test-path).
- `tests/fixtures/csharp_simple/AbsenceController.cs`: 9 source occurrences (test-path).
- `tests/fixtures/csharp_simple/AbsenceException.cs`: 5 source occurrences (test-path).
- `tests/fixtures/csharp_simple/AbsenceModel.cs`: 34 source occurrences (test-path).
- `tests/fixtures/csharp_simple/AbsenceRepository.cs`: 9 source occurrences (test-path).
- `tests/fixtures/csharp_simple/AbsenceService.cs`: 9 source occurrences (test-path).
- `tests/fixtures/csharp_simple/IAbsenceRepository.cs`: 6 source occurrences (test-path).
- `tests/fixtures/csharp_simple/IAbsenceService.cs`: 5 source occurrences (test-path).
- `tests/fixtures/csharp_simple/LeaveRequestValidator.cs`: 5 source occurrences (test-path).
- `tests/fixtures/go_package/main.go`: 2 source occurrences (test-path).
- `tests/fixtures/go_package/middleware/middleware.go`: 8 source occurrences (test-path).
- `tests/fixtures/go_package/model/model.go`: 5 source occurrences (test-path).
- `tests/fixtures/go_package/service/service.go`: 9 source occurrences (test-path).
- `tests/fixtures/go_simple/handler.go`: 14 source occurrences (test-path).
- `tests/fixtures/go_simple/middleware.go`: 15 source occurrences (test-path).
- `tests/fixtures/go_simple/model.go`: 18 source occurrences (test-path).
- `tests/fixtures/go_simple/repository.go`: 20 source occurrences (test-path).
- `tests/fixtures/go_simple/service.go`: 18 source occurrences (test-path).
- `tests/fixtures/java_package/com/example/controllers/UserController.java`: 6 source occurrences
  (test-path).
- `tests/fixtures/java_package/com/example/models/User.java`: 8 source occurrences (test-path).
- `tests/fixtures/java_package/com/example/services/UserService.java`: 5 source occurrences
  (test-path).
- `tests/fixtures/java_simple/InMemoryUserRepository.java`: 9 source occurrences (test-path).
- `tests/fixtures/java_simple/User.java`: 15 source occurrences (test-path).
- `tests/fixtures/java_simple/UserController.java`: 10 source occurrences (test-path).
- `tests/fixtures/java_simple/UserDto.java`: 10 source occurrences (test-path).
- `tests/fixtures/java_simple/UserMapper.java`: 3 source occurrences (test-path).
- `tests/fixtures/java_simple/UserNotFoundException.java`: 4 source occurrences (test-path).
- `tests/fixtures/java_simple/UserRepository.java`: 6 source occurrences (test-path).
- `tests/fixtures/java_simple/UserService.java`: 17 source occurrences (test-path).
- `tests/fixtures/mixed_dotnet/CSharpProject/ApiController.cs`: 4 source occurrences (test-path).
- `tests/fixtures/mixed_dotnet/VBNetProject/DataProcessor.vb`: 2 source occurrences (test-path).
- `tests/fixtures/python_package/app/models/item.py`: 4 source occurrences (test-path).
- `tests/fixtures/python_package/app/models/user.py`: 4 source occurrences (test-path).
- `tests/fixtures/python_package/app/services/user_service.py`: 5 source occurrences (test-path).
- `tests/fixtures/python_package/app/utils/helpers.py`: 2 source occurrences (test-path).
- `tests/fixtures/python_package/app/utils/validators.py`: 2 source occurrences (test-path).
- `tests/fixtures/python_package/main.py`: 2 source occurrences (test-path).
- `tests/fixtures/python_simple/config.py`: 9 source occurrences (test-path).
- `tests/fixtures/python_simple/exceptions.py`: 12 source occurrences (test-path).
- `tests/fixtures/python_simple/handler.py`: 14 source occurrences (test-path).
- `tests/fixtures/python_simple/models.py`: 25 source occurrences (test-path).
- `tests/fixtures/python_simple/repository.py`: 13 source occurrences (test-path).
- `tests/fixtures/python_simple/service.py`: 10 source occurrences (test-path).
- `tests/fixtures/python_simple/validators.py`: 7 source occurrences (test-path).
- `tests/fixtures/rust_simple/error.rs`: 10 source occurrences (test-path).
- `tests/fixtures/rust_simple/main.rs`: 10 source occurrences (test-path).
- `tests/fixtures/rust_simple/model.rs`: 14 source occurrences (test-path).
- `tests/fixtures/rust_simple/repository.rs`: 11 source occurrences (test-path).
- `tests/fixtures/rust_simple/service.rs`: 16 source occurrences (test-path).
- `tests/fixtures/typescript_simple/controller.ts`: 9 source occurrences (test-path).
- `tests/fixtures/typescript_simple/middleware.ts`: 11 source occurrences (test-path).
- `tests/fixtures/typescript_simple/models.ts`: 29 source occurrences (test-path).
- `tests/fixtures/typescript_simple/repository.ts`: 9 source occurrences (test-path).
- `tests/fixtures/typescript_simple/service.ts`: 10 source occurrences (test-path).
- `tests/fixtures/typescript_simple/utils.ts`: 6 source occurrences (test-path).
- `tests/fixtures/vbnet_simple/Calculator.vb`: 26 source occurrences (test-path).
- `tests/fixtures/vbnet_simple/EmployeeModule.vb`: 3 source occurrences (test-path).
- `tests/fixtures/vbnet_simple/EmployeeRepository.vb`: 17 source occurrences (test-path).
- `tests/fixtures/vbnet_simple/EmployeeService.vb`: 5 source occurrences (test-path).
- `tests/fixtures/vbnet_simple/EmployeeTypes.vb`: 7 source occurrences (test-path).
- `tests/fixtures/vbnet_simple/EmployeeUtils.vb`: 6 source occurrences (test-path).
- `tests/test_bindings.py`: 10 source occurrences (test-path).
- `crates/mycelium-core/src/mermaid.rs: syntax-derived test detection disabled; malformed syntax`
- `crates/mycelium-core/tests/test_calls.rs: test attribute binding is uncertain; #[test] detection
  disabled`
- `crates/mycelium-core/tests/test_communities.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `crates/mycelium-core/tests/test_imports.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `crates/mycelium-core/tests/test_languages.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `crates/mycelium-core/tests/test_namespace_index.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `crates/mycelium-core/tests/test_parsing.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `crates/mycelium-core/tests/test_pipeline.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `crates/mycelium-core/tests/test_processes.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `crates/mycelium-core/tests/test_structure.rs: test attribute binding is uncertain; #[test]
  detection disabled`
- `tests/fixtures/vbnet_simple/EmployeeModule.vb: syntax-derived test detection disabled; malformed
  syntax`
- `tests/fixtures/vbnet_simple/EmployeeRepository.vb: syntax-derived test detection disabled;
  malformed syntax`

## Diagram 1

```mermaid
classDiagram
    direction TB
    class c0000["Cli"] {
        <<struct>>
        -command: Commands
    }
    class c0001["Commands"] {
        <<enum>>
        -Export: Type1
        -Analyze: Type2
    }
    class c0002["crates/mycelium-cli/src/main.rs"] {
        <<module>>
        -main() Type3
        -run_quiet(config: Type4, output_path: Type5) Type3
        -run_with_progress(config: Type4, output_path: Type5, verbose: bool) Type3
        -run_export(input: Type7, output: Type7, options: Type8) Type6
    }
    class c0004["AnalysisConfig"] {
        <<struct>>
        +repo_path: String
        +output_path: Option~String~
        +languages: Option~Vec~String~~
        +resolution: f64
        +max_processes: usize
        +max_depth: usize
        +max_branching: usize
        +min_steps: usize
        +exclude_patterns: Vec~String~
        +verbose: bool
        +quiet: bool
        +max_file_size: u64
        +max_community_size: usize
        +default() Self
        +from(py_config: PyAnalysisConfig) Self
    }
    class c0005["AnalysisResult"] {
        <<struct>>
        +class_diagram: Type9
        +version: String
        +metadata: Type10
        +stats: Type10
        +structure: StructureOutput
        +symbols: Vec~SymbolOutput~
        +imports: ImportsOutput
        +calls: Vec~CallOutput~
        +communities: Vec~CommunityOutput~
        +processes: Vec~ProcessOutput~
        +default() Self
    }
    class c0006["CallEdge"] {
        <<struct>>
        +from_symbol: String
        +to_symbol: String
        +confidence: f64
        +tier: String
        +reason: String
        +line: usize
    }
    class c0007["CallOutput"] {
        <<struct>>
        +from: String
        +to: String
        +confidence: f64
        +tier: String
        +reason: String
        +line: usize
    }
    class c0008["Community"] {
        <<struct>>
        +id: String
        +label: String
        +members: Vec~String~
        +cohesion: f64
        +primary_language: String
    }
    class c0009["CommunityOutput"] {
        <<struct>>
        +id: String
        +label: String
        +members: Vec~String~
        +cohesion: f64
        +primary_language: String
    }
    class c0010["FileNode"] {
        <<struct>>
        +path: String
        +language: Option~String~
        +size: u64
        +lines: usize
    }
    class c0011["FileOutput"] {
        <<struct>>
        +path: String
        +language: Option~String~
        +size: u64
        +lines: usize
    }
    class c0012["FolderNode"] {
        <<struct>>
        +path: String
        +file_count: usize
    }
    class c0013["FolderOutput"] {
        <<struct>>
        +path: String
        +file_count: usize
    }
    class c0014["ImportEdge"] {
        <<struct>>
        +from_file: String
        +to_file: String
        +statement: String
    }
    class c0015["ImportOutput"] {
        <<struct>>
        +from: String
        +to: String
        +statement: String
    }
    class c0016["ImportStatement"] {
        <<struct>>
        +file: String
        +statement: String
        +target_name: String
        +line: usize
    }
    class c0017["ImportsOutput"] {
        <<struct>>
        +file_imports: Vec~ImportOutput~
        +project_references: Vec~ProjectRefOutput~
        +package_references: Vec~PackageRefOutput~
    }
    class c0018["PackageRefOutput"] {
        <<struct>>
        +project: String
        +package: String
        +version: String
    }
    class c0019["PackageReference"] {
        <<struct>>
        +project: String
        +package: String
        +version: String
    }
    class c0020["Process"] {
        <<struct>>
        +id: String
        +entry: String
        +terminal: String
        +steps: Vec~String~
        +process_type: String
        +total_confidence: f64
    }
    class c0021["ProcessOutput"] {
        <<struct>>
        +id: String
        +entry: String
        +terminal: String
        +steps: Vec~String~
        +process_type: String
        +total_confidence: f64
    }
    class c0022["ProjectRefOutput"] {
        <<struct>>
        +from: String
        +to: String
        +ref_type: String
    }
    class c0023["ProjectReference"] {
        <<struct>>
        +from_project: String
        +to_project: String
        +ref_type: String
    }
    class c0024["RawCall"] {
        <<struct>>
        +caller_file: String
        +caller_name: String
        +callee_name: String
        +line: usize
        +qualifier: Option~String~
    }
    class c0025["StructureOutput"] {
        <<struct>>
        +files: Vec~FileOutput~
        +folders: Vec~FolderOutput~
    }
    class c0026["Symbol"] {
        <<struct>>
        +id: String
        +name: String
        +symbol_type: SymbolType
        +file: String
        +line: usize
        +visibility: Visibility
        +exported: bool
        +parent: Option~String~
        +language: Option~String~
        +byte_range: Type11
        +parameter_types: Type12
    }
    class c0027["SymbolOutput"] {
        <<struct>>
        +id: String
        +name: String
        +symbol_type: String
        +file: String
        +line: usize
        +visibility: String
        +exported: bool
        +parent: Option~String~
        +language: Option~String~
    }
    class c0028["SymbolType"] {
        <<enum>>
        -Class: unknown
        -Function: unknown
        -Method: unknown
        -Interface: unknown
        -Struct: unknown
        -Enum: unknown
        -Namespace: unknown
        -Property: unknown
        -Constructor: unknown
        -Module: unknown
        -Record: unknown
        -Delegate: unknown
        -TypeAlias: unknown
        -Constant: unknown
        -Variable: unknown
        -Trait: unknown
        -Impl: unknown
        -Macro: unknown
        -Template: unknown
        -Typedef: unknown
        -Annotation: unknown
        -Static: unknown
        +as_str() Type13
        +from_str_value(s: Type5) Option~Self~
        +fmt(f: Type14) std::fmt::Result
    }
    class c0029["Visibility"] {
        <<enum>>
        -Public: unknown
        -Private: unknown
        -Internal: unknown
        -Protected: unknown
        -Friend: unknown
        -Unknown: unknown
        +as_str() Type13
        +fmt(f: Type14) std::fmt::Result
    }
    class c0030["crates/mycelium-core/src/config.rs"] {
        <<module>>
        -default_project_ref_type() String
        -default_process_type() String
        -default_resolution() f64
        -default_max_processes() usize
        -default_max_depth() usize
        -default_max_branching() usize
        -default_min_steps() usize
        -default_max_file_size() u64
        -default_max_community_size() usize
        -default_version() String
    }
    class c0031["Base"] {
        <<struct>>
        +name: String
        +relation: String
    }
    class c0032["Class"] {
        <<struct>>
        +id: String
        +name: String
        +kind: String
        +file: String
        +line: usize
        +test: Option~TestEvidence~
        +members: Vec~Member~
        +bases: Vec~Base~
    }
    class c0033["ClassDiagram"] {
        <<struct>>
        +classes: Vec~Class~
        +warnings: Vec~String~
        +test_detection: Option~TestDetection~
    }
    class c0034["Member"] {
        <<struct>>
        +file: String
        +name: String
        +kind: String
        +visibility: String
        +parameters: Vec~Parameter~
        +value_type: Option~String~
        +line: usize
        +end_line: usize
        +test: Option~TestEvidence~
    }
    class c0035["Parameter"] {
        <<struct>>
        +name: String
        +value_type: Option~String~
    }
    class c0036["TestDetection"] {
        <<struct>>
        +version: u32
        +diagnostics: Vec~TestDiagnostic~
        +default() Self
    }
    class c0037["TestDiagnostic"] {
        <<struct>>
        +file: String
        +message: String
    }
    class c0038["TestEvidence"] {
        <<struct>>
        +rule: String
        +file: String
        +line: usize
    }
    class c0039["crates/mycelium-core/src/declarations.rs"] {
        <<module>>
        +supported_test_rule(version: u32, rule: Type5) bool
    }
    class c0040["AssemblyIndex"] {
        <<struct>>
        -ns_to_project: Type15
        +new() Self
        +register(namespace: Type5, project: Type5) Type3
        +resolve_namespace(namespace: Type5) Type16
        +get_all_namespaces() Type17
        +default() Self
    }
    class c0042["ProjectFile"] {
        <<struct>>
        +name: String
        +target_framework: Option~String~
        +root_namespace: Option~String~
        +assembly_name: Option~String~
        +project_references: Vec~String~
        +package_references: Type18
    }
    class c0043["crates/mycelium-core/src/dotnet/project.rs"] {
        <<module>>
        +parse_project_file(content: Type5, project_path: Type5) ProjectFile
        -extract_element_text(content: Type5, tag: Type5) Option~String~
        -extract_include_attrs(content: Type5, tag: Type5) Vec~String~
        -extract_package_refs(content: Type5) Type18
        -extract_attr(element: Type5, attr: Type5) Option~String~
    }
    class c0044["SlnProject"] {
        <<struct>>
        +name: String
        +path: String
        +project_type_guid: String
        +project_guid: String
    }
    class c0045["crates/mycelium-core/src/dotnet/solution.rs"] {
        <<module>>
        +parse_solution(content: Type5) Vec~SlnProject~
    }
    class c0046["CallInfo"] {
        <<struct>>
        +id: String
        +confidence: f64
        +tier: String
        +reason: String
        +line: usize
    }
    class c0047["EdgeData"] {
        <<enum>>
        -Defines: unknown
        -Imports: Type19
        -Calls: Type20
        -ProjectReference: Type21
        -PackageReference: Type22
        -MemberOf: unknown
        -Step: Type23
        -Contains: unknown
        +edge_type() Type13
    }
    class c0048["KnowledgeGraph"] {
        <<struct>>
        +class_diagram: crate::declarations::ClassDiagram
        -graph: Type24
        -id_index: Type25
        +new() Self
        -ensure_node(id: Type5, data: NodeData) NodeIndex
        +get_node_index(id: Type5) Option~NodeIndex~
        +get_node_data(id: Type5) Type26
        +has_node(id: Type5) bool
        +add_file(node: Type27) Type3
        +add_folder(node: Type28) Type3
        +add_symbol(symbol: Type29) Type3
        +add_call(edge: Type30) Type3
        +add_import(edge: Type31) Type3
        +add_project_reference(reference: Type32) Type3
        +add_package_reference(reference: Type33) Type3
        +add_community(community: Type34) Type3
        +add_process(process: Type35) Type3
        +get_files() Type36
        +get_folders() Type36
        +get_symbols() Vec~SymbolInfo~
        +get_symbols_in_file(path: Type5) Vec~SymbolInfo~
        +get_callers(symbol_id: Type5) Vec~CallInfo~
        +get_callees(symbol_id: Type5) Vec~CallInfo~
        +get_call_edges() Type37
        +get_import_edges() Type38
        +get_project_references() Type38
        +get_package_references() Type38
        +get_communities() Type39
        +get_processes() Type40
        +symbol_count() usize
        +file_count() usize
        +folder_count() usize
        -node_id(idx: NodeIndex) Option~String~
        +inner_graph() Type41
        +id_index() Type42
        +default() Self
    }
    class c0049["NodeData"] {
        <<enum>>
        -File: Type43
        -Folder: Type44
        -Symbol: Type45
        -Community: Type46
        -Process: Type47
        -Package: Type48
        -Project: Type48
        +node_type() Type13
    }
    class c0050["SymbolInfo"] {
        <<struct>>
        +id: String
        +name: String
        +symbol_type: String
        +file: String
        +line: usize
        +visibility: String
        +exported: bool
        +parent: Option~String~
        +language: Option~String~
        +parameter_types: Type12
    }
    class c0052["NamespaceIndex"] {
        <<struct>>
        -ns_to_files: Type49
        -file_to_ns: Type49
        -file_imports: Type49
        +new() Self
        +register(namespace: Type5, file_path: Type5) Type3
        +get_files_for_namespace(namespace: Type5) Type50
        +register_file_import(file_path: Type5, namespace: Type5) Type3
        +get_imported_namespaces(file_path: Type5) Type50
        +get_namespaces_for_file(file_path: Type5) Type50
        +default() Self
    }
    class c0054["crates/mycelium-core/src/graph/scoring.rs"] {
        <<module>>
        -probe_depth(kg: Type51, sym_id: Type5, max_hops: usize) usize
        +score_entry_points(kg: Type51) Type52
    }
    class c0055["SymbolDefinition"] {
        <<struct>>
        +symbol_id: String
        +name: String
        +file: String
        +symbol_type: String
        +language: Option~String~
        +parent: Option~String~
    }
    class c0056["SymbolTable"] {
        <<struct>>
        -file_index: Type53
        -global_index: Type54
        +new() Self
        +add(symbol: Type29) Type3
        +lookup_exact(file_path: Type5, name: Type5) Type16
        +lookup_fuzzy(name: Type5) Type55
        +get_symbols_in_file(file_path: Type5) Type56
        +file_index() Type57
        +global_index() Type58
        +default() Self
    }
    class c0058["CAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0059["CppAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -extract_cpp_symbols(Signature1)
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0060["crates/mycelium-core/src/languages/c_cpp.rs"] {
        <<module>>
        -is_preproc_container(kind: Type5) bool
        -get_func_name(node: Type63, source: Type61) Option~String~
        -get_qualified_func_name(node: Type63, source: Type61) Option~String~
        -get_type_name(node: Type63, source: Type61) Option~String~
        -extract_c_symbols(Signature2)
        -extract_includes(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        -find_c_calls(Signature3)
        -extract_c_callee(node: Type63, source: Type61) Type66
        -find_enclosing_func(node: Type63, source: Type61) Option~String~
    }
    class c0061["CSharpAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -walk_node(Signature4)
        -extract_using(node: Type63, source: Type61, file_path: Type5) Option~ImportStatement~
        -find_calls(Signature5)
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0062["crates/mycelium-core/src/languages/csharp.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
        -is_container(node_type: Type5) bool
        -get_visibility(node: Type63, source: Type61) Visibility
        -get_name(node: Type63, source: Type61) Option~String~
        -extract_parameter_types(node: Type63, source: Type61) Type12
        -extract_callee(inv_node: Type63, source: Type61) Type66
        -find_enclosing_method(node: Type63, source: Type61) Option~String~
    }
    class c0063["crates/mycelium-core/src/languages/declarations/c_cpp.rs"] {
        <<module>>
        +walk(Signature6)
        -find_function(node: Type67) Type69
        -declarator_name(node: Type67, source: Type61) Option~String~
        -declaration_type(node: Type67, declarator: Type69, source: Type61) Option~String~
    }
    class c0064["crates/mycelium-core/src/languages/declarations/frameworks/attributes.rs"] {
        <<module>>
        +detect(root: Type67, source: Type61, file: Type5, detection: Type70, vb: bool) Type3
        -guard_name(guards: Type71, name: Type5) Type3
        -key(name: Type5) String
        +type_node(node: Type67) bool
        +scope(root: Type67, node: Type67, source: Type61) String
        -scope_inner(root: Type67, node: Type67, source: Type61, omit_modules: bool) String
        -ancestor(parent: Type67, node: Type67) bool
        +inherited_scope(node: Type67) bool
        -clean(name: Type5) String
        -rule(name: Type5, container: bool, vb: bool) Type72
    }
    class c0065["crates/mycelium-core/src/languages/declarations/frameworks/java.rs"] {
        <<module>>
        +detect(root: Type67, source: Type61, file: Type5, detection: Type70) Type3
    }
    class c0066["crates/mycelium-core/src/languages/declarations/frameworks/javascript.rs"] {
        <<module>>
        +detect(root: Type67, source: Type61, file: Type5, detection: Type70) Type3
        -inline(node: Type67) bool
        -literal(node: Type67, source: Type61) Option~String~
        -bound_names(node: Type67, source: Type61, names: Type71) Type3
    }
    class c0067["Bindings"] {
        <<struct>>
        -declared: BTreeSet~String~
        -guards: Type73
        +extend(other: Self) Type3
    }
    class c0068["Detection"] {
        <<struct>>
        -marks: Type74
        +bindings: Bindings
        +syntax_valid: bool
        +new(root: Type67, source: Type61, file: Type5, language: Type5) Self
        -mark(node: Type67, marker: Type67, file: Type5, rule: Type5) Type3
        +evidence(node: Type67) Option~TestEvidence~
    }
    class c0069["crates/mycelium-core/src/languages/declarations/frameworks/mod.rs"] {
        <<module>>
        -visit(node: Type75, callback: Type76) Type3
        +finish(diagram: Type77, files: Type50, bindings: Bindings) Type3
        -keyword(node: Type67, name: Type5) bool
    }
    class c0070["crates/mycelium-core/src/languages/declarations/frameworks/python.rs"] {
        <<module>>
        +detect(root: Type67, source: Type61, file: Type5, detection: Type70) Type3
        -bound_names(node: Type67, source: Type61, names: Type78) Type3
    }
    class c0071["crates/mycelium-core/src/languages/declarations/go.rs"] {
        <<module>>
        +walk(Signature6)
    }
    class c0072["crates/mycelium-core/src/languages/declarations/mod.rs"] {
        <<module>>
        +extract(Signature7)
        -text(node: Type67, source: Type61) String
        -field(node: Type67, name: Type5, source: Type61) Option~String~
        -children(node: Type67) Type80
        -add_class(diagram: Type68, file: Type5, name: String, kind: Type5, line: usize) usize
        -module(diagram: Type68, file: Type5) usize
        -rust_walk(Signature8)
        -enum_member(node: Type67, owner: usize, name: String, diagram: Type68) Type3
    }
    class c0073["crates/mycelium-core/src/languages/declarations/nominal.rs"] {
        <<module>>
        +walk(Signature9)
        -visibility(node: Type67, source: Type61, public_default: bool) String
        -add_field(Signature10)
        -type_field(node: Type67, name: Type5, source: Type61) Option~String~
        -base_types(node: Type67) Type80
    }
    class c0074["crates/mycelium-core/src/languages/declarations/python.rs"] {
        <<module>>
        +walk(Signature9)
        -push_field(Signature11)
        -instance_fields(Signature12)
    }
    class c0075["RustDetection"] {
        <<struct>>
        +syntax_valid: bool
        +uncertain_test_binding: bool
        +skipped_test: bool
        +new(root: Type67, source: Type61) Self
    }
    class c0076["crates/mycelium-core/src/languages/declarations/test_detection.rs"] {
        <<module>>
        +malformed(node: Type67) bool
        -uncertain_import(node: Type67, source: Type61) bool
        -binds_test(node: Type67, source: Type61) bool
        +rust_evidence(Signature13)
        -evidence(rule: Type5, node: Type67, file: Type5) TestEvidence
        -tokens(node: Type67, source: Type61) String
    }
    class c0077["crates/mycelium-core/src/languages/declarations/vbnet.rs"] {
        <<module>>
        +walk(Signature9)
        -vb_type(node: Type67, source: Type61) Option~String~
    }
    class c0078["GoAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_name_by_kind(node: Type63, target_kind: Type5, source: Type61) Option~String~
        -is_exported(name: Type5) bool
        -extract_string(node: Type63, source: Type61) Option~String~
        -extract_string_content(node: Type63, source: Type61) Option~String~
        -find_calls(Signature5)
        -extract_callee(node: Type63, source: Type61) Type66
        -find_enclosing(node: Type63, source: Type61) Option~String~
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0079["JavaAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -walk_node(Signature4)
        -find_calls(Signature5)
        -extract_callee(node: Type63, source: Type61) Type66
        -find_enclosing(node: Type63, source: Type61) Option~String~
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0080["crates/mycelium-core/src/languages/java.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
        -is_container(node_type: Type5) bool
        -get_visibility(node: Type63, source: Type61) Visibility
        -get_name(node: Type63, source: Type61) Option~String~
    }
    class c0081["AnalyserRegistry"] {
        <<struct>>
        -analysers: Vec~Box~dyn LanguageAnalyser~~
        -extension_map: Type84
        +new() Self
        +get_by_extension(ext: Type5) Type85
        +language_for_extension(ext: Type5) Type16
        +extensions() Type86
        +default() Self
    }
    class c0082["LanguageAnalyser"] {
        <<trait>>
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
        +get_language_for_ext(_ext: Type5) Language
        +is_available() bool
    }
    class c0083["PythonAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_name(node: Type63, source: Type61) Option~String~
        -walk_node(Signature4)
        -find_calls(Signature5)
        -extract_callee(node: Type63, source: Type61) Type66
        -find_enclosing(node: Type63, source: Type61) Option~String~
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0084["RustAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_name(node: Type63, source: Type61) Option~String~
        -is_pub(node: Type63) bool
        -walk_node(Signature4)
        -find_calls(Signature5)
        -extract_callee(node: Type63, source: Type61) Type66
        -find_enclosing(node: Type63, source: Type61) Option~String~
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0085["crates/mycelium-core/src/languages/rust_lang.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
    }
    class c0086["TypeScriptAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_ts_language() Language
        -get_tsx_language() Language
        -get_js_language() Language
        -language_for_path(file_path: Type5) Type13
        -get_name(node: Type63, source: Type61) Option~String~
        -walk_node(Signature4)
        -extract_class_members(Signature14)
        -extract_string_source(node: Type63, source: Type61) Option~String~
        -find_calls(Signature5)
        -extract_callee(node: Type63, source: Type61) Type66
        -find_enclosing(node: Type63, source: Type61) Option~String~
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +get_language_for_ext(ext: Type5) Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
    }
    class c0087["crates/mycelium-core/src/languages/typescript.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
    }
    class c0088["VbNetAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -walk_node(Signature4)
        -find_calls(Signature5)
        +extensions() Type59
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type60, source: Type61, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type60, source: Type61, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type60, source: Type61, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type62
        +is_available() bool
    }
    class c0089["crates/mycelium-core/src/languages/vbnet.rs"] {
        <<module>>
        -tree_sitter_vb_dotnet() Type87
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
        -is_container(node_type: Type5) bool
        -get_visibility(node: Type63, source: Type61) Visibility
        -get_name(node: Type63, source: Type61) Option~String~
        -extract_callee(node: Type63, source: Type61) Type66
        -find_enclosing_method(node: Type63, source: Type61) Option~String~
    }
    class c0090["CallEndpoint"] {
        <<struct>>
        -owner: Type88
        -member: Type89
        -name() Type90
        -location() Type91
    }
    class c0091["ExportError"] {
        <<enum>>
        -MissingDeclarations: unknown
        -InvalidOptions: unknown
        -NoMatchingPath: unknown
        -NoMatchingTestPath: Type92
        -InvalidTestMode: unknown
        +fmt(f: Type93) fmt::Result
    }
    class c0092["MermaidExport"] {
        <<struct>>
        +markdown: String
        +notices: Vec~String~
    }
    class c0093["MermaidOptions"] {
        <<struct>>
        +path: String
        +max_classes: usize
        +tests: TestMode
        +explain_tests: bool
        +test_paths: Vec~String~
        +keep_paths: Vec~String~
        +default() Self
    }
    class c0094["TestMode"] {
        <<enum>>
        -Exclude: unknown
        -Include: unknown
        +from_str(value: Type5) Type94
    }
    class c0095["crates/mycelium-core/src/mermaid.rs"] {
        <<module>>
        +export_mermaid(result: Type96, options: Type97) Type95
        +export_mermaid_report(result: Type96, options: Type97) Type98
        -type_edges(Signature15)
        -unique_occurrences(classes: Type99) Vec~Class~
        -merge_classes(raw: Type99, identities: Type99, warnings: Type71) Vec~Class~
        -name_index(classes: Type99) Type101
        -resolve(classes: Type99, index: Type102, file: Type5, name: Type5) Option~usize~
        -member_text(member: Type103, aliases: Type104) String
        -type_text(value: Type5, aliases: Type104) String
        -safe(value: Type5) String
        -pages(classes: Type99, maximum: usize) Type105
        -abbreviation(value: Type5, prefix: Type5, limit: usize, key: Type104) String
        -wrap_prose(markdown: Type5) String
        -visibility(value: Type5) Type5
        -type_names(value: Type5, language: Type5) Vec~String~
        -language_family(file: Type5) Type5
        -ordered_key(key: Type106) Type18
        -markdown_code(value: Type5) String
        -plain_member(member: Type103) String
    }
    class c0096["TestFilter"] {
        <<struct>>
        -test_paths: Vec~String~
        -keep_paths: Vec~String~
        -files: BTreeSet~String~
        -include: bool
        -detector_version: Option~u32~
        -explain: bool
        +notices: BTreeSet~String~
        -counts: Type107
        -explanations: BTreeSet~String~
        +new(result: Type96, options: Type97) Type108
        -hidden(file: Type5, line: usize, name: Type5, evidence: Type109, in_scope: bool) bool
        -valid_location(file: Type5, line: usize) bool
        +excludes_file(file: Type5) bool
        +retain(raw: Type99, in_scope: Type110) Vec~Class~
        +summary(filtered_calls: usize, filtered_edges: usize) String
    }
    class c0097["crates/mycelium-core/src/mermaid/filtering.rs"] {
        <<module>>
        +normalize_path(path: Type5) Type95
        +matches_path(file: Type5, prefix: Type5) bool
    }
    class c0098["crates/mycelium-core/src/output.rs"] {
        <<module>>
        -get_commit_hash(repo_path: Type5) Option~String~
        -count_languages(kg: Type51) Type84
        +build_result(Signature16)
        +write_output(result: Type96, output_path: Type5) Type113
    }
    class c0099["crates/mycelium-core/src/phases/calls.rs"] {
        <<module>>
        +run_calls_phase(config: Type4, kg: Type114, st: Type115, _ns_index: Type116) Type3
        -is_call_target(source_id: Type5, target_id: Type5, kg: Type51) bool
        -call_target_in_file(Signature17)
        -build_import_map(kg: Type51) Type49
        -build_field_type_map(file_path: Type5, kg: Type51) Type15
        -is_interface_self_call(Signature18)
        -is_interface_method(target_id: Type5, kg: Type51) bool
        -find_implementation(Signature19)
        -resolve_call(Signature20)
    }
    class c0100["AdjList"] {
        <<struct>>
        -node_map: Type84
        -nodes: Vec~String~
        -adj: Type121
        -new() Self
        -ensure_node(id: Type5) usize
        -add_edge(a: Type5, b: Type5, weight: f64) Type3
        -total_weight() f64
    }
    class c0101["crates/mycelium-core/src/phases/communities.rs"] {
        <<module>>
        +run_communities_phase(config: Type4, kg: Type114) Type3
        -louvain(adj: Type122, resolution: f64) Vec~Vec~String~~
        -split_oversized(community: Type50, adj: Type122, max_size: usize) Vec~Vec~String~~
        -generate_label(members: Type50, kg: Type51) String
        -disambiguate_label(Signature21)
        -compute_cohesion(members: Type50, adj: Type122) f64
        -primary_language(members: Type50, kg: Type51) String
        -common_prefix(strings: Type50) String
    }
    class c0102["crates/mycelium-core/src/phases/imports.rs"] {
        <<module>>
        +run_imports_phase(config: Type4, kg: Type114, st: Type115, ns_index: Type116) Type3
        -process_dotnet_projects(config: Type4, kg: Type114, assembly_index: Type123) Type3
        -register_observed_namespaces(kg: Type51, _assembly_index: Type124) Type3
        -process_source_imports(Signature22)
        -resolve_python_import(Signature23)
        -resolve_python_relative(Signature24)
        -resolve_ts_import(Signature25)
        -resolve_java_import(Signature26)
        -parse_go_mod(file_set: Type62, repo_root: Type5) Option~String~
        -build_go_dir_index(file_set: Type62) Type49
        -resolve_go_import(Signature27)
        -resolve_rust_import(Signature28)
        -resolve_c_include(Signature29)
        -resolve_fallback(Signature30)
        -normalize_path(path: Type5) String
    }
    class c0103["crates/mycelium-core/src/phases/parsing.rs"] {
        <<module>>
        +run_parsing_phase(config: Type4, kg: Type114, st: Type115, ns_index: Type116) Type3
    }
    class c0104["crates/mycelium-core/src/phases/processes.rs"] {
        <<module>>
        +run_processes_phase(config: Type4, kg: Type114) Type3
        -bfs_traces(Signature31)
        -deduplicate(traces: Vec~Vec~String~~) Vec~Vec~String~~
        -build_community_map(kg: Type51) Type15
        -classify_process(trace: Type50, community_map: Type17) String
        -compute_total_confidence(kg: Type51, trace: Type50) f64
        -sort_key(trace: Type50, total_conf: f64) Type125
    }
    class c0105["crates/mycelium-core/src/phases/structure.rs"] {
        <<module>>
        +run_structure_phase(config: Type4, kg: Type114) Type3
    }
    class c0106["crates/mycelium-core/src/pipeline.rs"] {
        <<module>>
        +run_pipeline(config: Type4, progress_callback: Option~ProgressCallback~) Type126
    }
    class c0122["PyAnalysisConfig"] {
        <<struct>>
        -repo_path: String
        -output_path: Option~String~
        -languages: Option~Vec~String~~
        -resolution: f64
        -max_processes: usize
        -max_depth: usize
        -max_branching: usize
        -min_steps: usize
        -exclude_patterns: Vec~String~
        -verbose: bool
        -quiet: bool
        -max_file_size: u64
        -max_community_size: usize
        -new(Signature32)
    }
    class c0123["crates/mycelium-python/src/lib.rs"] {
        <<module>>
        -analyze(Signature33)
        -export_mermaid(Signature34)
        -version() Type13
        -_mycelium_rust(m: Type130) Type129
    }
    class c0124["mycelium/cli.py"] {
        <<module>>
        +cli() None
        -_run_with_progress(config: PyAnalysisConfig) unknown
        -_run_quiet(config: PyAnalysisConfig) unknown
        +export_cmd(Signature35)
        +analyze_cmd(Signature36)
    }
    class c0254["vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs"] {
        <<module>>
        -main() Type3
    }
    class c0255["vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs"] {
        <<module>>
        -tree_sitter_tree_sitter_vb_dotnet() Type87
    }
    class c0256["vendor/tree-sitter-vb-dotnet/grammar.js"] {
        <<module>>
        commaSep(rule: unknown) unknown
        kw(word: unknown) unknown
        commaSep1(rule: unknown) unknown
        ci(keyword: unknown) unknown
    }
    class c0257["BdistWheel"] {
        <<class>>
        +get_tag() unknown
    }
    class c0258["Build"] {
        <<class>>
        +run() unknown
    }
    class c0259["EggInfo"] {
        <<class>>
        +find_sources() unknown
    }
    class c0260["vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h"] {
        <<module>>
        _array__erase(Signature37)
        _array__reserve(Signature38)
        _array__assign(Signature39)
        _array__swap(Signature40)
        _array__grow(Signature41)
        _array__splice(Signature42)
    }
    class c0261["TSCharacterRange"] {
        <<struct>>
        start: int32_t
        end: int32_t
    }
    class c0262["TSFieldMapEntry"] {
        <<struct>>
        field_id: TSFieldId
        child_index: uint8_t
        inherited: bool
    }
    class c0263["TSLanguage"] {
        <<struct>>
        abi_version: uint32_t
        symbol_count: uint32_t
        alias_count: uint32_t
        token_count: uint32_t
        external_token_count: uint32_t
        state_count: uint32_t
        large_state_count: uint32_t
        production_id_count: uint32_t
        field_count: uint32_t
        max_alias_sequence_length: uint16_t
        parse_table: Type136
        small_parse_table: Type136
        small_parse_table_map: Type137
        parse_actions: Type138
        symbol_names: Type139
        field_names: Type139
        field_map_slices: Type140
        field_map_entries: Type141
        symbol_metadata: Type142
        public_symbol_map: Type143
        alias_map: Type136
        alias_sequences: Type143
        lex_modes: Type144
        keyword_capture_token: TSSymbol
        external_scanner: Type145
        states: Type146
        symbol_map: Type143
        primary_state_ids: Type147
        name: Type148
        reserved_words: Type143
        max_reserved_word_set_size: uint16_t
        supertype_count: uint32_t
        supertype_symbols: Type143
        supertype_map_slices: Type140
        supertype_map_entries: Type143
        metadata: TSLanguageMetadata
    }
    class c0264["TSLanguageMetadata"] {
        <<struct>>
        major_version: uint8_t
        minor_version: uint8_t
        patch_version: uint8_t
    }
    class c0265["TSLexMode"] {
        <<struct>>
        lex_state: uint16_t
        external_lex_state: uint16_t
    }
    class c0266["TSLexer"] {
        <<struct>>
        lookahead: int32_t
        result_symbol: TSSymbol
    }
    class c0267["TSLexerMode"] {
        <<struct>>
        lex_state: uint16_t
        external_lex_state: uint16_t
        reserved_word_set_id: uint16_t
    }
    class c0268["TSMapSlice"] {
        <<struct>>
        index: uint16_t
        length: uint16_t
    }
    class c0269["TSParseAction"] {
        <<union>>
        shift: Type149
        type: uint8_t
        state: TSStateId
        extra: bool
        repetition: bool
        reduce: Type150
        child_count: uint8_t
        symbol: TSSymbol
        dynamic_precedence: int16_t
        production_id: uint16_t
    }
    class c0270["TSParseActionEntry"] {
        <<union>>
        action: TSParseAction
        entry: Type151
        count: uint8_t
        reusable: bool
    }
    class c0271["TSParseActionType"] {
        <<enum>>
        TSParseActionTypeShift: TSParseActionType
        TSParseActionTypeReduce: TSParseActionType
        TSParseActionTypeAccept: TSParseActionType
        TSParseActionTypeRecover: TSParseActionType
    }
    class c0272["TSSymbolMetadata"] {
        <<struct>>
        visible: bool
        named: bool
        supertype: bool
    }
    class c0273["vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h"] {
        <<module>>
        set_contains(ranges: Type152, len: uint32_t, lookahead: int32_t) bool
    }
    c0000 --> c0001 : field command
    c0002 ..> c0002 : 3 relationships (see list)
    c0002 ..> c0004 : 2 relationships (see list)
    c0002 ..> c0040 : run_export() calls new()
    c0002 ..> c0095 : run_export() calls export_mermaid_report()
    c0002 ..> c0098 : 2 relationships (see list)
    c0002 ..> c0106 : 2 relationships (see list)
    c0004 ..> c0122 : type in from
    c0005 --> c0007 : field calls
    c0005 --> c0009 : field communities
    c0005 --> c0017 : field imports
    c0005 --> c0021 : field processes
    c0005 --> c0025 : field structure
    c0005 --> c0027 : field symbols
    c0005 ..> c0030 : 8 relationships (see list)
    c0005 ..> c0036 : default() calls default()
    c0017 --> c0015 : field file_imports
    c0017 --> c0018 : field package_references
    c0017 --> c0022 : field project_references
    c0025 --> c0011 : field files
    c0025 --> c0013 : field folders
    c0026 --> c0028 : field symbol_type
    c0026 --> c0029 : field visibility
    c0029 ..> c0029 : fmt() calls as_str()
    c0032 --> c0031 : field bases
    c0032 --> c0034 : field members
    c0032 --> c0038 : field test
    c0033 --> c0032 : field classes
    c0033 --> c0036 : field test_detection
    c0034 --> c0035 : field parameters
    c0034 --> c0038 : field test
    c0036 --> c0037 : field diagnostics
    c0040 ..> c0028 : resolve_namespace() calls as_str()
    c0040 ..> c0040 : default() calls new()
    c0043 ..> c0028 : extract_attr() calls as_str()
    c0043 ..> c0040 : parse_project_file() calls new()
    c0043 ..> c0042 : type in parse_project_file
    c0043 ..> c0043 : 6 relationships (see list)
    c0045 ..> c0044 : type in parse_solution
    c0048 ..> c0006 : type in add_call
    c0048 ..> c0008 : type in add_community
    c0048 ..> c0010 : type in add_file
    c0048 ..> c0012 : type in add_folder
    c0048 ..> c0014 : type in add_import
    c0048 ..> c0019 : type in add_package_reference
    c0048 ..> c0020 : type in add_process
    c0048 ..> c0023 : type in add_project_reference
    c0048 ..> c0026 : type in add_symbol
    c0048 ..> c0028 : add_symbol() calls as_str()
    c0048 ..> c0040 : new() calls new()
    c0048 ..> c0046 : 2 relationships (see list)
    c0048 --> c0047 : field graph
    c0048 ..> c0047 : type in inner_graph
    c0048 ..> c0048 : 17 relationships (see list)
    c0048 --> c0049 : field graph
    c0048 ..> c0049 : 5 relationships (see list)
    c0048 ..> c0050 : 2 relationships (see list)
    c0048 ..> c0100 : 7 relationships (see list)
    c0052 ..> c0052 : default() calls new()
    c0054 ..> c0028 : score_entry_points() calls as_str()
    c0054 ..> c0048 : 6 relationships (see list)
    c0054 ..> c0054 : score_entry_points() calls probe_depth()
    c0056 ..> c0026 : type in add
    c0056 ..> c0028 : 2 relationships (see list)
    c0056 --> c0055 : field global_index
    c0056 ..> c0055 : 2 relationships (see list)
    c0056 ..> c0056 : default() calls new()
    c0058 ..> c0016 : type in extract_imports
    c0058 ..> c0024 : type in extract_calls
    c0058 ..> c0026 : type in extract_symbols
    c0058 ..|> c0082 : implements
    c0059 ..> c0016 : type in extract_imports
    c0059 ..> c0024 : type in extract_calls
    c0059 ..> c0026 : 2 relationships (see list)
    c0059 ..> c0059 : 2 relationships (see list)
    c0059 ..> c0060 : 5 relationships (see list)
    c0059 ..|> c0082 : implements
    c0060 ..> c0016 : type in extract_includes
    c0060 ..> c0024 : type in find_c_calls
    c0060 ..> c0026 : type in extract_c_symbols
    c0060 ..> c0060 : 8 relationships (see list)
    c0061 ..> c0016 : 2 relationships (see list)
    c0061 ..> c0024 : 2 relationships (see list)
    c0061 ..> c0026 : 2 relationships (see list)
    c0061 ..> c0061 : 4 relationships (see list)
    c0061 ..> c0062 : 7 relationships (see list)
    c0061 ..> c0078 : find_calls() calls find_calls()
    c0061 ..> c0079 : walk_node() calls walk_node()
    c0061 ..|> c0082 : implements
    c0062 ..> c0028 : 2 relationships (see list)
    c0062 ..> c0029 : type in get_visibility
    c0063 ..> c0033 : type in walk
    c0063 ..> c0063 : 3 relationships (see list)
    c0063 ..> c0071 : walk() calls walk()
    c0063 ..> c0072 : 10 relationships (see list)
    c0064 ..> c0028 : detect() calls as_str()
    c0064 ..> c0064 : 12 relationships (see list)
    c0064 ..> c0068 : 2 relationships (see list)
    c0064 ..> c0069 : 2 relationships (see list)
    c0064 ..> c0072 : 6 relationships (see list)
    c0065 ..> c0004 : detect() calls from()
    c0065 ..> c0028 : detect() calls as_str()
    c0065 ..> c0064 : 3 relationships (see list)
    c0065 ..> c0068 : 2 relationships (see list)
    c0065 ..> c0069 : detect() calls visit()
    c0065 ..> c0072 : 3 relationships (see list)
    c0066 ..> c0066 : 3 relationships (see list)
    c0066 ..> c0068 : 2 relationships (see list)
    c0066 ..> c0069 : 2 relationships (see list)
    c0066 ..> c0070 : bound_names() calls bound_names()
    c0066 ..> c0072 : 5 relationships (see list)
    c0066 ..> c0096 : detect() calls retain()
    c0067 --> c0038 : field guards
    c0068 ..> c0004 : new() calls default()
    c0068 --> c0038 : field marks
    c0068 ..> c0038 : type in evidence
    c0068 ..> c0064 : 2 relationships (see list)
    c0068 --> c0067 : field bindings
    c0068 ..> c0076 : new() calls malformed()
    c0069 ..> c0063 : keyword() calls walk()
    c0069 ..> c0067 : 2 relationships (see list)
    c0069 ..> c0068 : finish() calls new()
    c0069 ..> c0072 : 2 relationships (see list)
    c0070 ..> c0040 : detect() calls new()
    c0070 ..> c0063 : detect() calls walk()
    c0070 ..> c0066 : bound_names() calls bound_names()
    c0070 ..> c0068 : 2 relationships (see list)
    c0070 ..> c0069 : detect() calls visit()
    c0070 ..> c0070 : detect() calls bound_names()
    c0070 ..> c0072 : 5 relationships (see list)
    c0070 ..> c0095 : detect() calls resolve()
    c0070 ..> c0096 : detect() calls retain()
    c0071 ..> c0033 : type in walk
    c0071 ..> c0063 : walk() calls walk()
    c0071 ..> c0072 : 5 relationships (see list)
    c0072 ..> c0004 : extract() calls default()
    c0072 ..> c0033 : 5 relationships (see list)
    c0072 ..> c0040 : extract() calls new()
    c0072 ..> c0063 : 3 relationships (see list)
    c0072 ..> c0067 : extract() calls extend()
    c0072 ..> c0072 : 8 relationships (see list)
    c0072 ..> c0076 : rust_walk() calls rust_evidence()
    c0073 ..> c0033 : 2 relationships (see list)
    c0073 ..> c0063 : walk() calls walk()
    c0073 ..> c0068 : 2 relationships (see list)
    c0073 ..> c0072 : 10 relationships (see list)
    c0073 ..> c0073 : 6 relationships (see list)
    c0074 ..> c0033 : 3 relationships (see list)
    c0074 ..> c0063 : walk() calls walk()
    c0074 ..> c0068 : 2 relationships (see list)
    c0074 ..> c0072 : 8 relationships (see list)
    c0074 ..> c0074 : 3 relationships (see list)
    c0075 ..> c0076 : 2 relationships (see list)
    c0076 ..> c0038 : 2 relationships (see list)
    c0076 ..> c0063 : 5 relationships (see list)
    c0076 ..> c0072 : 2 relationships (see list)
    c0076 ..> c0075 : type in rust_evidence
    c0076 ..> c0076 : 3 relationships (see list)
    c0077 ..> c0033 : type in walk
    c0077 ..> c0063 : walk() calls walk()
    c0077 ..> c0068 : walk() calls evidence()
    c0077 ..> c0072 : 9 relationships (see list)
    c0077 ..> c0077 : walk() calls vb_type()
    c0078 ..> c0016 : type in extract_imports
    c0078 ..> c0024 : 2 relationships (see list)
    c0078 ..> c0026 : type in extract_symbols
    c0078 ..> c0061 : find_calls() calls find_calls()
    c0078 ..> c0078 : 10 relationships (see list)
    c0078 ..|> c0082 : implements
    c0079 ..> c0016 : type in extract_imports
    c0079 ..> c0024 : 2 relationships (see list)
    c0079 ..> c0026 : 2 relationships (see list)
    c0079 ..> c0061 : 2 relationships (see list)
    c0079 ..> c0079 : 5 relationships (see list)
    c0079 ..> c0080 : 5 relationships (see list)
    c0079 ..|> c0082 : implements
    c0080 ..> c0028 : 2 relationships (see list)
    c0080 ..> c0029 : type in get_visibility
    c0081 ..> c0028 : extensions() calls as_str()
    c0081 ..> c0058 : language_for_extension() calls language_name()
    c0081 ..> c0081 : 3 relationships (see list)
    c0081 --> c0082 : field analysers
    c0081 ..> c0082 : type in get_by_extension
    c0081 ..> c0088 : new() calls is_available()
    c0082 ..> c0016 : type in extract_imports
    c0082 ..> c0024 : type in extract_calls
    c0082 ..> c0026 : type in extract_symbols
    c0083 ..> c0016 : type in extract_imports
    c0083 ..> c0024 : 2 relationships (see list)
    c0083 ..> c0026 : 2 relationships (see list)
    c0083 ..> c0061 : 2 relationships (see list)
    c0083 ..|> c0082 : implements
    c0083 ..> c0083 : 6 relationships (see list)
    c0084 ..> c0016 : type in extract_imports
    c0084 ..> c0024 : 2 relationships (see list)
    c0084 ..> c0026 : 2 relationships (see list)
    c0084 ..> c0061 : 2 relationships (see list)
    c0084 ..|> c0082 : implements
    c0084 ..> c0084 : 8 relationships (see list)
    c0084 ..> c0085 : walk_node() calls node_to_symbol_type()
    c0085 ..> c0028 : type in node_to_symbol_type
    c0086 ..> c0016 : type in extract_imports
    c0086 ..> c0024 : 2 relationships (see list)
    c0086 ..> c0026 : 3 relationships (see list)
    c0086 ..> c0061 : find_calls() calls find_calls()
    c0086 ..|> c0082 : implements
    c0086 ..> c0086 : 13 relationships (see list)
    c0086 ..> c0087 : walk_node() calls node_to_symbol_type()
    c0087 ..> c0028 : type in node_to_symbol_type
    c0088 ..> c0016 : type in extract_imports
    c0088 ..> c0024 : 2 relationships (see list)
    c0088 ..> c0026 : 2 relationships (see list)
    c0088 ..> c0061 : 2 relationships (see list)
    c0088 ..|> c0082 : implements
    c0088 ..> c0088 : 3 relationships (see list)
    c0088 ..> c0089 : 6 relationships (see list)
    c0089 ..> c0028 : type in node_to_symbol_type
    c0089 ..> c0029 : type in get_visibility
    c0089 ..> c0089 : find_enclosing_method() calls get_name()
    c0090 --> c0032 : field owner
    c0090 --> c0034 : field member
    c0093 --> c0094 : field tests
    c0095 ..> c0005 : 2 relationships (see list)
    c0095 ..> c0028 : 4 relationships (see list)
    c0095 ..> c0032 : 6 relationships (see list)
    c0095 ..> c0034 : 3 relationships (see list)
    c0095 ..> c0040 : 4 relationships (see list)
    c0095 ..> c0067 : 2 relationships (see list)
    c0095 ..> c0081 : language_family() calls language_for_extension()
    c0095 ..> c0091 : 2 relationships (see list)
    c0095 ..> c0092 : type in export_mermaid_report
    c0095 ..> c0093 : 2 relationships (see list)
    c0095 ..> c0095 : 23 relationships (see list)
    c0095 ..> c0096 : 3 relationships (see list)
    c0095 ..> c0097 : 2 relationships (see list)
    c0096 ..> c0005 : type in new
    c0096 ..> c0028 : hidden() calls as_str()
    c0096 ..> c0032 : type in retain
    c0096 ..> c0038 : type in hidden
    c0096 ..> c0039 : hidden() calls supported_test_rule()
    c0096 ..> c0067 : new() calls extend()
    c0096 ..> c0091 : type in new
    c0096 ..> c0093 : type in new
    c0096 ..> c0096 : 3 relationships (see list)
    c0096 ..> c0097 : 5 relationships (see list)
    c0097 ..> c0091 : type in normalize_path
    c0098 ..> c0004 : type in build_result
    c0098 ..> c0005 : 2 relationships (see list)
    c0098 ..> c0040 : 3 relationships (see list)
    c0098 ..> c0048 : 12 relationships (see list)
    c0098 ..> c0056 : type in build_result
    c0098 ..> c0098 : 2 relationships (see list)
    c0099 ..> c0004 : type in run_calls_phase
    c0099 ..> c0006 : type in resolve_call
    c0099 ..> c0028 : 3 relationships (see list)
    c0099 ..> c0040 : run_calls_phase() calls new()
    c0099 ..> c0048 : 17 relationships (see list)
    c0099 ..> c0052 : type in run_calls_phase
    c0099 ..> c0056 : 9 relationships (see list)
    c0099 ..> c0058 : run_calls_phase() calls extract_calls()
    c0099 ..> c0081 : run_calls_phase() calls get_by_extension()
    c0099 ..> c0086 : run_calls_phase() calls get_language_for_ext()
    c0099 ..> c0088 : run_calls_phase() calls is_available()
    c0099 ..> c0099 : 12 relationships (see list)
    c0100 ..> c0100 : add_edge() calls ensure_node()
    c0101 ..> c0004 : type in run_communities_phase
    c0101 ..> c0028 : 5 relationships (see list)
    c0101 ..> c0048 : 9 relationships (see list)
    c0101 ..> c0067 : 2 relationships (see list)
    c0101 ..> c0100 : 10 relationships (see list)
    c0101 ..> c0101 : 8 relationships (see list)
    c0102 ..> c0004 : 3 relationships (see list)
    c0102 ..> c0028 : process_source_imports() calls as_str()
    c0102 ..> c0040 : 16 relationships (see list)
    c0102 ..> c0043 : process_dotnet_projects() calls parse_project_file()
    c0102 ..> c0045 : process_dotnet_projects() calls parse_solution()
    c0102 ..> c0048 : 13 relationships (see list)
    c0102 ..> c0052 : 4 relationships (see list)
    c0102 ..> c0056 : 4 relationships (see list)
    c0102 ..> c0058 : process_source_imports() calls extract_imports()
    c0102 ..> c0081 : process_source_imports() calls get_by_extension()
    c0102 ..> c0086 : process_source_imports() calls get_language_for_ext()
    c0102 ..> c0088 : process_source_imports() calls is_available()
    c0102 ..> c0102 : 16 relationships (see list)
    c0103 ..> c0004 : 2 relationships (see list)
    c0103 ..> c0028 : run_parsing_phase() calls as_str()
    c0103 ..> c0040 : 2 relationships (see list)
    c0103 ..> c0048 : 3 relationships (see list)
    c0103 ..> c0052 : type in run_parsing_phase
    c0103 ..> c0056 : 2 relationships (see list)
    c0103 ..> c0058 : 2 relationships (see list)
    c0103 ..> c0067 : run_parsing_phase() calls extend()
    c0103 ..> c0069 : run_parsing_phase() calls finish()
    c0103 ..> c0072 : run_parsing_phase() calls extract()
    c0103 ..> c0081 : run_parsing_phase() calls get_by_extension()
    c0103 ..> c0086 : run_parsing_phase() calls get_language_for_ext()
    c0104 ..> c0004 : type in run_processes_phase
    c0104 ..> c0028 : 2 relationships (see list)
    c0104 ..> c0040 : bfs_traces() calls new()
    c0104 ..> c0048 : 8 relationships (see list)
    c0104 ..> c0054 : run_processes_phase() calls score_entry_points()
    c0104 ..> c0067 : run_processes_phase() calls extend()
    c0104 ..> c0104 : 6 relationships (see list)
    c0105 ..> c0004 : type in run_structure_phase
    c0105 ..> c0028 : run_structure_phase() calls as_str()
    c0105 ..> c0040 : run_structure_phase() calls new()
    c0105 ..> c0048 : 3 relationships (see list)
    c0105 ..> c0081 : run_structure_phase() calls language_for_extension()
    c0106 ..> c0004 : type in run_pipeline
    c0106 ..> c0005 : type in run_pipeline
    c0106 ..> c0040 : run_pipeline() calls new()
    c0106 ..> c0098 : run_pipeline() calls build_result()
    c0123 ..> c0072 : export_mermaid() calls extract()
    c0123 ..> c0094 : export_mermaid() calls from_str()
    c0123 ..> c0095 : export_mermaid() calls export_mermaid_report()
    c0123 ..> c0106 : analyze() calls run_pipeline()
    c0123 ..> c0122 : type in analyze
    c0124 ..> c0124 : 2 relationships (see list)
    c0254 ..> c0040 : main() calls new()
    c0256 ..> c0256 : 2 relationships (see list)
    c0263 --> c0262 : field field_map_entries
    c0263 --> c0264 : field metadata
    c0263 --> c0266 : field external_scanner
    c0263 --> c0267 : field lex_modes
    c0263 --> c0268 : 2 relationships (see list)
    c0263 --> c0270 : field parse_actions
    c0263 --> c0272 : field symbol_metadata
    c0270 --> c0269 : field action
    c0273 ..> c0261 : type in set_contains
```

## Source index

- c0000: `Cli` — `crates/mycelium-cli/src/main.rs`:19
- c0001: `Commands` — `crates/mycelium-cli/src/main.rs`:25
- c0002: `crates/mycelium-cli/src/main.rs` — `crates/mycelium-cli/src/main.rs`:1
- c0004: `AnalysisConfig` — `crates/mycelium-core/src/config.rs`:260
- c0005: `AnalysisResult` — `crates/mycelium-core/src/config.rs`:331
- c0006: `CallEdge` — `crates/mycelium-core/src/config.rs`:187
- c0007: `CallOutput` — `crates/mycelium-core/src/config.rs`:448
- c0008: `Community` — `crates/mycelium-core/src/config.rs`:228
- c0009: `CommunityOutput` — `crates/mycelium-core/src/config.rs`:459
- c0010: `FileNode` — `crates/mycelium-core/src/config.rs`:132
- c0011: `FileOutput` — `crates/mycelium-core/src/config.rs`:385
- c0012: `FolderNode` — `crates/mycelium-core/src/config.rs`:141
- c0013: `FolderOutput` — `crates/mycelium-core/src/config.rs`:393
- c0014: `ImportEdge` — `crates/mycelium-core/src/config.rs`:198
- c0015: `ImportOutput` — `crates/mycelium-core/src/config.rs`:425
- c0016: `ImportStatement` — `crates/mycelium-core/src/config.rs`:168
- c0017: `ImportsOutput` — `crates/mycelium-core/src/config.rs`:415
- c0018: `PackageRefOutput` — `crates/mycelium-core/src/config.rs`:440
- c0019: `PackageReference` — `crates/mycelium-core/src/config.rs`:219
- c0020: `Process` — `crates/mycelium-core/src/config.rs`:241
- c0021: `ProcessOutput` — `crates/mycelium-core/src/config.rs`:469
- c0022: `ProjectRefOutput` — `crates/mycelium-core/src/config.rs`:432
- c0023: `ProjectReference` — `crates/mycelium-core/src/config.rs`:206
- c0024: `RawCall` — `crates/mycelium-core/src/config.rs`:177
- c0025: `StructureOutput` — `crates/mycelium-core/src/config.rs`:377
- c0026: `Symbol` — `crates/mycelium-core/src/config.rs`:148
- c0027: `SymbolOutput` — `crates/mycelium-core/src/config.rs`:400
- c0028: `SymbolType` — `crates/mycelium-core/src/config.rs`:8
- c0029: `Visibility` — `crates/mycelium-core/src/config.rs`:101
- c0030: `crates/mycelium-core/src/config.rs` — `crates/mycelium-core/src/config.rs`:1
- c0031: `Base` — `crates/mycelium-core/src/declarations.rs`:31
- c0032: `Class` — `crates/mycelium-core/src/declarations.rs`:16
- c0033: `ClassDiagram` — `crates/mycelium-core/src/declarations.rs`:8
- c0034: `Member` — `crates/mycelium-core/src/declarations.rs`:38
- c0035: `Parameter` — `crates/mycelium-core/src/declarations.rs`:55
- c0036: `TestDetection` — `crates/mycelium-core/src/declarations.rs`:62
- c0037: `TestDiagnostic` — `crates/mycelium-core/src/declarations.rs`:86
- c0038: `TestEvidence` — `crates/mycelium-core/src/declarations.rs`:77
- c0039: `crates/mycelium-core/src/declarations.rs` — `crates/mycelium-core/src/declarations.rs`:1
- c0040: `AssemblyIndex` — `crates/mycelium-core/src/dotnet/assembly.rs`:9
- c0042: `ProjectFile` — `crates/mycelium-core/src/dotnet/project.rs`:7
- c0043: `crates/mycelium-core/src/dotnet/project.rs` —
  `crates/mycelium-core/src/dotnet/project.rs`:1
- c0044: `SlnProject` — `crates/mycelium-core/src/dotnet/solution.rs`:8
- c0045: `crates/mycelium-core/src/dotnet/solution.rs` —
  `crates/mycelium-core/src/dotnet/solution.rs`:1
- c0046: `CallInfo` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:138
- c0047: `EdgeData` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:74
- c0048: `KnowledgeGraph` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:114
- c0049: `NodeData` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:14
- c0050: `SymbolInfo` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:123
- c0052: `NamespaceIndex` — `crates/mycelium-core/src/graph/namespace_index.rs`:6
- c0054: `crates/mycelium-core/src/graph/scoring.rs` — `crates/mycelium-core/src/graph/scoring.rs`:1
- c0055: `SymbolDefinition` — `crates/mycelium-core/src/graph/symbol_table.rs`:9
- c0056: `SymbolTable` — `crates/mycelium-core/src/graph/symbol_table.rs`:22
- c0058: `CAnalyser` — `crates/mycelium-core/src/languages/c_cpp.rs`:366
- c0059: `CppAnalyser` — `crates/mycelium-core/src/languages/c_cpp.rs`:425
- c0060: `crates/mycelium-core/src/languages/c_cpp.rs` —
  `crates/mycelium-core/src/languages/c_cpp.rs`:1
- c0061: `CSharpAnalyser` — `crates/mycelium-core/src/languages/csharp.rs`:195
- c0062: `crates/mycelium-core/src/languages/csharp.rs` —
  `crates/mycelium-core/src/languages/csharp.rs`:1
- c0063: `crates/mycelium-core/src/languages/declarations/c_cpp.rs` —
  `crates/mycelium-core/src/languages/declarations/c_cpp.rs`:1
- c0064: `crates/mycelium-core/src/languages/declarations/frameworks/attributes.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/attributes.rs`:1
- c0065: `crates/mycelium-core/src/languages/declarations/frameworks/java.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/java.rs`:1
- c0066: `crates/mycelium-core/src/languages/declarations/frameworks/javascript.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/javascript.rs`:1
- c0067: `Bindings` — `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs`:77
- c0068: `Detection` — `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs`:13
- c0069: `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs`:1
- c0070: `crates/mycelium-core/src/languages/declarations/frameworks/python.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/python.rs`:1
- c0071: `crates/mycelium-core/src/languages/declarations/go.rs` —
  `crates/mycelium-core/src/languages/declarations/go.rs`:1
- c0072: `crates/mycelium-core/src/languages/declarations/mod.rs` —
  `crates/mycelium-core/src/languages/declarations/mod.rs`:1
- c0073: `crates/mycelium-core/src/languages/declarations/nominal.rs` —
  `crates/mycelium-core/src/languages/declarations/nominal.rs`:1
- c0074: `crates/mycelium-core/src/languages/declarations/python.rs` —
  `crates/mycelium-core/src/languages/declarations/python.rs`:1
- c0075: `RustDetection` — `crates/mycelium-core/src/languages/declarations/test_detection.rs`:6
- c0076: `crates/mycelium-core/src/languages/declarations/test_detection.rs` —
  `crates/mycelium-core/src/languages/declarations/test_detection.rs`:1
- c0077: `crates/mycelium-core/src/languages/declarations/vbnet.rs` —
  `crates/mycelium-core/src/languages/declarations/vbnet.rs`:1
- c0078: `GoAnalyser` — `crates/mycelium-core/src/languages/go_lang.rs`:66
- c0079: `JavaAnalyser` — `crates/mycelium-core/src/languages/java.rs`:119
- c0080: `crates/mycelium-core/src/languages/java.rs` —
  `crates/mycelium-core/src/languages/java.rs`:1
- c0081: `AnalyserRegistry` — `crates/mycelium-core/src/languages/mod.rs`:55
- c0082: `LanguageAnalyser` — `crates/mycelium-core/src/languages/mod.rs`:20
- c0083: `PythonAnalyser` — `crates/mycelium-core/src/languages/python.rs`:93
- c0084: `RustAnalyser` — `crates/mycelium-core/src/languages/rust_lang.rs`:111
- c0085: `crates/mycelium-core/src/languages/rust_lang.rs` —
  `crates/mycelium-core/src/languages/rust_lang.rs`:1
- c0086: `TypeScriptAnalyser` — `crates/mycelium-core/src/languages/typescript.rs`:74
- c0087: `crates/mycelium-core/src/languages/typescript.rs` —
  `crates/mycelium-core/src/languages/typescript.rs`:1
- c0088: `VbNetAnalyser` — `crates/mycelium-core/src/languages/vbnet.rs`:142
- c0089: `crates/mycelium-core/src/languages/vbnet.rs` —
  `crates/mycelium-core/src/languages/vbnet.rs`:1
- c0090: `CallEndpoint` — `crates/mycelium-core/src/mermaid.rs`:407
- c0091: `ExportError` — `crates/mycelium-core/src/mermaid.rs`:63
- c0092: `MermaidExport` — `crates/mycelium-core/src/mermaid.rs`:31
- c0093: `MermaidOptions` — `crates/mycelium-core/src/mermaid.rs`:38
- c0094: `TestMode` — `crates/mycelium-core/src/mermaid.rs`:13
- c0095: `crates/mycelium-core/src/mermaid.rs` — `crates/mycelium-core/src/mermaid.rs`:1
- c0096: `TestFilter` — `crates/mycelium-core/src/mermaid/filtering.rs`:9
- c0097: `crates/mycelium-core/src/mermaid/filtering.rs` —
  `crates/mycelium-core/src/mermaid/filtering.rs`:1
- c0098: `crates/mycelium-core/src/output.rs` — `crates/mycelium-core/src/output.rs`:1
- c0099: `crates/mycelium-core/src/phases/calls.rs` — `crates/mycelium-core/src/phases/calls.rs`:1
- c0100: `AdjList` — `crates/mycelium-core/src/phases/communities.rs`:99
- c0101: `crates/mycelium-core/src/phases/communities.rs` —
  `crates/mycelium-core/src/phases/communities.rs`:1
- c0102: `crates/mycelium-core/src/phases/imports.rs` —
  `crates/mycelium-core/src/phases/imports.rs`:1
- c0103: `crates/mycelium-core/src/phases/parsing.rs` —
  `crates/mycelium-core/src/phases/parsing.rs`:1
- c0104: `crates/mycelium-core/src/phases/processes.rs` —
  `crates/mycelium-core/src/phases/processes.rs`:1
- c0105: `crates/mycelium-core/src/phases/structure.rs` —
  `crates/mycelium-core/src/phases/structure.rs`:1
- c0106: `crates/mycelium-core/src/pipeline.rs` — `crates/mycelium-core/src/pipeline.rs`:1
- c0122: `PyAnalysisConfig` — `crates/mycelium-python/src/lib.rs`:12
- c0123: `crates/mycelium-python/src/lib.rs` — `crates/mycelium-python/src/lib.rs`:1
- c0124: `mycelium/cli.py` — `mycelium/cli.py`:1
- c0254: `vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs` —
  `vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs`:1
- c0255: `vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs` —
  `vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs`:1
- c0256: `vendor/tree-sitter-vb-dotnet/grammar.js` — `vendor/tree-sitter-vb-dotnet/grammar.js`:1
- c0257: `BdistWheel` — `vendor/tree-sitter-vb-dotnet/setup.py`:38
- c0258: `Build` — `vendor/tree-sitter-vb-dotnet/setup.py`:30
- c0259: `EggInfo` — `vendor/tree-sitter-vb-dotnet/setup.py`:46
- c0260: `vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h` —
  `vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h`:1
- c0261: `TSCharacterRange` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:102
- c0262: `TSFieldMapEntry` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:28
- c0263: `TSLanguage` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:107
- c0264: `TSLanguageMetadata` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:21
- c0265: `TSLexMode` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:83
- c0266: `TSLexer` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:48
- c0267: `TSLexerMode` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:88
- c0268: `TSMapSlice` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:35
- c0269: `TSParseAction` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:66
- c0270: `TSParseActionEntry` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:94
- c0271: `TSParseActionType` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:59
- c0272: `TSSymbolMetadata` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:40
- c0273: `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h` —
  `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:1

## Relationships

- c0000 --> c0001: `field command`
- c0002 ..> c0002: `main() calls run_export()`
- c0002 ..> c0002: `main() calls run_quiet()`
- c0002 ..> c0002: `main() calls run_with_progress()`
- c0002 ..> c0004: `type in run_quiet`
- c0002 ..> c0004: `type in run_with_progress`
- c0002 ..> c0040: `run_export() calls new()`
- c0002 ..> c0095: `run_export() calls export_mermaid_report()`
- c0002 ..> c0098: `run_quiet() calls write_output()`
- c0002 ..> c0098: `run_with_progress() calls write_output()`
- c0002 ..> c0106: `run_quiet() calls run_pipeline()`
- c0002 ..> c0106: `run_with_progress() calls run_pipeline()`
- c0004 ..> c0122: `type in from`
- c0005 --> c0007: `field calls`
- c0005 --> c0009: `field communities`
- c0005 --> c0017: `field imports`
- c0005 --> c0021: `field processes`
- c0005 --> c0025: `field structure`
- c0005 --> c0027: `field symbols`
- c0005 ..> c0030: `default() calls default_max_branching()`
- c0005 ..> c0030: `default() calls default_max_community_size()`
- c0005 ..> c0030: `default() calls default_max_depth()`
- c0005 ..> c0030: `default() calls default_max_file_size()`
- c0005 ..> c0030: `default() calls default_max_processes()`
- c0005 ..> c0030: `default() calls default_min_steps()`
- c0005 ..> c0030: `default() calls default_resolution()`
- c0005 ..> c0030: `default() calls default_version()`
- c0005 ..> c0036: `default() calls default()`
- c0017 --> c0015: `field file_imports`
- c0017 --> c0018: `field package_references`
- c0017 --> c0022: `field project_references`
- c0025 --> c0011: `field files`
- c0025 --> c0013: `field folders`
- c0026 --> c0028: `field symbol_type`
- c0026 --> c0029: `field visibility`
- c0029 ..> c0029: `fmt() calls as_str()`
- c0032 --> c0031: `field bases`
- c0032 --> c0034: `field members`
- c0032 --> c0038: `field test`
- c0033 --> c0032: `field classes`
- c0033 --> c0036: `field test_detection`
- c0034 --> c0035: `field parameters`
- c0034 --> c0038: `field test`
- c0036 --> c0037: `field diagnostics`
- c0040 ..> c0028: `resolve_namespace() calls as_str()`
- c0040 ..> c0040: `default() calls new()`
- c0043 ..> c0028: `extract_attr() calls as_str()`
- c0043 ..> c0040: `parse_project_file() calls new()`
- c0043 ..> c0042: `type in parse_project_file`
- c0043 ..> c0043: `extract_include_attrs() calls extract_attr()`
- c0043 ..> c0043: `extract_package_refs() calls extract_attr()`
- c0043 ..> c0043: `extract_package_refs() calls extract_element_text()`
- c0043 ..> c0043: `parse_project_file() calls extract_element_text()`
- c0043 ..> c0043: `parse_project_file() calls extract_include_attrs()`
- c0043 ..> c0043: `parse_project_file() calls extract_package_refs()`
- c0045 ..> c0044: `type in parse_solution`
- c0048 ..> c0006: `type in add_call`
- c0048 ..> c0008: `type in add_community`
- c0048 ..> c0010: `type in add_file`
- c0048 ..> c0012: `type in add_folder`
- c0048 ..> c0014: `type in add_import`
- c0048 ..> c0019: `type in add_package_reference`
- c0048 ..> c0020: `type in add_process`
- c0048 ..> c0023: `type in add_project_reference`
- c0048 ..> c0026: `type in add_symbol`
- c0048 ..> c0028: `add_symbol() calls as_str()`
- c0048 ..> c0040: `new() calls new()`
- c0048 ..> c0046: `type in get_callees`
- c0048 ..> c0046: `type in get_callers`
- c0048 --> c0047: `field graph`
- c0048 ..> c0047: `type in inner_graph`
- c0048 ..> c0048: `add_community() calls ensure_node()`
- c0048 ..> c0048: `add_file() calls ensure_node()`
- c0048 ..> c0048: `add_folder() calls ensure_node()`
- c0048 ..> c0048: `add_import() calls ensure_node()`
- c0048 ..> c0048: `add_package_reference() calls ensure_node()`
- c0048 ..> c0048: `add_process() calls ensure_node()`
- c0048 ..> c0048: `add_project_reference() calls ensure_node()`
- c0048 ..> c0048: `add_symbol() calls ensure_node()`
- c0048 ..> c0048: `default() calls new()`
- c0048 ..> c0048: `get_call_edges() calls node_id()`
- c0048 ..> c0048: `get_callees() calls node_id()`
- c0048 ..> c0048: `get_callers() calls node_id()`
- c0048 ..> c0048: `get_communities() calls node_id()`
- c0048 ..> c0048: `get_import_edges() calls node_id()`
- c0048 ..> c0048: `get_package_references() calls node_id()`
- c0048 ..> c0048: `get_processes() calls node_id()`
- c0048 ..> c0048: `get_project_references() calls node_id()`
- c0048 --> c0049: `field graph`
- c0048 ..> c0049: `type in ensure_node`
- c0048 ..> c0049: `type in get_files`
- c0048 ..> c0049: `type in get_folders`
- c0048 ..> c0049: `type in get_node_data`
- c0048 ..> c0049: `type in inner_graph`
- c0048 ..> c0050: `type in get_symbols`
- c0048 ..> c0050: `type in get_symbols_in_file`
- c0048 ..> c0100: `add_call() calls add_edge()`
- c0048 ..> c0100: `add_community() calls add_edge()`
- c0048 ..> c0100: `add_import() calls add_edge()`
- c0048 ..> c0100: `add_package_reference() calls add_edge()`
- c0048 ..> c0100: `add_process() calls add_edge()`
- c0048 ..> c0100: `add_project_reference() calls add_edge()`
- c0048 ..> c0100: `add_symbol() calls add_edge()`
- c0052 ..> c0052: `default() calls new()`
- c0054 ..> c0028: `score_entry_points() calls as_str()`
- c0054 ..> c0048: `probe_depth() calls get_callees()`
- c0054 ..> c0048: `score_entry_points() calls get_callees()`
- c0054 ..> c0048: `score_entry_points() calls get_callers()`
- c0054 ..> c0048: `score_entry_points() calls get_symbols()`
- c0054 ..> c0048: `type in probe_depth`
- c0054 ..> c0048: `type in score_entry_points`
- c0054 ..> c0054: `score_entry_points() calls probe_depth()`
- c0056 ..> c0026: `type in add`
- c0056 ..> c0028: `add() calls as_str()`
- c0056 ..> c0028: `lookup_exact() calls as_str()`
- c0056 --> c0055: `field global_index`
- c0056 ..> c0055: `type in global_index`
- c0056 ..> c0055: `type in lookup_fuzzy`
- c0056 ..> c0056: `default() calls new()`
- c0058 ..> c0016: `type in extract_imports`
- c0058 ..> c0024: `type in extract_calls`
- c0058 ..> c0026: `type in extract_symbols`
- c0058 ..|> c0082: `implements`
- c0059 ..> c0016: `type in extract_imports`
- c0059 ..> c0024: `type in extract_calls`
- c0059 ..> c0026: `type in extract_cpp_symbols`
- c0059 ..> c0026: `type in extract_symbols`
- c0059 ..> c0059: `extract_calls() calls builtin_exclusions()`
- c0059 ..> c0059: `extract_symbols() calls extract_cpp_symbols()`
- c0059 ..> c0060: `extract_calls() calls find_c_calls()`
- c0059 ..> c0060: `extract_cpp_symbols() calls extract_c_symbols()`
- c0059 ..> c0060: `extract_cpp_symbols() calls get_type_name()`
- c0059 ..> c0060: `extract_imports() calls extract_includes()`
- c0059 ..> c0060: `extract_symbols() calls extract_c_symbols()`
- c0059 ..|> c0082: `implements`
- c0060 ..> c0016: `type in extract_includes`
- c0060 ..> c0024: `type in find_c_calls`
- c0060 ..> c0026: `type in extract_c_symbols`
- c0060 ..> c0060: `extract_c_symbols() calls get_func_name()`
- c0060 ..> c0060: `extract_c_symbols() calls get_qualified_func_name()`
- c0060 ..> c0060: `extract_c_symbols() calls get_type_name()`
- c0060 ..> c0060: `extract_c_symbols() calls is_preproc_container()`
- c0060 ..> c0060: `find_c_calls() calls extract_c_callee()`
- c0060 ..> c0060: `find_c_calls() calls find_enclosing_func()`
- c0060 ..> c0060: `find_enclosing_func() calls get_qualified_func_name()`
- c0060 ..> c0060: `get_func_name() calls get_qualified_func_name()`
- c0061 ..> c0016: `type in extract_imports`
- c0061 ..> c0016: `type in extract_using`
- c0061 ..> c0024: `type in extract_calls`
- c0061 ..> c0024: `type in find_calls`
- c0061 ..> c0026: `type in extract_symbols`
- c0061 ..> c0026: `type in walk_node`
- c0061 ..> c0061: `extract_calls() calls builtin_exclusions()`
- c0061 ..> c0061: `extract_calls() calls find_calls()`
- c0061 ..> c0061: `extract_imports() calls extract_using()`
- c0061 ..> c0061: `extract_symbols() calls walk_node()`
- c0061 ..> c0062: `find_calls() calls extract_callee()`
- c0061 ..> c0062: `find_calls() calls find_enclosing_method()`
- c0061 ..> c0062: `walk_node() calls extract_parameter_types()`
- c0061 ..> c0062: `walk_node() calls get_name()`
- c0061 ..> c0062: `walk_node() calls get_visibility()`
- c0061 ..> c0062: `walk_node() calls is_container()`
- c0061 ..> c0062: `walk_node() calls node_to_symbol_type()`
- c0061 ..> c0078: `find_calls() calls find_calls()`
- c0061 ..> c0079: `walk_node() calls walk_node()`
- c0061 ..|> c0082: `implements`
- c0062 ..> c0028: `get_visibility() calls as_str()`
- c0062 ..> c0028: `type in node_to_symbol_type`
- c0062 ..> c0029: `type in get_visibility`
- c0063 ..> c0033: `type in walk`
- c0063 ..> c0063: `walk() calls declaration_type()`
- c0063 ..> c0063: `walk() calls declarator_name()`
- c0063 ..> c0063: `walk() calls find_function()`
- c0063 ..> c0071: `walk() calls walk()`
- c0063 ..> c0072: `declaration_type() calls children()`
- c0063 ..> c0072: `declaration_type() calls field()`
- c0063 ..> c0072: `declaration_type() calls text()`
- c0063 ..> c0072: `declarator_name() calls text()`
- c0063 ..> c0072: `walk() calls add_class()`
- c0063 ..> c0072: `walk() calls children()`
- c0063 ..> c0072: `walk() calls enum_member()`
- c0063 ..> c0072: `walk() calls field()`
- c0063 ..> c0072: `walk() calls module()`
- c0063 ..> c0072: `walk() calls text()`
- c0064 ..> c0028: `detect() calls as_str()`
- c0064 ..> c0064: `detect() calls ancestor()`
- c0064 ..> c0064: `detect() calls clean()`
- c0064 ..> c0064: `detect() calls guard_name()`
- c0064 ..> c0064: `detect() calls inherited_scope()`
- c0064 ..> c0064: `detect() calls key()`
- c0064 ..> c0064: `detect() calls rule()`
- c0064 ..> c0064: `detect() calls scope()`
- c0064 ..> c0064: `detect() calls type_node()`
- c0064 ..> c0064: `guard_name() calls key()`
- c0064 ..> c0064: `inherited_scope() calls type_node()`
- c0064 ..> c0064: `scope() calls scope_inner()`
- c0064 ..> c0064: `scope_inner() calls type_node()`
- c0064 ..> c0068: `detect() calls mark()`
- c0064 ..> c0068: `type in detect`
- c0064 ..> c0069: `detect() calls keyword()`
- c0064 ..> c0069: `detect() calls visit()`
- c0064 ..> c0072: `detect() calls children()`
- c0064 ..> c0072: `detect() calls field()`
- c0064 ..> c0072: `detect() calls text()`
- c0064 ..> c0072: `inherited_scope() calls children()`
- c0064 ..> c0072: `scope_inner() calls children()`
- c0064 ..> c0072: `scope_inner() calls field()`
- c0065 ..> c0004: `detect() calls from()`
- c0065 ..> c0028: `detect() calls as_str()`
- c0065 ..> c0064: `detect() calls inherited_scope()`
- c0065 ..> c0064: `detect() calls scope()`
- c0065 ..> c0064: `detect() calls type_node()`
- c0065 ..> c0068: `detect() calls mark()`
- c0065 ..> c0068: `type in detect`
- c0065 ..> c0069: `detect() calls visit()`
- c0065 ..> c0072: `detect() calls children()`
- c0065 ..> c0072: `detect() calls field()`
- c0065 ..> c0072: `detect() calls text()`
- c0066 ..> c0066: `detect() calls bound_names()`
- c0066 ..> c0066: `detect() calls inline()`
- c0066 ..> c0066: `detect() calls literal()`
- c0066 ..> c0068: `detect() calls mark()`
- c0066 ..> c0068: `type in detect`
- c0066 ..> c0069: `detect() calls keyword()`
- c0066 ..> c0069: `detect() calls visit()`
- c0066 ..> c0070: `bound_names() calls bound_names()`
- c0066 ..> c0072: `bound_names() calls children()`
- c0066 ..> c0072: `bound_names() calls text()`
- c0066 ..> c0072: `detect() calls children()`
- c0066 ..> c0072: `detect() calls field()`
- c0066 ..> c0072: `detect() calls text()`
- c0066 ..> c0096: `detect() calls retain()`
- c0067 --> c0038: `field guards`
- c0068 ..> c0004: `new() calls default()`
- c0068 --> c0038: `field marks`
- c0068 ..> c0038: `type in evidence`
- c0068 ..> c0064: `evidence() calls type_node()`
- c0068 ..> c0064: `new() calls detect()`
- c0068 --> c0067: `field bindings`
- c0068 ..> c0076: `new() calls malformed()`
- c0069 ..> c0063: `keyword() calls walk()`
- c0069 ..> c0067: `finish() calls extend()`
- c0069 ..> c0067: `type in finish`
- c0069 ..> c0068: `finish() calls new()`
- c0069 ..> c0072: `keyword() calls children()`
- c0069 ..> c0072: `visit() calls children()`
- c0070 ..> c0040: `detect() calls new()`
- c0070 ..> c0063: `detect() calls walk()`
- c0070 ..> c0066: `bound_names() calls bound_names()`
- c0070 ..> c0068: `detect() calls mark()`
- c0070 ..> c0068: `type in detect`
- c0070 ..> c0069: `detect() calls visit()`
- c0070 ..> c0070: `detect() calls bound_names()`
- c0070 ..> c0072: `bound_names() calls children()`
- c0070 ..> c0072: `bound_names() calls text()`
- c0070 ..> c0072: `detect() calls children()`
- c0070 ..> c0072: `detect() calls field()`
- c0070 ..> c0072: `detect() calls text()`
- c0070 ..> c0095: `detect() calls resolve()`
- c0070 ..> c0096: `detect() calls retain()`
- c0071 ..> c0033: `type in walk`
- c0071 ..> c0063: `walk() calls walk()`
- c0071 ..> c0072: `walk() calls add_class()`
- c0071 ..> c0072: `walk() calls children()`
- c0071 ..> c0072: `walk() calls field()`
- c0071 ..> c0072: `walk() calls module()`
- c0071 ..> c0072: `walk() calls text()`
- c0072 ..> c0004: `extract() calls default()`
- c0072 ..> c0033: `type in add_class`
- c0072 ..> c0033: `type in enum_member`
- c0072 ..> c0033: `type in extract`
- c0072 ..> c0033: `type in module`
- c0072 ..> c0033: `type in rust_walk`
- c0072 ..> c0040: `extract() calls new()`
- c0072 ..> c0063: `children() calls walk()`
- c0072 ..> c0063: `extract() calls walk()`
- c0072 ..> c0063: `rust_walk() calls walk()`
- c0072 ..> c0067: `extract() calls extend()`
- c0072 ..> c0072: `extract() calls rust_walk()`
- c0072 ..> c0072: `field() calls text()`
- c0072 ..> c0072: `module() calls add_class()`
- c0072 ..> c0072: `rust_walk() calls add_class()`
- c0072 ..> c0072: `rust_walk() calls children()`
- c0072 ..> c0072: `rust_walk() calls field()`
- c0072 ..> c0072: `rust_walk() calls module()`
- c0072 ..> c0072: `rust_walk() calls text()`
- c0072 ..> c0076: `rust_walk() calls rust_evidence()`
- c0073 ..> c0033: `type in add_field`
- c0073 ..> c0033: `type in walk`
- c0073 ..> c0063: `walk() calls walk()`
- c0073 ..> c0068: `add_field() calls evidence()`
- c0073 ..> c0068: `walk() calls evidence()`
- c0073 ..> c0072: `base_types() calls children()`
- c0073 ..> c0072: `type_field() calls field()`
- c0073 ..> c0072: `visibility() calls children()`
- c0073 ..> c0072: `visibility() calls text()`
- c0073 ..> c0072: `walk() calls add_class()`
- c0073 ..> c0072: `walk() calls children()`
- c0073 ..> c0072: `walk() calls enum_member()`
- c0073 ..> c0072: `walk() calls field()`
- c0073 ..> c0072: `walk() calls module()`
- c0073 ..> c0072: `walk() calls text()`
- c0073 ..> c0073: `add_field() calls visibility()`
- c0073 ..> c0073: `base_types() calls walk()`
- c0073 ..> c0073: `walk() calls add_field()`
- c0073 ..> c0073: `walk() calls base_types()`
- c0073 ..> c0073: `walk() calls type_field()`
- c0073 ..> c0073: `walk() calls visibility()`
- c0074 ..> c0033: `type in instance_fields`
- c0074 ..> c0033: `type in push_field`
- c0074 ..> c0033: `type in walk`
- c0074 ..> c0063: `walk() calls walk()`
- c0074 ..> c0068: `push_field() calls evidence()`
- c0074 ..> c0068: `walk() calls evidence()`
- c0074 ..> c0072: `instance_fields() calls children()`
- c0074 ..> c0072: `instance_fields() calls field()`
- c0074 ..> c0072: `push_field() calls field()`
- c0074 ..> c0072: `walk() calls add_class()`
- c0074 ..> c0072: `walk() calls children()`
- c0074 ..> c0072: `walk() calls field()`
- c0074 ..> c0072: `walk() calls module()`
- c0074 ..> c0072: `walk() calls text()`
- c0074 ..> c0074: `instance_fields() calls push_field()`
- c0074 ..> c0074: `walk() calls instance_fields()`
- c0074 ..> c0074: `walk() calls push_field()`
- c0075 ..> c0076: `new() calls malformed()`
- c0075 ..> c0076: `new() calls uncertain_import()`
- c0076 ..> c0038: `type in evidence`
- c0076 ..> c0038: `type in rust_evidence`
- c0076 ..> c0063: `binds_test() calls walk()`
- c0076 ..> c0063: `malformed() calls walk()`
- c0076 ..> c0063: `rust_evidence() calls walk()`
- c0076 ..> c0063: `tokens() calls walk()`
- c0076 ..> c0063: `uncertain_import() calls walk()`
- c0076 ..> c0072: `malformed() calls children()`
- c0076 ..> c0072: `tokens() calls children()`
- c0076 ..> c0075: `type in rust_evidence`
- c0076 ..> c0076: `rust_evidence() calls evidence()`
- c0076 ..> c0076: `rust_evidence() calls tokens()`
- c0076 ..> c0076: `uncertain_import() calls binds_test()`
- c0077 ..> c0033: `type in walk`
- c0077 ..> c0063: `walk() calls walk()`
- c0077 ..> c0068: `walk() calls evidence()`
- c0077 ..> c0072: `vb_type() calls children()`
- c0077 ..> c0072: `vb_type() calls field()`
- c0077 ..> c0072: `vb_type() calls text()`
- c0077 ..> c0072: `walk() calls add_class()`
- c0077 ..> c0072: `walk() calls children()`
- c0077 ..> c0072: `walk() calls enum_member()`
- c0077 ..> c0072: `walk() calls field()`
- c0077 ..> c0072: `walk() calls module()`
- c0077 ..> c0072: `walk() calls text()`
- c0077 ..> c0077: `walk() calls vb_type()`
- c0078 ..> c0016: `type in extract_imports`
- c0078 ..> c0024: `type in extract_calls`
- c0078 ..> c0024: `type in find_calls`
- c0078 ..> c0026: `type in extract_symbols`
- c0078 ..> c0061: `find_calls() calls find_calls()`
- c0078 ..> c0078: `extract_calls() calls builtin_exclusions()`
- c0078 ..> c0078: `extract_calls() calls find_calls()`
- c0078 ..> c0078: `extract_imports() calls extract_string()`
- c0078 ..> c0078: `extract_imports() calls extract_string_content()`
- c0078 ..> c0078: `extract_string() calls extract_string_content()`
- c0078 ..> c0078: `extract_symbols() calls get_name_by_kind()`
- c0078 ..> c0078: `extract_symbols() calls is_exported()`
- c0078 ..> c0078: `find_calls() calls extract_callee()`
- c0078 ..> c0078: `find_calls() calls find_enclosing()`
- c0078 ..> c0078: `find_enclosing() calls get_name_by_kind()`
- c0078 ..|> c0082: `implements`
- c0079 ..> c0016: `type in extract_imports`
- c0079 ..> c0024: `type in extract_calls`
- c0079 ..> c0024: `type in find_calls`
- c0079 ..> c0026: `type in extract_symbols`
- c0079 ..> c0026: `type in walk_node`
- c0079 ..> c0061: `find_calls() calls find_calls()`
- c0079 ..> c0061: `walk_node() calls walk_node()`
- c0079 ..> c0079: `extract_calls() calls builtin_exclusions()`
- c0079 ..> c0079: `extract_calls() calls find_calls()`
- c0079 ..> c0079: `extract_symbols() calls walk_node()`
- c0079 ..> c0079: `find_calls() calls extract_callee()`
- c0079 ..> c0079: `find_calls() calls find_enclosing()`
- c0079 ..> c0080: `find_enclosing() calls get_name()`
- c0079 ..> c0080: `walk_node() calls get_name()`
- c0079 ..> c0080: `walk_node() calls get_visibility()`
- c0079 ..> c0080: `walk_node() calls is_container()`
- c0079 ..> c0080: `walk_node() calls node_to_symbol_type()`
- c0079 ..|> c0082: `implements`
- c0080 ..> c0028: `get_visibility() calls as_str()`
- c0080 ..> c0028: `type in node_to_symbol_type`
- c0080 ..> c0029: `type in get_visibility`
- c0081 ..> c0028: `extensions() calls as_str()`
- c0081 ..> c0058: `language_for_extension() calls language_name()`
- c0081 ..> c0081: `default() calls new()`
- c0081 ..> c0081: `language_for_extension() calls get_by_extension()`
- c0081 ..> c0081: `new() calls extensions()`
- c0081 --> c0082: `field analysers`
- c0081 ..> c0082: `type in get_by_extension`
- c0081 ..> c0088: `new() calls is_available()`
- c0082 ..> c0016: `type in extract_imports`
- c0082 ..> c0024: `type in extract_calls`
- c0082 ..> c0026: `type in extract_symbols`
- c0083 ..> c0016: `type in extract_imports`
- c0083 ..> c0024: `type in extract_calls`
- c0083 ..> c0024: `type in find_calls`
- c0083 ..> c0026: `type in extract_symbols`
- c0083 ..> c0026: `type in walk_node`
- c0083 ..> c0061: `find_calls() calls find_calls()`
- c0083 ..> c0061: `walk_node() calls walk_node()`
- c0083 ..|> c0082: `implements`
- c0083 ..> c0083: `extract_calls() calls builtin_exclusions()`
- c0083 ..> c0083: `extract_calls() calls find_calls()`
- c0083 ..> c0083: `extract_symbols() calls walk_node()`
- c0083 ..> c0083: `find_calls() calls extract_callee()`
- c0083 ..> c0083: `find_calls() calls find_enclosing()`
- c0083 ..> c0083: `walk_node() calls get_name()`
- c0084 ..> c0016: `type in extract_imports`
- c0084 ..> c0024: `type in extract_calls`
- c0084 ..> c0024: `type in find_calls`
- c0084 ..> c0026: `type in extract_symbols`
- c0084 ..> c0026: `type in walk_node`
- c0084 ..> c0061: `find_calls() calls find_calls()`
- c0084 ..> c0061: `walk_node() calls walk_node()`
- c0084 ..|> c0082: `implements`
- c0084 ..> c0084: `extract_calls() calls builtin_exclusions()`
- c0084 ..> c0084: `extract_calls() calls find_calls()`
- c0084 ..> c0084: `extract_symbols() calls walk_node()`
- c0084 ..> c0084: `find_calls() calls extract_callee()`
- c0084 ..> c0084: `find_calls() calls find_enclosing()`
- c0084 ..> c0084: `find_enclosing() calls get_name()`
- c0084 ..> c0084: `walk_node() calls get_name()`
- c0084 ..> c0084: `walk_node() calls is_pub()`
- c0084 ..> c0085: `walk_node() calls node_to_symbol_type()`
- c0085 ..> c0028: `type in node_to_symbol_type`
- c0086 ..> c0016: `type in extract_imports`
- c0086 ..> c0024: `type in extract_calls`
- c0086 ..> c0024: `type in find_calls`
- c0086 ..> c0026: `type in extract_class_members`
- c0086 ..> c0026: `type in extract_symbols`
- c0086 ..> c0026: `type in walk_node`
- c0086 ..> c0061: `find_calls() calls find_calls()`
- c0086 ..|> c0082: `implements`
- c0086 ..> c0086: `extract_calls() calls builtin_exclusions()`
- c0086 ..> c0086: `extract_calls() calls find_calls()`
- c0086 ..> c0086: `extract_imports() calls extract_string_source()`
- c0086 ..> c0086: `extract_symbols() calls walk_node()`
- c0086 ..> c0086: `find_calls() calls extract_callee()`
- c0086 ..> c0086: `find_calls() calls find_enclosing()`
- c0086 ..> c0086: `get_language() calls get_ts_language()`
- c0086 ..> c0086: `get_language_for_ext() calls get_js_language()`
- c0086 ..> c0086: `get_language_for_ext() calls get_ts_language()`
- c0086 ..> c0086: `get_language_for_ext() calls get_tsx_language()`
- c0086 ..> c0086: `walk_node() calls extract_class_members()`
- c0086 ..> c0086: `walk_node() calls get_name()`
- c0086 ..> c0086: `walk_node() calls language_for_path()`
- c0086 ..> c0087: `walk_node() calls node_to_symbol_type()`
- c0087 ..> c0028: `type in node_to_symbol_type`
- c0088 ..> c0016: `type in extract_imports`
- c0088 ..> c0024: `type in extract_calls`
- c0088 ..> c0024: `type in find_calls`
- c0088 ..> c0026: `type in extract_symbols`
- c0088 ..> c0026: `type in walk_node`
- c0088 ..> c0061: `find_calls() calls find_calls()`
- c0088 ..> c0061: `walk_node() calls walk_node()`
- c0088 ..|> c0082: `implements`
- c0088 ..> c0088: `extract_calls() calls builtin_exclusions()`
- c0088 ..> c0088: `extract_calls() calls find_calls()`
- c0088 ..> c0088: `extract_symbols() calls walk_node()`
- c0088 ..> c0089: `find_calls() calls extract_callee()`
- c0088 ..> c0089: `find_calls() calls find_enclosing_method()`
- c0088 ..> c0089: `walk_node() calls get_name()`
- c0088 ..> c0089: `walk_node() calls get_visibility()`
- c0088 ..> c0089: `walk_node() calls is_container()`
- c0088 ..> c0089: `walk_node() calls node_to_symbol_type()`
- c0089 ..> c0028: `type in node_to_symbol_type`
- c0089 ..> c0029: `type in get_visibility`
- c0089 ..> c0089: `find_enclosing_method() calls get_name()`
- c0090 --> c0032: `field owner`
- c0090 --> c0034: `field member`
- c0093 --> c0094: `field tests`
- c0095 ..> c0005: `type in export_mermaid`
- c0095 ..> c0005: `type in export_mermaid_report`
- c0095 ..> c0028: `export_mermaid_report() calls as_str()`
- c0095 ..> c0028: `type_edges() calls as_str()`
- c0095 ..> c0028: `type_names() calls as_str()`
- c0095 ..> c0028: `unique_occurrences() calls as_str()`
- c0095 ..> c0032: `type in merge_classes`
- c0095 ..> c0032: `type in name_index`
- c0095 ..> c0032: `type in pages`
- c0095 ..> c0032: `type in resolve`
- c0095 ..> c0032: `type in type_edges`
- c0095 ..> c0032: `type in unique_occurrences`
- c0095 ..> c0034: `type in member_text`
- c0095 ..> c0034: `type in pages`
- c0095 ..> c0034: `type in plain_member`
- c0095 ..> c0040: `export_mermaid_report() calls new()`
- c0095 ..> c0040: `language_family() calls new()`
- c0095 ..> c0040: `resolve() calls new()`
- c0095 ..> c0040: `type_names() calls new()`
- c0095 ..> c0067: `merge_classes() calls extend()`
- c0095 ..> c0067: `type_names() calls extend()`
- c0095 ..> c0081: `language_family() calls language_for_extension()`
- c0095 ..> c0091: `type in export_mermaid`
- c0095 ..> c0091: `type in export_mermaid_report`
- c0095 ..> c0092: `type in export_mermaid_report`
- c0095 ..> c0093: `type in export_mermaid`
- c0095 ..> c0093: `type in export_mermaid_report`
- c0095 ..> c0095: `export_mermaid() calls export_mermaid_report()`
- c0095 ..> c0095: `export_mermaid_report() calls abbreviation()`
- c0095 ..> c0095: `export_mermaid_report() calls language_family()`
- c0095 ..> c0095: `export_mermaid_report() calls member_text()`
- c0095 ..> c0095: `export_mermaid_report() calls merge_classes()`
- c0095 ..> c0095: `export_mermaid_report() calls name_index()`
- c0095 ..> c0095: `export_mermaid_report() calls ordered_key()`
- c0095 ..> c0095: `export_mermaid_report() calls pages()`
- c0095 ..> c0095: `export_mermaid_report() calls plain_member()`
- c0095 ..> c0095: `export_mermaid_report() calls resolve()`
- c0095 ..> c0095: `export_mermaid_report() calls safe()`
- c0095 ..> c0095: `export_mermaid_report() calls type_edges()`
- c0095 ..> c0095: `export_mermaid_report() calls unique_occurrences()`
- c0095 ..> c0095: `export_mermaid_report() calls wrap_prose()`
- c0095 ..> c0095: `member_text() calls type_text()`
- c0095 ..> c0095: `member_text() calls visibility()`
- c0095 ..> c0095: `merge_classes() calls name_index()`
- c0095 ..> c0095: `merge_classes() calls resolve()`
- c0095 ..> c0095: `resolve() calls language_family()`
- c0095 ..> c0095: `type_edges() calls language_family()`
- c0095 ..> c0095: `type_edges() calls name_index()`
- c0095 ..> c0095: `type_edges() calls resolve()`
- c0095 ..> c0095: `type_edges() calls type_names()`
- c0095 ..> c0096: `export_mermaid_report() calls excludes_file()`
- c0095 ..> c0096: `export_mermaid_report() calls retain()`
- c0095 ..> c0096: `export_mermaid_report() calls summary()`
- c0095 ..> c0097: `export_mermaid_report() calls matches_path()`
- c0095 ..> c0097: `export_mermaid_report() calls normalize_path()`
- c0096 ..> c0005: `type in new`
- c0096 ..> c0028: `hidden() calls as_str()`
- c0096 ..> c0032: `type in retain`
- c0096 ..> c0038: `type in hidden`
- c0096 ..> c0039: `hidden() calls supported_test_rule()`
- c0096 ..> c0067: `new() calls extend()`
- c0096 ..> c0091: `type in new`
- c0096 ..> c0093: `type in new`
- c0096 ..> c0096: `hidden() calls valid_location()`
- c0096 ..> c0096: `retain() calls hidden()`
- c0096 ..> c0096: `retain() calls valid_location()`
- c0096 ..> c0097: `excludes_file() calls matches_path()`
- c0096 ..> c0097: `hidden() calls matches_path()`
- c0096 ..> c0097: `new() calls matches_path()`
- c0096 ..> c0097: `new() calls normalize_path()`
- c0096 ..> c0097: `valid_location() calls normalize_path()`
- c0097 ..> c0091: `type in normalize_path`
- c0098 ..> c0004: `type in build_result`
- c0098 ..> c0005: `type in build_result`
- c0098 ..> c0005: `type in write_output`
- c0098 ..> c0040: `build_result() calls new()`
- c0098 ..> c0040: `get_commit_hash() calls new()`
- c0098 ..> c0040: `write_output() calls new()`
- c0098 ..> c0048: `build_result() calls get_call_edges()`
- c0098 ..> c0048: `build_result() calls get_communities()`
- c0098 ..> c0048: `build_result() calls get_files()`
- c0098 ..> c0048: `build_result() calls get_folders()`
- c0098 ..> c0048: `build_result() calls get_import_edges()`
- c0098 ..> c0048: `build_result() calls get_package_references()`
- c0098 ..> c0048: `build_result() calls get_processes()`
- c0098 ..> c0048: `build_result() calls get_project_references()`
- c0098 ..> c0048: `build_result() calls get_symbols()`
- c0098 ..> c0048: `count_languages() calls get_files()`
- c0098 ..> c0048: `type in build_result`
- c0098 ..> c0048: `type in count_languages`
- c0098 ..> c0056: `type in build_result`
- c0098 ..> c0098: `build_result() calls count_languages()`
- c0098 ..> c0098: `build_result() calls get_commit_hash()`
- c0099 ..> c0004: `type in run_calls_phase`
- c0099 ..> c0006: `type in resolve_call`
- c0099 ..> c0028: `call_target_in_file() calls as_str()`
- c0099 ..> c0028: `is_call_target() calls as_str()`
- c0099 ..> c0028: `run_calls_phase() calls as_str()`
- c0099 ..> c0040: `run_calls_phase() calls new()`
- c0099 ..> c0048: `build_field_type_map() calls get_symbols_in_file()`
- c0099 ..> c0048: `build_import_map() calls get_import_edges()`
- c0099 ..> c0048: `find_implementation() calls get_symbols()`
- c0099 ..> c0048: `is_call_target() calls get_node_data()`
- c0099 ..> c0048: `is_interface_method() calls get_symbols()`
- c0099 ..> c0048: `is_interface_self_call() calls get_symbols()`
- c0099 ..> c0048: `run_calls_phase() calls add_call()`
- c0099 ..> c0048: `run_calls_phase() calls get_files()`
- c0099 ..> c0048: `type in build_field_type_map`
- c0099 ..> c0048: `type in build_import_map`
- c0099 ..> c0048: `type in call_target_in_file`
- c0099 ..> c0048: `type in find_implementation`
- c0099 ..> c0048: `type in is_call_target`
- c0099 ..> c0048: `type in is_interface_method`
- c0099 ..> c0048: `type in is_interface_self_call`
- c0099 ..> c0048: `type in resolve_call`
- c0099 ..> c0048: `type in run_calls_phase`
- c0099 ..> c0052: `type in run_calls_phase`
- c0099 ..> c0056: `call_target_in_file() calls lookup_exact()`
- c0099 ..> c0056: `call_target_in_file() calls lookup_fuzzy()`
- c0099 ..> c0056: `find_implementation() calls lookup_fuzzy()`
- c0099 ..> c0056: `resolve_call() calls lookup_exact()`
- c0099 ..> c0056: `resolve_call() calls lookup_fuzzy()`
- c0099 ..> c0056: `type in call_target_in_file`
- c0099 ..> c0056: `type in find_implementation`
- c0099 ..> c0056: `type in resolve_call`
- c0099 ..> c0056: `type in run_calls_phase`
- c0099 ..> c0058: `run_calls_phase() calls extract_calls()`
- c0099 ..> c0081: `run_calls_phase() calls get_by_extension()`
- c0099 ..> c0086: `run_calls_phase() calls get_language_for_ext()`
- c0099 ..> c0088: `run_calls_phase() calls is_available()`
- c0099 ..> c0099: `call_target_in_file() calls is_call_target()`
- c0099 ..> c0099: `find_implementation() calls call_target_in_file()`
- c0099 ..> c0099: `find_implementation() calls is_call_target()`
- c0099 ..> c0099: `find_implementation() calls is_interface_method()`
- c0099 ..> c0099: `resolve_call() calls call_target_in_file()`
- c0099 ..> c0099: `resolve_call() calls find_implementation()`
- c0099 ..> c0099: `resolve_call() calls is_call_target()`
- c0099 ..> c0099: `resolve_call() calls is_interface_method()`
- c0099 ..> c0099: `resolve_call() calls is_interface_self_call()`
- c0099 ..> c0099: `run_calls_phase() calls build_field_type_map()`
- c0099 ..> c0099: `run_calls_phase() calls build_import_map()`
- c0099 ..> c0099: `run_calls_phase() calls resolve_call()`
- c0100 ..> c0100: `add_edge() calls ensure_node()`
- c0101 ..> c0004: `type in run_communities_phase`
- c0101 ..> c0028: `compute_cohesion() calls as_str()`
- c0101 ..> c0028: `disambiguate_label() calls as_str()`
- c0101 ..> c0028: `generate_label() calls as_str()`
- c0101 ..> c0028: `primary_language() calls as_str()`
- c0101 ..> c0028: `split_oversized() calls as_str()`
- c0101 ..> c0048: `disambiguate_label() calls get_symbols()`
- c0101 ..> c0048: `generate_label() calls get_symbols()`
- c0101 ..> c0048: `primary_language() calls get_symbols()`
- c0101 ..> c0048: `run_communities_phase() calls add_community()`
- c0101 ..> c0048: `run_communities_phase() calls get_call_edges()`
- c0101 ..> c0048: `type in disambiguate_label`
- c0101 ..> c0048: `type in generate_label`
- c0101 ..> c0048: `type in primary_language`
- c0101 ..> c0048: `type in run_communities_phase`
- c0101 ..> c0067: `run_communities_phase() calls extend()`
- c0101 ..> c0067: `split_oversized() calls extend()`
- c0101 ..> c0100: `louvain() calls total_weight()`
- c0101 ..> c0100: `run_communities_phase() calls add_edge()`
- c0101 ..> c0100: `run_communities_phase() calls new()`
- c0101 ..> c0100: `split_oversized() calls add_edge()`
- c0101 ..> c0100: `split_oversized() calls ensure_node()`
- c0101 ..> c0100: `split_oversized() calls new()`
- c0101 ..> c0100: `split_oversized() calls total_weight()`
- c0101 ..> c0100: `type in compute_cohesion`
- c0101 ..> c0100: `type in louvain`
- c0101 ..> c0100: `type in split_oversized`
- c0101 ..> c0101: `generate_label() calls common_prefix()`
- c0101 ..> c0101: `run_communities_phase() calls compute_cohesion()`
- c0101 ..> c0101: `run_communities_phase() calls disambiguate_label()`
- c0101 ..> c0101: `run_communities_phase() calls generate_label()`
- c0101 ..> c0101: `run_communities_phase() calls louvain()`
- c0101 ..> c0101: `run_communities_phase() calls primary_language()`
- c0101 ..> c0101: `run_communities_phase() calls split_oversized()`
- c0101 ..> c0101: `split_oversized() calls louvain()`
- c0102 ..> c0004: `type in process_dotnet_projects`
- c0102 ..> c0004: `type in process_source_imports`
- c0102 ..> c0004: `type in run_imports_phase`
- c0102 ..> c0028: `process_source_imports() calls as_str()`
- c0102 ..> c0040: `build_go_dir_index() calls new()`
- c0102 ..> c0040: `parse_go_mod() calls new()`
- c0102 ..> c0040: `process_dotnet_projects() calls new()`
- c0102 ..> c0040: `process_dotnet_projects() calls register()`
- c0102 ..> c0040: `process_source_imports() calls new()`
- c0102 ..> c0040: `resolve_c_include() calls new()`
- c0102 ..> c0040: `resolve_fallback() calls new()`
- c0102 ..> c0040: `resolve_fallback() calls resolve_namespace()`
- c0102 ..> c0040: `resolve_python_relative() calls new()`
- c0102 ..> c0040: `resolve_rust_import() calls new()`
- c0102 ..> c0040: `resolve_ts_import() calls new()`
- c0102 ..> c0040: `run_imports_phase() calls new()`
- c0102 ..> c0040: `type in process_dotnet_projects`
- c0102 ..> c0040: `type in process_source_imports`
- c0102 ..> c0040: `type in register_observed_namespaces`
- c0102 ..> c0040: `type in resolve_fallback`
- c0102 ..> c0043: `process_dotnet_projects() calls parse_project_file()`
- c0102 ..> c0045: `process_dotnet_projects() calls parse_solution()`
- c0102 ..> c0048: `process_dotnet_projects() calls add_package_reference()`
- c0102 ..> c0048: `process_dotnet_projects() calls add_project_reference()`
- c0102 ..> c0048: `process_dotnet_projects() calls get_files()`
- c0102 ..> c0048: `process_source_imports() calls add_import()`
- c0102 ..> c0048: `process_source_imports() calls get_files()`
- c0102 ..> c0048: `register_observed_namespaces() calls get_symbols()`
- c0102 ..> c0048: `resolve_fallback() calls get_files()`
- c0102 ..> c0048: `resolve_fallback() calls get_symbols_in_file()`
- c0102 ..> c0048: `type in process_dotnet_projects`
- c0102 ..> c0048: `type in process_source_imports`
- c0102 ..> c0048: `type in register_observed_namespaces`
- c0102 ..> c0048: `type in resolve_fallback`
- c0102 ..> c0048: `type in run_imports_phase`
- c0102 ..> c0052: `process_source_imports() calls get_files_for_namespace()`
- c0102 ..> c0052: `process_source_imports() calls register_file_import()`
- c0102 ..> c0052: `type in process_source_imports`
- c0102 ..> c0052: `type in run_imports_phase`
- c0102 ..> c0056: `resolve_fallback() calls lookup_fuzzy()`
- c0102 ..> c0056: `type in process_source_imports`
- c0102 ..> c0056: `type in resolve_fallback`
- c0102 ..> c0056: `type in run_imports_phase`
- c0102 ..> c0058: `process_source_imports() calls extract_imports()`
- c0102 ..> c0081: `process_source_imports() calls get_by_extension()`
- c0102 ..> c0086: `process_source_imports() calls get_language_for_ext()`
- c0102 ..> c0088: `process_source_imports() calls is_available()`
- c0102 ..> c0102: `process_dotnet_projects() calls normalize_path()`
- c0102 ..> c0102: `process_source_imports() calls build_go_dir_index()`
- c0102 ..> c0102: `process_source_imports() calls parse_go_mod()`
- c0102 ..> c0102: `process_source_imports() calls resolve_c_include()`
- c0102 ..> c0102: `process_source_imports() calls resolve_fallback()`
- c0102 ..> c0102: `process_source_imports() calls resolve_go_import()`
- c0102 ..> c0102: `process_source_imports() calls resolve_java_import()`
- c0102 ..> c0102: `process_source_imports() calls resolve_python_import()`
- c0102 ..> c0102: `process_source_imports() calls resolve_rust_import()`
- c0102 ..> c0102: `process_source_imports() calls resolve_ts_import()`
- c0102 ..> c0102: `resolve_c_include() calls normalize_path()`
- c0102 ..> c0102: `resolve_python_import() calls resolve_python_relative()`
- c0102 ..> c0102: `resolve_ts_import() calls normalize_path()`
- c0102 ..> c0102: `run_imports_phase() calls process_dotnet_projects()`
- c0102 ..> c0102: `run_imports_phase() calls process_source_imports()`
- c0102 ..> c0102: `run_imports_phase() calls register_observed_namespaces()`
- c0103 ..> c0004: `run_parsing_phase() calls default()`
- c0103 ..> c0004: `type in run_parsing_phase`
- c0103 ..> c0028: `run_parsing_phase() calls as_str()`
- c0103 ..> c0040: `run_parsing_phase() calls new()`
- c0103 ..> c0040: `run_parsing_phase() calls register()`
- c0103 ..> c0048: `run_parsing_phase() calls add_symbol()`
- c0103 ..> c0048: `run_parsing_phase() calls get_files()`
- c0103 ..> c0048: `type in run_parsing_phase`
- c0103 ..> c0052: `type in run_parsing_phase`
- c0103 ..> c0056: `run_parsing_phase() calls add()`
- c0103 ..> c0056: `type in run_parsing_phase`
- c0103 ..> c0058: `run_parsing_phase() calls extract_symbols()`
- c0103 ..> c0058: `run_parsing_phase() calls language_name()`
- c0103 ..> c0067: `run_parsing_phase() calls extend()`
- c0103 ..> c0069: `run_parsing_phase() calls finish()`
- c0103 ..> c0072: `run_parsing_phase() calls extract()`
- c0103 ..> c0081: `run_parsing_phase() calls get_by_extension()`
- c0103 ..> c0086: `run_parsing_phase() calls get_language_for_ext()`
- c0104 ..> c0004: `type in run_processes_phase`
- c0104 ..> c0028: `classify_process() calls as_str()`
- c0104 ..> c0028: `deduplicate() calls as_str()`
- c0104 ..> c0040: `bfs_traces() calls new()`
- c0104 ..> c0048: `bfs_traces() calls get_callees()`
- c0104 ..> c0048: `build_community_map() calls get_communities()`
- c0104 ..> c0048: `compute_total_confidence() calls get_callees()`
- c0104 ..> c0048: `run_processes_phase() calls add_process()`
- c0104 ..> c0048: `type in bfs_traces`
- c0104 ..> c0048: `type in build_community_map`
- c0104 ..> c0048: `type in compute_total_confidence`
- c0104 ..> c0048: `type in run_processes_phase`
- c0104 ..> c0054: `run_processes_phase() calls score_entry_points()`
- c0104 ..> c0067: `run_processes_phase() calls extend()`
- c0104 ..> c0104: `run_processes_phase() calls bfs_traces()`
- c0104 ..> c0104: `run_processes_phase() calls build_community_map()`
- c0104 ..> c0104: `run_processes_phase() calls classify_process()`
- c0104 ..> c0104: `run_processes_phase() calls compute_total_confidence()`
- c0104 ..> c0104: `run_processes_phase() calls deduplicate()`
- c0104 ..> c0104: `run_processes_phase() calls sort_key()`
- c0105 ..> c0004: `type in run_structure_phase`
- c0105 ..> c0028: `run_structure_phase() calls as_str()`
- c0105 ..> c0040: `run_structure_phase() calls new()`
- c0105 ..> c0048: `run_structure_phase() calls add_file()`
- c0105 ..> c0048: `run_structure_phase() calls add_folder()`
- c0105 ..> c0048: `type in run_structure_phase`
- c0105 ..> c0081: `run_structure_phase() calls language_for_extension()`
- c0106 ..> c0004: `type in run_pipeline`
- c0106 ..> c0005: `type in run_pipeline`
- c0106 ..> c0040: `run_pipeline() calls new()`
- c0106 ..> c0098: `run_pipeline() calls build_result()`
- c0123 ..> c0072: `export_mermaid() calls extract()`
- c0123 ..> c0094: `export_mermaid() calls from_str()`
- c0123 ..> c0095: `export_mermaid() calls export_mermaid_report()`
- c0123 ..> c0106: `analyze() calls run_pipeline()`
- c0123 ..> c0122: `type in analyze`
- c0124 ..> c0124: `analyze_cmd() calls _run_quiet()`
- c0124 ..> c0124: `analyze_cmd() calls _run_with_progress()`
- c0254 ..> c0040: `main() calls new()`
- c0256 ..> c0256: `commaSep() calls commaSep1()`
- c0256 ..> c0256: `kw() calls ci()`
- c0263 --> c0262: `field field_map_entries`
- c0263 --> c0264: `field metadata`
- c0263 --> c0266: `field external_scanner`
- c0263 --> c0267: `field lex_modes`
- c0263 --> c0268: `field field_map_slices`
- c0263 --> c0268: `field supertype_map_slices`
- c0263 --> c0270: `field parse_actions`
- c0263 --> c0272: `field symbol_metadata`
- c0270 --> c0269: `field action`
- c0273 ..> c0261: `type in set_contains`

## Type key

- Type1: `{ input: PathBuf, #[arg(short, long)] output: PathBuf, #[arg(long, default_value =
  "mermaid", value_parser = ["mermaid"])] format: String, /// Repository-relative file or directory
  to include #[arg(long, default_value = "")] path: String, /// Maximum boxes in each diagram
  #[arg(long, default_value_t = 8)] max_classes: usize, /// Hide recognised tests by default;
  include preserves the complete view #[arg(long, default_value = "exclude", value_parser =
  ["exclude", "include"])] tests: String, /// Repository-relative test file or directory
  (repeatable) #[arg(long)] test_path: Vec<String>, /// Keep a file or directory, overriding test
  detection (repeatable) #[arg(long)] keep_path: Vec<String>, /// Include a collapsed list of test
  selection reasons #[arg(long)] explain_tests: bool, }`
- Type2: `{ /// Path to the repository to analyse path: PathBuf, /// Output JSON file path
  #[arg(short, long)] output: Option<String>, /// Comma-separated language filter #[arg(short,
  long)] languages: Option<String>, /// Louvain resolution parameter #[arg(long, default_value =
  "1.0")] resolution: f64, /// Maximum execution flows to detect #[arg(long, default_value = "75")]
  max_processes: usize, /// Maximum BFS trace depth #[arg(long, default_value = "10")] max_depth:
  usize, /// Additional glob patterns to exclude #[arg(long)] exclude: Vec<String>, /// Show
  per-phase timing breakdown #[arg(long)] verbose: bool, /// Suppress all output except errors
  #[arg(long)] quiet: bool, }`
- Type3: `()`
- Type4: `&AnalysisConfig`
- Type5: `&str`
- Type6: `Result<(), Box<dyn std::error::Error>>`
- Type7: `&std::path::Path`
- Type8: `&mycelium_core::mermaid::MermaidOptions`
- Type9: `Option<crate::declarations::ClassDiagram>`
- Type10: `HashMap<String, serde_json::Value>`
- Type11: `Option<(usize, usize)>`
- Type12: `Option<Vec<(String, String)>>`
- Type13: `&'static str`
- Type14: `&mut std::fmt::Formatter<'_>`
- Type15: `HashMap<String, String>`
- Type16: `Option<&str>`
- Type17: `&HashMap<String, String>`
- Type18: `Vec<(String, String)>`
- Type19: `{ statement: String, }`
- Type20: `{ confidence: f64, tier: String, reason: String, line: usize, }`
- Type21: `{ ref_type: String, }`
- Type22: `{ version: String, }`
- Type23: `{ order: usize, }`
- Type24: `DiGraph<NodeData, EdgeData>`
- Type25: `HashMap<String, NodeIndex>`
- Type26: `Option<&NodeData>`
- Type27: `&FileNode`
- Type28: `&FolderNode`
- Type29: `&Symbol`
- Type30: `&CallEdge`
- Type31: `&ImportEdge`
- Type32: `&ProjectReference`
- Type33: `&PackageReference`
- Type34: `&Community`
- Type35: `&Process`
- Type36: `Vec<&NodeData>`
- Type37: `Vec<(String, String, f64, String, String, usize)>`
- Type38: `Vec<(String, String, String)>`
- Type39: `Vec<(String, String, Vec<String>, f64, String)>`
- Type40: `Vec<(String, String, String, Vec<String>, String, f64)>`
- Type41: `&DiGraph<NodeData, EdgeData>`
- Type42: `&HashMap<String, NodeIndex>`
- Type43: `{ path: String, language: Option<String>, size: u64, lines: usize, }`
- Type44: `{ path: String, file_count: usize, }`
- Type45: `{ id: String, name: String, symbol_type: String, file: String, line: usize, visibility:
  String, exported: bool, parent: Option<String>, language: Option<String>, parameter_types:
  Option<Vec<(String, String)>>, }`
- Type46: `{ id: String, label: String, cohesion: f64, primary_language: String, }`
- Type47: `{ id: String, entry: String, terminal: String, process_type: String, total_confidence:
  f64, }`
- Type48: `{ name: String, }`
- Type49: `HashMap<String, Vec<String>>`
- Type50: `&[String]`
- Type51: `&KnowledgeGraph`
- Type52: `Vec<(String, f64)>`
- Type53: `HashMap<String, HashMap<String, String>>`
- Type54: `HashMap<String, Vec<SymbolDefinition>>`
- Type55: `&[SymbolDefinition]`
- Type56: `Option<&HashMap<String, String>>`
- Type57: `&HashMap<String, HashMap<String, String>>`
- Type58: `&HashMap<String, Vec<SymbolDefinition>>`
- Type59: `&[&str]`
- Type60: `&Tree`
- Type61: `&[u8]`
- Type62: `&HashSet<String>`
- Type63: `&Node`
- Type64: `&mut Vec<Symbol>`
- Type65: `&mut Vec<RawCall>`
- Type66: `(Option<String>, Option<String>)`
- Type67: `Node<'_>`
- Type68: `&mut ClassDiagram`
- Type69: `Option<Node<'_>>`
- Type70: `&mut Detection`
- Type71: `&mut BTreeSet<String>`
- Type72: `Option<&'static str>`
- Type73: `Vec<(TestEvidence, BTreeSet<String>)>`
- Type74: `BTreeMap<usize, TestEvidence>`
- Type75: `Node<'tree>`
- Type76: `&mut impl FnMut(Node<'tree>)`
- Type77: `&mut crate::declarations::ClassDiagram`
- Type78: `&mut std::collections::BTreeSet<String>`
- Type79: `&mut frameworks::Bindings`
- Type80: `Vec<Node<'_>>`
- Type81: `&mut test_detection::RustDetection`
- Type82: `&frameworks::Detection`
- Type83: `&mut RustDetection`
- Type84: `HashMap<String, usize>`
- Type85: `Option<&dyn LanguageAnalyser>`
- Type86: `Vec<&str>`
- Type87: `*const ()`
- Type88: `&'a Class`
- Type89: `Option<&'a Member>`
- Type90: `&'a str`
- Type91: `(&'a str, &'a str, usize, &'a str)`
- Type92: `(String)`
- Type93: `&mut fmt::Formatter<'_>`
- Type94: `Result<Self, Self::Err>`
- Type95: `Result<String, ExportError>`
- Type96: `&AnalysisResult`
- Type97: `&MermaidOptions`
- Type98: `Result<MermaidExport, ExportError>`
- Type99: `&[Class]`
- Type100: `&BTreeSet<&str>`
- Type101: `BTreeMap<String, Vec<usize>>`
- Type102: `&BTreeMap<String, Vec<usize>>`
- Type103: `&Member`
- Type104: `&mut BTreeMap<String, String>`
- Type105: `Vec<Vec<(usize, &[Member])>>`
- Type106: `BTreeMap<String, String>`
- Type107: `BTreeMap<(String, String), usize>`
- Type108: `Result<Self, ExportError>`
- Type109: `Option<&TestEvidence>`
- Type110: `&BTreeSet<String>`
- Type111: `&SymbolTable`
- Type112: `&HashMap<String, f64>`
- Type113: `std::io::Result<()>`
- Type114: `&mut KnowledgeGraph`
- Type115: `&mut SymbolTable`
- Type116: `&mut NamespaceIndex`
- Type117: `Option<&'a str>`
- Type118: `&'a SymbolTable`
- Type119: `&HashMap<String, Vec<String>>`
- Type120: `&crate::config::RawCall`
- Type121: `Vec<Vec<(usize, f64)>>`
- Type122: `&AdjList`
- Type123: `&mut AssemblyIndex`
- Type124: `&AssemblyIndex`
- Type125: `(f64, usize)`
- Type126: `Result<AnalysisResult, Box<dyn std::error::Error>>`
- Type127: `Python<'_>`
- Type128: `&Bound<'_, PyDict>`
- Type129: `PyResult<()>`
- Type130: `&Bound<'_, PyModule>`
- Type131: `str | None`
- Type132: `tuple[str, ...]`
- Type133: `void*`
- Type134: `uint32_t*`
- Type135: `const void*`
- Type136: `const uint16_t*`
- Type137: `const uint32_t*`
- Type138: `const TSParseActionEntry*`
- Type139: `const char**`
- Type140: `const TSMapSlice*`
- Type141: `const TSFieldMapEntry*`
- Type142: `const TSSymbolMetadata*`
- Type143: `const TSSymbol*`
- Type144: `const TSLexerMode*`
- Type145: `struct { const bool *states; const TSSymbol *symbol_map; void *(*create)(void); void
  (*destroy)(void *); bool (*scan)(void *, TSLexer *, const bool *symbol_whitelist); unsigned
  (*serialize)(void *, char *); void (*deserialize)(void *, const char *, unsigned); }`
- Type146: `const bool*`
- Type147: `const TSStateId*`
- Type148: `const char*`
- Type149: `struct { uint8_t type; TSStateId state; bool extra; bool repetition; }`
- Type150: `struct { uint8_t type; uint8_t child_count; TSSymbol symbol; int16_t dynamic_precedence;
  uint16_t production_id; }`
- Type151: `struct { uint8_t count; bool reusable; }`
- Type152: `const TSCharacterRange*`

## Signature key

- Signature1: `-extract_cpp_symbols(node: &Node, source: &[u8], file_path: &str, symbols: &mut
  Vec<Symbol>, parent_id: Option<&str>) ()`
- Signature2: `-extract_c_symbols(node: &Node, source: &[u8], file_path: &str, symbols: &mut
  Vec<Symbol>, parent_id: Option<&str>, lang: &str) ()`
- Signature3: `-find_c_calls(node: &Node, source: &[u8], file_path: &str, calls: &mut Vec<RawCall>,
  exclusions: &HashSet<String>) ()`
- Signature4: `-walk_node(node: &Node, source: &[u8], file_path: &str, symbols: &mut Vec<Symbol>,
  parent_id: Option<&str>) ()`
- Signature5: `-find_calls(node: &Node, source: &[u8], file_path: &str, calls: &mut Vec<RawCall>,
  exclusions: &HashSet<String>) ()`
- Signature6: `+walk(node: Node<'_>, source: &[u8], file: &str, owner: Option<usize>, diagram: &mut
  ClassDiagram) ()`
- Signature7: `+extract(tree: &Tree, source: &[u8], file: &str, language: &str, bindings: &mut
  frameworks::Bindings) ClassDiagram`
- Signature8: `-rust_walk(node: Node<'_>, source: &[u8], file: &str, owner: Option<usize>, diagram:
  &mut ClassDiagram, detection: &mut test_detection::RustDetection) ()`
- Signature9: `+walk(node: Node<'_>, source: &[u8], file: &str, owner: Option<usize>, diagram: &mut
  ClassDiagram, detection: &frameworks::Detection) ()`
- Signature10: `-add_field(node: Node<'_>, source: &[u8], index: usize, name: String, value_type:
  Option<String>, diagram: &mut ClassDiagram, detection: &frameworks::Detection) ()`
- Signature11: `-push_field(node: Node<'_>, source: &[u8], index: usize, name: String, diagram: &mut
  ClassDiagram, detection: &frameworks::Detection) ()`
- Signature12: `-instance_fields(node: Node<'_>, source: &[u8], index: usize, diagram: &mut
  ClassDiagram, detection: &frameworks::Detection) ()`
- Signature13: `+rust_evidence(node: Node<'_>, source: &[u8], file: &str, detection: &mut
  RustDetection) Option<TestEvidence>`
- Signature14: `-extract_class_members(body_node: &Node, file_path: &str, source: &[u8], symbols:
  &mut Vec<Symbol>, parent_name: &str, lang: &str) ()`
- Signature15: `-type_edges(classes: &[Class], all_classes: &[Class], hidden_ids: &BTreeSet<&str>,
  warnings: &mut BTreeSet<String>) BTreeSet<DiagramEdge>`
- Signature16: `+build_result(config: &AnalysisConfig, kg: &KnowledgeGraph, _st: &SymbolTable,
  timings: &HashMap<String, f64>, total_ms: f64) AnalysisResult`
- Signature17: `-call_target_in_file(st: &'a SymbolTable, kg: &KnowledgeGraph, source_id: &str,
  file: &str, name: &str) Option<&'a str>`
- Signature18: `-is_interface_self_call(caller_name: &str, callee_name: &str, target_id: &str, kg:
  &KnowledgeGraph) bool`
- Signature19: `-find_implementation(callee_name: &str, interface_target_id: &str, st: &SymbolTable,
  import_map: &HashMap<String, Vec<String>>, file_path: &str, kg: &KnowledgeGraph) Option<String>`
- Signature20: `-resolve_call(raw_call: &crate::config::RawCall, file_path: &str, st: &SymbolTable,
  import_map: &HashMap<String, Vec<String>>, kg: &KnowledgeGraph, field_type_map: &HashMap<String,
  String>) Option<CallEdge>`
- Signature21: `-disambiguate_label(label: &str, members: &[String], kg: &KnowledgeGraph,
  used_labels: &HashSet<String>) String`
- Signature22: `-process_source_imports(config: &AnalysisConfig, kg: &mut KnowledgeGraph, st: &mut
  SymbolTable, assembly_index: &AssemblyIndex, ns_index: &mut NamespaceIndex) ()`
- Signature23: `-resolve_python_import(target_name: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature24: `-resolve_python_relative(target_name: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature25: `-resolve_ts_import(target_name: &str, source_file: &str, file_set: &HashSet<String>)
  Option<String>`
- Signature26: `-resolve_java_import(target_name: &str, source_file: &str, file_set:
  &HashSet<String>, basename_index: &HashMap<String, Vec<String>>) Option<String>`
- Signature27: `-resolve_go_import(target_name: &str, go_module: Option<&str>, go_dir_index:
  &HashMap<String, Vec<String>>) Vec<String>`
- Signature28: `-resolve_rust_import(target_name: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature29: `-resolve_c_include(target_name: &str, statement: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature30: `-resolve_fallback(target_name: &str, _source_file: &str, st: &SymbolTable,
  assembly_index: &AssemblyIndex, kg: &KnowledgeGraph) Option<String>`
- Signature31: `-bfs_traces(kg: &KnowledgeGraph, start: &str, max_depth: usize, max_branching:
  usize, min_steps: usize) Vec<Vec<String>>`
- Signature32: `-new(repo_path: String, output_path: Option<String>, languages: Option<Vec<String>>,
  resolution: f64, max_processes: usize, max_depth: usize, max_branching: usize, min_steps: usize,
  exclude_patterns: Vec<String>, verbose: bool, quiet: bool, max_file_size: u64, max_community_size:
  usize) Self`
- Signature33: `-analyze(py: Python<'_>, path: &str, config: Option<PyAnalysisConfig>, progress:
  Option<PyObject>) PyResult<Py<PyDict>>`
- Signature34: `-export_mermaid(py: Python<'_>, result: &Bound<'_, PyDict>, path: &str, max_classes:
  usize, tests: &str, test_paths: Option<Vec<String>>, keep_paths: Option<Vec<String>>,
  explain_tests: bool) PyResult<String>`
- Signature35: `+export_cmd(input_path: unknown, output_path: unknown, output_format: unknown, path:
  unknown, max_classes: unknown, tests: unknown, test_paths: unknown, keep_paths: unknown,
  explain_tests: unknown) unknown`
- Signature36: `+analyze_cmd(path: str, output_path: str | None, languages: str | None, resolution:
  float, max_processes: int, max_depth: int, exclude: tuple[str, ...], verbose: bool, quiet: bool)
  None`
- Signature37: `_array__erase(self_contents: void*, size: uint32_t*, element_size: size_t, index:
  uint32_t) void`
- Signature38: `_array__reserve(contents: void*, capacity: uint32_t*, element_size: size_t,
  new_capacity: uint32_t) void*`
- Signature39: `_array__assign(self_contents: void*, self_size: uint32_t*, self_capacity: uint32_t*,
  other_contents: const void*, other_size: uint32_t, element_size: size_t) void*`
- Signature40: `_array__swap(self_size: uint32_t*, self_capacity: uint32_t*, other_size: uint32_t*,
  other_capacity: uint32_t*) void`
- Signature41: `_array__grow(contents: void*, size: uint32_t, capacity: uint32_t*, count: uint32_t,
  element_size: size_t) void*`
- Signature42: `_array__splice(self_contents: void*, size: uint32_t*, capacity: uint32_t*,
  element_size: size_t, index: uint32_t, old_count: uint32_t, new_count: uint32_t, elements: const
  void*) void*`

## Extraction warnings

- `Ambiguous type: Commands uses Repository`
- `Unresolved or out-of-scope base: AnalyserRegistry -> Default`
- `Unresolved or out-of-scope base: AnalysisConfig -> Default`
- `Unresolved or out-of-scope base: AnalysisConfig -> From<PyAnalysisConfig>`
- `Unresolved or out-of-scope base: AnalysisResult -> Default`
- `Unresolved or out-of-scope base: AssemblyIndex -> Default`
- `Unresolved or out-of-scope base: BdistWheel -> bdist_wheel`
- `Unresolved or out-of-scope base: Build -> build`
- `Unresolved or out-of-scope base: CAnalyser -> Default`
- `Unresolved or out-of-scope base: CSharpAnalyser -> Default`
- `Unresolved or out-of-scope base: CppAnalyser -> Default`
- `Unresolved or out-of-scope base: EggInfo -> egg_info`
- `Unresolved or out-of-scope base: ExportError -> fmt::Display`
- `Unresolved or out-of-scope base: ExportError -> std::error::Error`
- `Unresolved or out-of-scope base: GoAnalyser -> Default`
- `Unresolved or out-of-scope base: JavaAnalyser -> Default`
- `Unresolved or out-of-scope base: KnowledgeGraph -> Default`
- `Unresolved or out-of-scope base: MermaidOptions -> Default`
- `Unresolved or out-of-scope base: NamespaceIndex -> Default`
- `Unresolved or out-of-scope base: PythonAnalyser -> Default`
- `Unresolved or out-of-scope base: RustAnalyser -> Default`
- `Unresolved or out-of-scope base: SymbolTable -> Default`
- `Unresolved or out-of-scope base: SymbolType -> std::fmt::Display`
- `Unresolved or out-of-scope base: TestDetection -> Default`
- `Unresolved or out-of-scope base: TestMode -> std::str::FromStr`
- `Unresolved or out-of-scope base: TypeScriptAnalyser -> Default`
- `Unresolved or out-of-scope base: VbNetAnalyser -> Default`
- `Unresolved or out-of-scope base: Visibility -> std::fmt::Display`
- `crates/mycelium-core/src/mermaid.rs: syntax errors; declarations may be incomplete`
- `tests/fixtures/vbnet_simple/EmployeeModule.vb: syntax errors; declarations may be incomplete`
- `tests/fixtures/vbnet_simple/EmployeeRepository.vb: syntax errors; declarations may be incomplete`
- `vendor/tree-sitter-vb-dotnet/src/tree_sitter/alloc.h: syntax errors; declarations may be
  incomplete`
- `vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h: syntax errors; declarations may be
  incomplete`
- `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h: syntax errors; declarations may be
  incomplete`
