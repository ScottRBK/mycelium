# Mermaid class diagrams

Declared types and signatures; unknown types are `unknown`. Receivers are omitted.
Fields are associations, not lifetime ownership. Calls are static heuristic estimates.
Members and connections within each view are uncapped.
Parallel arrows are summarized.
Cross-diagram relationships are retained in the complete relationship list.

Included: 129 boxes. Calls without in-scope endpoints: 0.

## Test filtering

Mode: exclude.
Saved detector version: 2.
Test paths: `crates/mycelium-cli/tests`, `crates/mycelium-core/tests`, `tests`.
Keep paths: none.
Calls removed by test filtering: 1397. Type relationships removed: 197.

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
- `crates/mycelium-core/tests/test_python_type_bindings.rs`: 23 source occurrences (test-path).
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
        +python_bindings: Type15
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
    class c0036["PythonBinding"] {
        <<enum>>
        -Import: Type16
        -Module: Type16
        -Modules: Type17
        -Class: Type18
        -Unknown: unknown
    }
    class c0037["PythonBindings"] {
        <<struct>>
        +names: Type19
        +uncertain: bool
    }
    class c0038["TestDetection"] {
        <<struct>>
        +version: u32
        +diagnostics: Vec~TestDiagnostic~
        +default() Self
    }
    class c0039["TestDiagnostic"] {
        <<struct>>
        +file: String
        +message: String
    }
    class c0040["TestEvidence"] {
        <<struct>>
        +rule: String
        +file: String
        +line: usize
    }
    class c0041["crates/mycelium-core/src/declarations.rs"] {
        <<module>>
        +supported_test_rule(version: u32, rule: Type5) bool
    }
    class c0042["AssemblyIndex"] {
        <<struct>>
        -ns_to_project: Type20
        +new() Self
        +register(namespace: Type5, project: Type5) Type3
        +resolve_namespace(namespace: Type5) Type21
        +get_all_namespaces() Type22
        +default() Self
    }
    class c0044["ProjectFile"] {
        <<struct>>
        +name: String
        +target_framework: Option~String~
        +root_namespace: Option~String~
        +assembly_name: Option~String~
        +project_references: Vec~String~
        +package_references: Type23
    }
    class c0045["crates/mycelium-core/src/dotnet/project.rs"] {
        <<module>>
        +parse_project_file(content: Type5, project_path: Type5) ProjectFile
        -extract_element_text(content: Type5, tag: Type5) Option~String~
        -extract_include_attrs(content: Type5, tag: Type5) Vec~String~
        -extract_package_refs(content: Type5) Type23
        -extract_attr(element: Type5, attr: Type5) Option~String~
    }
    class c0046["SlnProject"] {
        <<struct>>
        +name: String
        +path: String
        +project_type_guid: String
        +project_guid: String
    }
    class c0047["crates/mycelium-core/src/dotnet/solution.rs"] {
        <<module>>
        +parse_solution(content: Type5) Vec~SlnProject~
    }
    class c0048["CallInfo"] {
        <<struct>>
        +id: String
        +confidence: f64
        +tier: String
        +reason: String
        +line: usize
    }
    class c0049["EdgeData"] {
        <<enum>>
        -Defines: unknown
        -Imports: Type24
        -Calls: Type25
        -ProjectReference: Type26
        -PackageReference: Type27
        -MemberOf: unknown
        -Step: Type28
        -Contains: unknown
        +edge_type() Type13
    }
    class c0050["KnowledgeGraph"] {
        <<struct>>
        +class_diagram: crate::declarations::ClassDiagram
        -graph: Type29
        -id_index: Type30
        +new() Self
        -ensure_node(id: Type5, data: NodeData) NodeIndex
        +get_node_index(id: Type5) Option~NodeIndex~
        +get_node_data(id: Type5) Type31
        +has_node(id: Type5) bool
        +add_file(node: Type32) Type3
        +add_folder(node: Type33) Type3
        +add_symbol(symbol: Type34) Type3
        +add_call(edge: Type35) Type3
        +add_import(edge: Type36) Type3
        +add_project_reference(reference: Type37) Type3
        +add_package_reference(reference: Type38) Type3
        +add_community(community: Type39) Type3
        +add_process(process: Type40) Type3
        +get_files() Type41
        +get_folders() Type41
        +get_symbols() Vec~SymbolInfo~
        +get_symbols_in_file(path: Type5) Vec~SymbolInfo~
        +get_callers(symbol_id: Type5) Vec~CallInfo~
        +get_callees(symbol_id: Type5) Vec~CallInfo~
        +get_call_edges() Type42
        +get_import_edges() Type43
        +get_project_references() Type43
        +get_package_references() Type43
        +get_communities() Type44
        +get_processes() Type45
        +symbol_count() usize
        +file_count() usize
        +folder_count() usize
        -node_id(idx: NodeIndex) Option~String~
        +inner_graph() Type46
        +id_index() Type47
        +default() Self
    }
    class c0051["NodeData"] {
        <<enum>>
        -File: Type48
        -Folder: Type49
        -Symbol: Type50
        -Community: Type51
        -Process: Type52
        -Package: Type53
        -Project: Type53
        +node_type() Type13
    }
    class c0052["SymbolInfo"] {
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
    class c0054["NamespaceIndex"] {
        <<struct>>
        -ns_to_files: Type54
        -file_to_ns: Type54
        -file_imports: Type54
        +new() Self
        +register(namespace: Type5, file_path: Type5) Type3
        +get_files_for_namespace(namespace: Type5) Type55
        +register_file_import(file_path: Type5, namespace: Type5) Type3
        +get_imported_namespaces(file_path: Type5) Type55
        +get_namespaces_for_file(file_path: Type5) Type55
        +default() Self
    }
    class c0056["crates/mycelium-core/src/graph/scoring.rs"] {
        <<module>>
        -probe_depth(kg: Type56, sym_id: Type5, max_hops: usize) usize
        +score_entry_points(kg: Type56) Type57
    }
    class c0057["SymbolDefinition"] {
        <<struct>>
        +symbol_id: String
        +name: String
        +file: String
        +symbol_type: String
        +language: Option~String~
        +parent: Option~String~
    }
    class c0058["SymbolTable"] {
        <<struct>>
        -file_index: Type58
        -global_index: Type59
        +new() Self
        +add(symbol: Type34) Type3
        +lookup_exact(file_path: Type5, name: Type5) Type21
        +lookup_fuzzy(name: Type5) Type60
        +get_symbols_in_file(file_path: Type5) Type61
        +file_index() Type62
        +global_index() Type63
        +default() Self
    }
    class c0060["CAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0061["CppAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -extract_cpp_symbols(Signature1)
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0062["crates/mycelium-core/src/languages/c_cpp.rs"] {
        <<module>>
        -is_preproc_container(kind: Type5) bool
        -get_func_name(node: Type68, source: Type66) Option~String~
        -get_qualified_func_name(node: Type68, source: Type66) Option~String~
        -get_type_name(node: Type68, source: Type66) Option~String~
        -extract_c_symbols(Signature2)
        -extract_includes(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        -find_c_calls(Signature3)
        -extract_c_callee(node: Type68, source: Type66) Type71
        -find_enclosing_func(node: Type68, source: Type66) Option~String~
    }
    class c0063["CSharpAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -walk_node(Signature4)
        -extract_using(node: Type68, source: Type66, file_path: Type5) Option~ImportStatement~
        -find_calls(Signature5)
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0064["crates/mycelium-core/src/languages/csharp.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
        -is_container(node_type: Type5) bool
        -get_visibility(node: Type68, source: Type66) Visibility
        -get_name(node: Type68, source: Type66) Option~String~
        -extract_parameter_types(node: Type68, source: Type66) Type12
        -extract_callee(inv_node: Type68, source: Type66) Type71
        -find_enclosing_method(node: Type68, source: Type66) Option~String~
    }
    class c0065["crates/mycelium-core/src/languages/declarations/c_cpp.rs"] {
        <<module>>
        +walk(Signature6)
        -find_function(node: Type72) Type74
        -declarator_name(node: Type72, source: Type66) Option~String~
        -declaration_type(node: Type72, declarator: Type74, source: Type66) Option~String~
    }
    class c0066["crates/mycelium-core/src/languages/declarations/frameworks/attributes.rs"] {
        <<module>>
        +detect(root: Type72, source: Type66, file: Type5, detection: Type75, vb: bool) Type3
        -guard_name(guards: Type76, name: Type5) Type3
        -key(name: Type5) String
        +type_node(node: Type72) bool
        +scope(root: Type72, node: Type72, source: Type66) String
        -scope_inner(root: Type72, node: Type72, source: Type66, omit_modules: bool) String
        -ancestor(parent: Type72, node: Type72) bool
        +inherited_scope(node: Type72) bool
        -clean(name: Type5) String
        -rule(name: Type5, container: bool, vb: bool) Type77
    }
    class c0067["crates/mycelium-core/src/languages/declarations/frameworks/java.rs"] {
        <<module>>
        +detect(root: Type72, source: Type66, file: Type5, detection: Type75) Type3
    }
    class c0068["crates/mycelium-core/src/languages/declarations/frameworks/javascript.rs"] {
        <<module>>
        +detect(root: Type72, source: Type66, file: Type5, detection: Type75) Type3
        -inline(node: Type72) bool
        -literal(node: Type72, source: Type66) Option~String~
        -bound_names(node: Type72, source: Type66, names: Type76) Type3
    }
    class c0069["Bindings"] {
        <<struct>>
        -declared: BTreeSet~String~
        -guards: Type78
        +extend(other: Self) Type3
    }
    class c0070["Detection"] {
        <<struct>>
        -marks: Type79
        +bindings: Bindings
        +syntax_valid: bool
        +new(root: Type72, source: Type66, file: Type5, language: Type5) Self
        -mark(node: Type72, marker: Type72, file: Type5, rule: Type5) Type3
        +evidence(node: Type72) Option~TestEvidence~
    }
    class c0071["crates/mycelium-core/src/languages/declarations/frameworks/mod.rs"] {
        <<module>>
        -visit(node: Type80, callback: Type81) Type3
        +finish(diagram: Type82, files: Type55, bindings: Bindings) Type3
        -keyword(node: Type72, name: Type5) bool
    }
    class c0072["crates/mycelium-core/src/languages/declarations/frameworks/python.rs"] {
        <<module>>
        +detect(root: Type72, source: Type66, file: Type5, detection: Type75) Type3
        -bound_names(node: Type72, source: Type66, names: Type83) Type3
    }
    class c0073["crates/mycelium-core/src/languages/declarations/go.rs"] {
        <<module>>
        +walk(Signature6)
    }
    class c0074["crates/mycelium-core/src/languages/declarations/mod.rs"] {
        <<module>>
        +extract(Signature7)
        -text(node: Type72, source: Type66) String
        -field(node: Type72, name: Type5, source: Type66) Option~String~
        -children(node: Type72) Type85
        -add_class(diagram: Type73, file: Type5, name: String, kind: Type5, line: usize) usize
        -module(diagram: Type73, file: Type5) usize
        -rust_walk(Signature8)
        -enum_member(node: Type72, owner: usize, name: String, diagram: Type73) Type3
    }
    class c0075["crates/mycelium-core/src/languages/declarations/nominal.rs"] {
        <<module>>
        +walk(Signature9)
        -visibility(node: Type72, source: Type66, public_default: bool) String
        -add_field(Signature10)
        -type_field(node: Type72, name: Type5, source: Type66) Option~String~
        -base_types(node: Type72) Type85
    }
    class c0076["crates/mycelium-core/src/languages/declarations/python.rs"] {
        <<module>>
        +bindings(root: Type72, source: Type66, file: Type5) PythonBindings
        -import_bindings(node: Type72, source: Type66, file: Type5, bindings: Type88) Type3
        -typing_guard(node: Type72, source: Type66, bindings: Type89) bool
        -uncertain_bindings(Signature11)
        -block_bound_names(node: Type72, source: Type66, bindings: Type88) Type3
        -import_target(module: Type5, name: Type5, file: Type5) Option~String~
        -insert_binding(bindings: Type88, name: String, value: PythonBinding) Type3
        +walk(Signature9)
        -push_field(Signature12)
        -instance_fields(Signature13)
    }
    class c0077["RustDetection"] {
        <<struct>>
        +syntax_valid: bool
        +uncertain_test_binding: bool
        +skipped_test: bool
        +new(root: Type72, source: Type66) Self
    }
    class c0078["crates/mycelium-core/src/languages/declarations/test_detection.rs"] {
        <<module>>
        +malformed(node: Type72) bool
        -uncertain_import(node: Type72, source: Type66) bool
        -binds_test(node: Type72, source: Type66) bool
        +rust_evidence(Signature14)
        -evidence(rule: Type5, node: Type72, file: Type5) TestEvidence
        -tokens(node: Type72, source: Type66) String
    }
    class c0079["crates/mycelium-core/src/languages/declarations/vbnet.rs"] {
        <<module>>
        +walk(Signature9)
        -vb_type(node: Type72, source: Type66) Option~String~
    }
    class c0080["GoAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_name_by_kind(node: Type68, target_kind: Type5, source: Type66) Option~String~
        -is_exported(name: Type5) bool
        -extract_string(node: Type68, source: Type66) Option~String~
        -extract_string_content(node: Type68, source: Type66) Option~String~
        -find_calls(Signature5)
        -extract_callee(node: Type68, source: Type66) Type71
        -find_enclosing(node: Type68, source: Type66) Option~String~
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0081["JavaAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -walk_node(Signature4)
        -find_calls(Signature5)
        -extract_callee(node: Type68, source: Type66) Type71
        -find_enclosing(node: Type68, source: Type66) Option~String~
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0082["crates/mycelium-core/src/languages/java.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
        -is_container(node_type: Type5) bool
        -get_visibility(node: Type68, source: Type66) Visibility
        -get_name(node: Type68, source: Type66) Option~String~
    }
    class c0083["AnalyserRegistry"] {
        <<struct>>
        -analysers: Vec~Box~dyn LanguageAnalyser~~
        -extension_map: Type92
        +new() Self
        +get_by_extension(ext: Type5) Type93
        +language_for_extension(ext: Type5) Type21
        +extensions() Type94
        +default() Self
    }
    class c0084["LanguageAnalyser"] {
        <<trait>>
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
        +get_language_for_ext(_ext: Type5) Language
        +is_available() bool
    }
    class c0085["PythonAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_name(node: Type68, source: Type66) Option~String~
        -walk_node(Signature4)
        -find_calls(Signature5)
        -extract_callee(node: Type68, source: Type66) Type71
        -find_enclosing(node: Type68, source: Type66) Option~String~
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0086["RustAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_name(node: Type68, source: Type66) Option~String~
        -is_pub(node: Type68) bool
        -walk_node(Signature4)
        -find_calls(Signature5)
        -extract_callee(node: Type68, source: Type66) Type71
        -find_enclosing(node: Type68, source: Type66) Option~String~
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0087["crates/mycelium-core/src/languages/rust_lang.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
    }
    class c0088["TypeScriptAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -get_ts_language() Language
        -get_tsx_language() Language
        -get_js_language() Language
        -language_for_path(file_path: Type5) Type13
        -get_name(node: Type68, source: Type66) Option~String~
        -walk_node(Signature4)
        -extract_class_members(Signature15)
        -extract_string_source(node: Type68, source: Type66) Option~String~
        -find_calls(Signature5)
        -extract_callee(node: Type68, source: Type66) Type71
        -find_enclosing(node: Type68, source: Type66) Option~String~
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +get_language_for_ext(ext: Type5) Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
    }
    class c0089["crates/mycelium-core/src/languages/typescript.rs"] {
        <<module>>
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
    }
    class c0090["VbNetAnalyser"] {
        <<struct>>
        +default() Self
        +new() Self
        -walk_node(Signature4)
        -find_calls(Signature5)
        +extensions() Type64
        +language_name() Type5
        +get_language() Language
        +extract_symbols(tree: Type65, source: Type66, file_path: Type5) Vec~Symbol~
        +extract_imports(tree: Type65, source: Type66, file_path: Type5) Vec~ImportStatement~
        +extract_calls(tree: Type65, source: Type66, file_path: Type5) Vec~RawCall~
        +builtin_exclusions() Type67
        +is_available() bool
    }
    class c0091["crates/mycelium-core/src/languages/vbnet.rs"] {
        <<module>>
        -tree_sitter_vb_dotnet() Type95
        -node_to_symbol_type(node_type: Type5) Option~SymbolType~
        -is_container(node_type: Type5) bool
        -get_visibility(node: Type68, source: Type66) Visibility
        -get_name(node: Type68, source: Type66) Option~String~
        -extract_callee(node: Type68, source: Type66) Type71
        -find_enclosing_method(node: Type68, source: Type66) Option~String~
    }
    class c0092["CallEndpoint"] {
        <<struct>>
        -owner: Type96
        -member: Type97
        -name() Type98
        -location() Type99
    }
    class c0093["ExportError"] {
        <<enum>>
        -MissingDeclarations: unknown
        -InvalidOptions: unknown
        -NoMatchingPath: unknown
        -NoMatchingTestPath: Type16
        -InvalidTestMode: unknown
        +fmt(f: Type100) fmt::Result
    }
    class c0094["MermaidExport"] {
        <<struct>>
        +markdown: String
        +notices: Vec~String~
    }
    class c0095["MermaidOptions"] {
        <<struct>>
        +path: String
        +max_classes: usize
        +tests: TestMode
        +explain_tests: bool
        +test_paths: Vec~String~
        +keep_paths: Vec~String~
        +default() Self
    }
    class c0096["TestMode"] {
        <<enum>>
        -Exclude: unknown
        -Include: unknown
        +from_str(value: Type5) Type101
    }
    class c0097["crates/mycelium-core/src/mermaid.rs"] {
        <<module>>
        +export_mermaid(result: Type103, options: Type104) Type102
        +export_mermaid_report(result: Type103, options: Type104) Type105
        -type_edges(Signature16)
        -unique_occurrences(classes: Type106) Vec~Class~
        -merge_classes(raw: Type106, identities: Type106, warnings: Type76) Vec~Class~
        -name_index(classes: Type106) Type109
        -resolve(classes: Type106, index: Type110, file: Type5, name: Type5) Option~usize~
        -member_text(member: Type111, aliases: Type112) String
        -type_text(value: Type5, aliases: Type112) String
        -safe(value: Type5) String
        -pages(classes: Type106, maximum: usize) Type113
        -abbreviation(value: Type5, prefix: Type5, limit: usize, key: Type112) String
        -wrap_prose(markdown: Type5) String
        -visibility(value: Type5) Type5
        -type_names(value: Type5, language: Type5) Vec~String~
        -language_family(file: Type5) Type5
        -ordered_key(key: Type114) Type23
        -markdown_code(value: Type5) String
        -plain_member(member: Type111) String
    }
    class c0098["TestFilter"] {
        <<struct>>
        -test_paths: Vec~String~
        -keep_paths: Vec~String~
        -files: BTreeSet~String~
        -include: bool
        -detector_version: Option~u32~
        -explain: bool
        +notices: BTreeSet~String~
        -counts: Type115
        -explanations: BTreeSet~String~
        +new(result: Type103, options: Type104) Type116
        -hidden(file: Type5, line: usize, name: Type5, evidence: Type117, in_scope: bool) bool
        -valid_location(file: Type5, line: usize) bool
        +excludes_file(file: Type5) bool
        +retain(raw: Type106, in_scope: Type118) Vec~Class~
        +summary(filtered_calls: usize, filtered_edges: usize) String
    }
    class c0099["crates/mycelium-core/src/mermaid/filtering.rs"] {
        <<module>>
        +normalize_path(path: Type5) Type102
        +matches_path(file: Type5, prefix: Type5) bool
    }
    class c0100["PythonTypes"] {
        <<struct>>
        -bindings: Type119
        -modules: Type120
        -module_suffixes: Type121
        -classes: Type122
        +new(bindings: Type119, declarations: Type123) Self
        +resolve(file: Type5, name: Type5) Option~Option~usize~~
        -module(name: Type5, origin: Type5) Type124
        +reference_name(file: Type5, name: Type125) Type125
        -module_files(name: Type5, origin: Type5) Type126
        -imported(target: Type5, origin: Type5, seen: Type128) Type127
        -member(file: Type5, name: Type5, seen: Type128) Type127
        -class(file: Type5, name: Type5, line: usize) Option~usize~
    }
    class c0101["Target"] {
        <<enum>>
        -Class: Type18
        -Module: Type129
    }
    class c0102["crates/mycelium-core/src/output.rs"] {
        <<module>>
        -get_commit_hash(repo_path: Type5) Option~String~
        -count_languages(kg: Type56) Type92
        +build_result(Signature17)
        +write_output(result: Type103, output_path: Type5) Type132
    }
    class c0103["crates/mycelium-core/src/phases/calls.rs"] {
        <<module>>
        +run_calls_phase(config: Type4, kg: Type133, st: Type134, _ns_index: Type135) Type3
        -is_call_target(source_id: Type5, target_id: Type5, kg: Type56) bool
        -call_target_in_file(Signature18)
        -build_import_map(kg: Type56) Type54
        -build_field_type_map(file_path: Type5, kg: Type56) Type20
        -is_interface_self_call(Signature19)
        -is_interface_method(target_id: Type5, kg: Type56) bool
        -find_implementation(Signature20)
        -resolve_call(Signature21)
    }
    class c0104["AdjList"] {
        <<struct>>
        -node_map: Type92
        -nodes: Vec~String~
        -adj: Type139
        -new() Self
        -ensure_node(id: Type5) usize
        -add_edge(a: Type5, b: Type5, weight: f64) Type3
        -total_weight() f64
    }
    class c0105["crates/mycelium-core/src/phases/communities.rs"] {
        <<module>>
        +run_communities_phase(config: Type4, kg: Type133) Type3
        -louvain(adj: Type140, resolution: f64) Vec~Vec~String~~
        -split_oversized(community: Type55, adj: Type140, max_size: usize) Vec~Vec~String~~
        -generate_label(members: Type55, kg: Type56) String
        -disambiguate_label(Signature22)
        -compute_cohesion(members: Type55, adj: Type140) f64
        -primary_language(members: Type55, kg: Type56) String
        -common_prefix(strings: Type55) String
    }
    class c0106["crates/mycelium-core/src/phases/imports.rs"] {
        <<module>>
        +run_imports_phase(config: Type4, kg: Type133, st: Type134, ns_index: Type135) Type3
        -process_dotnet_projects(config: Type4, kg: Type133, assembly_index: Type141) Type3
        -register_observed_namespaces(kg: Type56, _assembly_index: Type142) Type3
        -process_source_imports(Signature23)
        -resolve_python_import(Signature24)
        -resolve_python_relative(Signature25)
        -resolve_ts_import(Signature26)
        -resolve_java_import(Signature27)
        -parse_go_mod(file_set: Type67, repo_root: Type5) Option~String~
        -build_go_dir_index(file_set: Type67) Type54
        -resolve_go_import(Signature28)
        -resolve_rust_import(Signature29)
        -resolve_c_include(Signature30)
        -resolve_fallback(Signature31)
        -normalize_path(path: Type5) String
    }
    class c0107["crates/mycelium-core/src/phases/parsing.rs"] {
        <<module>>
        +run_parsing_phase(config: Type4, kg: Type133, st: Type134, ns_index: Type135) Type3
    }
    class c0108["crates/mycelium-core/src/phases/processes.rs"] {
        <<module>>
        +run_processes_phase(config: Type4, kg: Type133) Type3
        -bfs_traces(Signature32)
        -deduplicate(traces: Vec~Vec~String~~) Vec~Vec~String~~
        -build_community_map(kg: Type56) Type20
        -classify_process(trace: Type55, community_map: Type22) String
        -compute_total_confidence(kg: Type56, trace: Type55) f64
        -sort_key(trace: Type55, total_conf: f64) Type143
    }
    class c0109["crates/mycelium-core/src/phases/structure.rs"] {
        <<module>>
        +run_structure_phase(config: Type4, kg: Type133) Type3
    }
    class c0110["crates/mycelium-core/src/pipeline.rs"] {
        <<module>>
        +run_pipeline(config: Type4, progress_callback: Option~ProgressCallback~) Type144
    }
    class c0127["PyAnalysisConfig"] {
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
        -new(Signature33)
    }
    class c0128["crates/mycelium-python/src/lib.rs"] {
        <<module>>
        -analyze(Signature34)
        -export_mermaid(Signature35)
        -version() Type13
        -_mycelium_rust(m: Type148) Type147
    }
    class c0129["mycelium/cli.py"] {
        <<module>>
        +cli() None
        -_run_with_progress(config: PyAnalysisConfig) unknown
        -_run_quiet(config: PyAnalysisConfig) unknown
        +export_cmd(Signature36)
        +analyze_cmd(Signature37)
    }
    class c0259["vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs"] {
        <<module>>
        -main() Type3
    }
    class c0260["vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs"] {
        <<module>>
        -tree_sitter_tree_sitter_vb_dotnet() Type95
    }
    class c0261["vendor/tree-sitter-vb-dotnet/grammar.js"] {
        <<module>>
        commaSep(rule: unknown) unknown
        kw(word: unknown) unknown
        commaSep1(rule: unknown) unknown
        ci(keyword: unknown) unknown
    }
    class c0262["BdistWheel"] {
        <<class>>
        +get_tag() unknown
    }
    class c0263["Build"] {
        <<class>>
        +run() unknown
    }
    class c0264["EggInfo"] {
        <<class>>
        +find_sources() unknown
    }
    class c0265["vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h"] {
        <<module>>
        _array__erase(Signature38)
        _array__reserve(Signature39)
        _array__assign(Signature40)
        _array__swap(Signature41)
        _array__grow(Signature42)
        _array__splice(Signature43)
    }
    class c0266["TSCharacterRange"] {
        <<struct>>
        start: int32_t
        end: int32_t
    }
    class c0267["TSFieldMapEntry"] {
        <<struct>>
        field_id: TSFieldId
        child_index: uint8_t
        inherited: bool
    }
    class c0268["TSLanguage"] {
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
        parse_table: Type154
        small_parse_table: Type154
        small_parse_table_map: Type155
        parse_actions: Type156
        symbol_names: Type157
        field_names: Type157
        field_map_slices: Type158
        field_map_entries: Type159
        symbol_metadata: Type160
        public_symbol_map: Type161
        alias_map: Type154
        alias_sequences: Type161
        lex_modes: Type162
        keyword_capture_token: TSSymbol
        external_scanner: Type163
        states: Type164
        symbol_map: Type161
        primary_state_ids: Type165
        name: Type166
        reserved_words: Type161
        max_reserved_word_set_size: uint16_t
        supertype_count: uint32_t
        supertype_symbols: Type161
        supertype_map_slices: Type158
        supertype_map_entries: Type161
        metadata: TSLanguageMetadata
    }
    class c0269["TSLanguageMetadata"] {
        <<struct>>
        major_version: uint8_t
        minor_version: uint8_t
        patch_version: uint8_t
    }
    class c0270["TSLexMode"] {
        <<struct>>
        lex_state: uint16_t
        external_lex_state: uint16_t
    }
    class c0271["TSLexer"] {
        <<struct>>
        lookahead: int32_t
        result_symbol: TSSymbol
    }
    class c0272["TSLexerMode"] {
        <<struct>>
        lex_state: uint16_t
        external_lex_state: uint16_t
        reserved_word_set_id: uint16_t
    }
    class c0273["TSMapSlice"] {
        <<struct>>
        index: uint16_t
        length: uint16_t
    }
    class c0274["TSParseAction"] {
        <<union>>
        shift: Type167
        type: uint8_t
        state: TSStateId
        extra: bool
        repetition: bool
        reduce: Type168
        child_count: uint8_t
        symbol: TSSymbol
        dynamic_precedence: int16_t
        production_id: uint16_t
    }
    class c0275["TSParseActionEntry"] {
        <<union>>
        action: TSParseAction
        entry: Type169
        count: uint8_t
        reusable: bool
    }
    class c0276["TSParseActionType"] {
        <<enum>>
        TSParseActionTypeShift: TSParseActionType
        TSParseActionTypeReduce: TSParseActionType
        TSParseActionTypeAccept: TSParseActionType
        TSParseActionTypeRecover: TSParseActionType
    }
    class c0277["TSSymbolMetadata"] {
        <<struct>>
        visible: bool
        named: bool
        supertype: bool
    }
    class c0278["vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h"] {
        <<module>>
        set_contains(ranges: Type170, len: uint32_t, lookahead: int32_t) bool
    }
    c0000 --> c0001 : field command
    c0002 ..> c0002 : 3 relationships (see list)
    c0002 ..> c0004 : 2 relationships (see list)
    c0002 ..> c0042 : run_export() calls new()
    c0002 ..> c0097 : run_export() calls export_mermaid_report()
    c0002 ..> c0102 : 2 relationships (see list)
    c0002 ..> c0110 : 2 relationships (see list)
    c0004 ..> c0127 : type in from
    c0005 --> c0007 : field calls
    c0005 --> c0009 : field communities
    c0005 --> c0017 : field imports
    c0005 --> c0021 : field processes
    c0005 --> c0025 : field structure
    c0005 --> c0027 : field symbols
    c0005 ..> c0030 : 8 relationships (see list)
    c0005 ..> c0038 : default() calls default()
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
    c0032 --> c0040 : field test
    c0033 --> c0032 : field classes
    c0033 --> c0037 : field python_bindings
    c0033 --> c0038 : field test_detection
    c0034 --> c0035 : field parameters
    c0034 --> c0040 : field test
    c0037 --> c0036 : field names
    c0038 --> c0039 : field diagnostics
    c0042 ..> c0028 : resolve_namespace() calls as_str()
    c0042 ..> c0042 : default() calls new()
    c0045 ..> c0028 : extract_attr() calls as_str()
    c0045 ..> c0042 : parse_project_file() calls new()
    c0045 ..> c0044 : type in parse_project_file
    c0045 ..> c0045 : 6 relationships (see list)
    c0047 ..> c0046 : type in parse_solution
    c0050 ..> c0006 : type in add_call
    c0050 ..> c0008 : type in add_community
    c0050 ..> c0010 : type in add_file
    c0050 ..> c0012 : type in add_folder
    c0050 ..> c0014 : type in add_import
    c0050 ..> c0019 : type in add_package_reference
    c0050 ..> c0020 : type in add_process
    c0050 ..> c0023 : type in add_project_reference
    c0050 ..> c0026 : type in add_symbol
    c0050 ..> c0028 : add_symbol() calls as_str()
    c0050 ..> c0042 : new() calls new()
    c0050 ..> c0048 : 2 relationships (see list)
    c0050 --> c0049 : field graph
    c0050 ..> c0049 : type in inner_graph
    c0050 ..> c0050 : 17 relationships (see list)
    c0050 --> c0051 : field graph
    c0050 ..> c0051 : 5 relationships (see list)
    c0050 ..> c0052 : 2 relationships (see list)
    c0050 ..> c0104 : 7 relationships (see list)
    c0054 ..> c0054 : default() calls new()
    c0056 ..> c0028 : score_entry_points() calls as_str()
    c0056 ..> c0050 : 6 relationships (see list)
    c0056 ..> c0056 : score_entry_points() calls probe_depth()
    c0058 ..> c0026 : type in add
    c0058 ..> c0028 : 2 relationships (see list)
    c0058 --> c0057 : field global_index
    c0058 ..> c0057 : 2 relationships (see list)
    c0058 ..> c0058 : default() calls new()
    c0060 ..> c0016 : type in extract_imports
    c0060 ..> c0024 : type in extract_calls
    c0060 ..> c0026 : type in extract_symbols
    c0060 ..|> c0084 : implements
    c0061 ..> c0016 : type in extract_imports
    c0061 ..> c0024 : type in extract_calls
    c0061 ..> c0026 : 2 relationships (see list)
    c0061 ..> c0061 : 2 relationships (see list)
    c0061 ..> c0062 : 5 relationships (see list)
    c0061 ..|> c0084 : implements
    c0062 ..> c0016 : type in extract_includes
    c0062 ..> c0024 : type in find_c_calls
    c0062 ..> c0026 : type in extract_c_symbols
    c0062 ..> c0062 : 8 relationships (see list)
    c0063 ..> c0016 : 2 relationships (see list)
    c0063 ..> c0024 : 2 relationships (see list)
    c0063 ..> c0026 : 2 relationships (see list)
    c0063 ..> c0063 : 4 relationships (see list)
    c0063 ..> c0064 : 7 relationships (see list)
    c0063 ..> c0080 : find_calls() calls find_calls()
    c0063 ..> c0081 : walk_node() calls walk_node()
    c0063 ..|> c0084 : implements
    c0064 ..> c0028 : 2 relationships (see list)
    c0064 ..> c0029 : type in get_visibility
    c0065 ..> c0033 : type in walk
    c0065 ..> c0065 : 3 relationships (see list)
    c0065 ..> c0073 : walk() calls walk()
    c0065 ..> c0074 : 10 relationships (see list)
    c0066 ..> c0028 : detect() calls as_str()
    c0066 ..> c0066 : 12 relationships (see list)
    c0066 ..> c0070 : 2 relationships (see list)
    c0066 ..> c0071 : 2 relationships (see list)
    c0066 ..> c0074 : 6 relationships (see list)
    c0067 ..> c0004 : detect() calls from()
    c0067 ..> c0028 : detect() calls as_str()
    c0067 ..> c0066 : 3 relationships (see list)
    c0067 ..> c0070 : 2 relationships (see list)
    c0067 ..> c0071 : detect() calls visit()
    c0067 ..> c0074 : 3 relationships (see list)
    c0068 ..> c0068 : 3 relationships (see list)
    c0068 ..> c0070 : 2 relationships (see list)
    c0068 ..> c0071 : 2 relationships (see list)
    c0068 ..> c0072 : bound_names() calls bound_names()
    c0068 ..> c0074 : 5 relationships (see list)
    c0068 ..> c0098 : detect() calls retain()
    c0069 --> c0040 : field guards
    c0070 ..> c0004 : new() calls default()
    c0070 --> c0040 : field marks
    c0070 ..> c0040 : type in evidence
    c0070 ..> c0066 : 2 relationships (see list)
    c0070 --> c0069 : field bindings
    c0070 ..> c0078 : new() calls malformed()
    c0071 ..> c0065 : keyword() calls walk()
    c0071 ..> c0069 : 2 relationships (see list)
    c0071 ..> c0070 : finish() calls new()
    c0071 ..> c0074 : 2 relationships (see list)
    c0072 ..> c0042 : detect() calls new()
    c0072 ..> c0065 : detect() calls walk()
    c0072 ..> c0068 : bound_names() calls bound_names()
    c0072 ..> c0070 : 2 relationships (see list)
    c0072 ..> c0071 : detect() calls visit()
    c0072 ..> c0072 : detect() calls bound_names()
    c0072 ..> c0074 : 5 relationships (see list)
    c0072 ..> c0097 : detect() calls resolve()
    c0072 ..> c0098 : detect() calls retain()
    c0073 ..> c0033 : type in walk
    c0073 ..> c0065 : walk() calls walk()
    c0073 ..> c0074 : 5 relationships (see list)
    c0074 ..> c0004 : extract() calls default()
    c0074 ..> c0033 : 5 relationships (see list)
    c0074 ..> c0042 : extract() calls new()
    c0074 ..> c0065 : 3 relationships (see list)
    c0074 ..> c0069 : extract() calls extend()
    c0074 ..> c0074 : 8 relationships (see list)
    c0074 ..> c0076 : extract() calls bindings()
    c0074 ..> c0078 : rust_walk() calls rust_evidence()
    c0075 ..> c0033 : 2 relationships (see list)
    c0075 ..> c0065 : walk() calls walk()
    c0075 ..> c0070 : 2 relationships (see list)
    c0075 ..> c0074 : 10 relationships (see list)
    c0075 ..> c0075 : 6 relationships (see list)
    c0076 ..> c0032 : bindings() constructs Class
    c0076 ..> c0033 : 3 relationships (see list)
    c0076 ..> c0036 : type in insert_binding
    c0076 ..> c0037 : 6 relationships (see list)
    c0076 ..> c0065 : walk() calls walk()
    c0076 ..> c0069 : 2 relationships (see list)
    c0076 ..> c0070 : 2 relationships (see list)
    c0076 ..> c0074 : 18 relationships (see list)
    c0076 ..> c0076 : 12 relationships (see list)
    c0077 ..> c0078 : 2 relationships (see list)
    c0078 ..> c0040 : 2 relationships (see list)
    c0078 ..> c0065 : 5 relationships (see list)
    c0078 ..> c0074 : 2 relationships (see list)
    c0078 ..> c0077 : type in rust_evidence
    c0078 ..> c0078 : 3 relationships (see list)
    c0079 ..> c0033 : type in walk
    c0079 ..> c0065 : walk() calls walk()
    c0079 ..> c0070 : walk() calls evidence()
    c0079 ..> c0074 : 9 relationships (see list)
    c0079 ..> c0079 : walk() calls vb_type()
    c0080 ..> c0016 : type in extract_imports
    c0080 ..> c0024 : 2 relationships (see list)
    c0080 ..> c0026 : type in extract_symbols
    c0080 ..> c0063 : find_calls() calls find_calls()
    c0080 ..> c0080 : 10 relationships (see list)
    c0080 ..|> c0084 : implements
    c0081 ..> c0016 : type in extract_imports
    c0081 ..> c0024 : 2 relationships (see list)
    c0081 ..> c0026 : 2 relationships (see list)
    c0081 ..> c0063 : 2 relationships (see list)
    c0081 ..> c0081 : 5 relationships (see list)
    c0081 ..> c0082 : 5 relationships (see list)
    c0081 ..|> c0084 : implements
    c0082 ..> c0028 : 2 relationships (see list)
    c0082 ..> c0029 : type in get_visibility
    c0083 ..> c0028 : extensions() calls as_str()
    c0083 ..> c0060 : language_for_extension() calls language_name()
    c0083 ..> c0083 : 3 relationships (see list)
    c0083 --> c0084 : field analysers
    c0083 ..> c0084 : type in get_by_extension
    c0083 ..> c0090 : new() calls is_available()
    c0084 ..> c0016 : type in extract_imports
    c0084 ..> c0024 : type in extract_calls
    c0084 ..> c0026 : type in extract_symbols
    c0085 ..> c0016 : type in extract_imports
    c0085 ..> c0024 : 2 relationships (see list)
    c0085 ..> c0026 : 2 relationships (see list)
    c0085 ..> c0063 : 2 relationships (see list)
    c0085 ..|> c0084 : implements
    c0085 ..> c0085 : 6 relationships (see list)
    c0086 ..> c0016 : type in extract_imports
    c0086 ..> c0024 : 2 relationships (see list)
    c0086 ..> c0026 : 2 relationships (see list)
    c0086 ..> c0063 : 2 relationships (see list)
    c0086 ..|> c0084 : implements
    c0086 ..> c0086 : 8 relationships (see list)
    c0086 ..> c0087 : walk_node() calls node_to_symbol_type()
    c0087 ..> c0028 : type in node_to_symbol_type
    c0088 ..> c0016 : type in extract_imports
    c0088 ..> c0024 : 2 relationships (see list)
    c0088 ..> c0026 : 3 relationships (see list)
    c0088 ..> c0063 : find_calls() calls find_calls()
    c0088 ..|> c0084 : implements
    c0088 ..> c0088 : 13 relationships (see list)
    c0088 ..> c0089 : walk_node() calls node_to_symbol_type()
    c0089 ..> c0028 : type in node_to_symbol_type
    c0090 ..> c0016 : type in extract_imports
    c0090 ..> c0024 : 2 relationships (see list)
    c0090 ..> c0026 : 2 relationships (see list)
    c0090 ..> c0063 : 2 relationships (see list)
    c0090 ..|> c0084 : implements
    c0090 ..> c0090 : 3 relationships (see list)
    c0090 ..> c0091 : 6 relationships (see list)
    c0091 ..> c0028 : type in node_to_symbol_type
    c0091 ..> c0029 : type in get_visibility
    c0091 ..> c0091 : find_enclosing_method() calls get_name()
    c0092 --> c0032 : field owner
    c0092 --> c0034 : field member
    c0095 --> c0096 : field tests
    c0097 ..> c0005 : 2 relationships (see list)
    c0097 ..> c0028 : 4 relationships (see list)
    c0097 ..> c0032 : 6 relationships (see list)
    c0097 ..> c0034 : 3 relationships (see list)
    c0097 ..> c0042 : 4 relationships (see list)
    c0097 ..> c0069 : 2 relationships (see list)
    c0097 ..> c0083 : language_family() calls language_for_extension()
    c0097 ..> c0093 : 2 relationships (see list)
    c0097 ..> c0094 : type in export_mermaid_report
    c0097 ..> c0095 : 2 relationships (see list)
    c0097 ..> c0097 : 23 relationships (see list)
    c0097 ..> c0098 : 3 relationships (see list)
    c0097 ..> c0099 : 2 relationships (see list)
    c0098 ..> c0005 : type in new
    c0098 ..> c0028 : hidden() calls as_str()
    c0098 ..> c0032 : type in retain
    c0098 ..> c0040 : type in hidden
    c0098 ..> c0041 : hidden() calls supported_test_rule()
    c0098 ..> c0069 : new() calls extend()
    c0098 ..> c0093 : type in new
    c0098 ..> c0095 : type in new
    c0098 ..> c0098 : 3 relationships (see list)
    c0098 ..> c0099 : 5 relationships (see list)
    c0099 ..> c0093 : type in normalize_path
    c0100 ..> c0032 : type in new
    c0100 --> c0037 : field bindings
    c0100 ..> c0037 : type in new
    c0100 ..> c0101 : 2 relationships (see list)
    c0102 ..> c0004 : type in build_result
    c0102 ..> c0005 : 2 relationships (see list)
    c0102 ..> c0042 : 3 relationships (see list)
    c0102 ..> c0050 : 12 relationships (see list)
    c0102 ..> c0058 : type in build_result
    c0102 ..> c0102 : 2 relationships (see list)
    c0103 ..> c0004 : type in run_calls_phase
    c0103 ..> c0006 : type in resolve_call
    c0103 ..> c0028 : 3 relationships (see list)
    c0103 ..> c0042 : run_calls_phase() calls new()
    c0103 ..> c0050 : 17 relationships (see list)
    c0103 ..> c0054 : type in run_calls_phase
    c0103 ..> c0058 : 9 relationships (see list)
    c0103 ..> c0060 : run_calls_phase() calls extract_calls()
    c0103 ..> c0083 : run_calls_phase() calls get_by_extension()
    c0103 ..> c0088 : run_calls_phase() calls get_language_for_ext()
    c0103 ..> c0090 : run_calls_phase() calls is_available()
    c0103 ..> c0103 : 12 relationships (see list)
    c0104 ..> c0104 : add_edge() calls ensure_node()
    c0105 ..> c0004 : type in run_communities_phase
    c0105 ..> c0028 : 5 relationships (see list)
    c0105 ..> c0050 : 9 relationships (see list)
    c0105 ..> c0069 : 2 relationships (see list)
    c0105 ..> c0104 : 10 relationships (see list)
    c0105 ..> c0105 : 8 relationships (see list)
    c0106 ..> c0004 : 3 relationships (see list)
    c0106 ..> c0028 : process_source_imports() calls as_str()
    c0106 ..> c0042 : 16 relationships (see list)
    c0106 ..> c0045 : process_dotnet_projects() calls parse_project_file()
    c0106 ..> c0047 : process_dotnet_projects() calls parse_solution()
    c0106 ..> c0050 : 13 relationships (see list)
    c0106 ..> c0054 : 4 relationships (see list)
    c0106 ..> c0058 : 4 relationships (see list)
    c0106 ..> c0060 : process_source_imports() calls extract_imports()
    c0106 ..> c0083 : process_source_imports() calls get_by_extension()
    c0106 ..> c0088 : process_source_imports() calls get_language_for_ext()
    c0106 ..> c0090 : process_source_imports() calls is_available()
    c0106 ..> c0106 : 16 relationships (see list)
    c0107 ..> c0004 : 2 relationships (see list)
    c0107 ..> c0028 : run_parsing_phase() calls as_str()
    c0107 ..> c0042 : 2 relationships (see list)
    c0107 ..> c0050 : 3 relationships (see list)
    c0107 ..> c0054 : type in run_parsing_phase
    c0107 ..> c0058 : 2 relationships (see list)
    c0107 ..> c0060 : 2 relationships (see list)
    c0107 ..> c0069 : run_parsing_phase() calls extend()
    c0107 ..> c0071 : run_parsing_phase() calls finish()
    c0107 ..> c0074 : run_parsing_phase() calls extract()
    c0107 ..> c0083 : run_parsing_phase() calls get_by_extension()
    c0107 ..> c0088 : run_parsing_phase() calls get_language_for_ext()
    c0108 ..> c0004 : type in run_processes_phase
    c0108 ..> c0028 : 2 relationships (see list)
    c0108 ..> c0042 : bfs_traces() calls new()
    c0108 ..> c0050 : 8 relationships (see list)
    c0108 ..> c0056 : run_processes_phase() calls score_entry_points()
    c0108 ..> c0069 : run_processes_phase() calls extend()
    c0108 ..> c0108 : 6 relationships (see list)
    c0109 ..> c0004 : type in run_structure_phase
    c0109 ..> c0028 : run_structure_phase() calls as_str()
    c0109 ..> c0042 : run_structure_phase() calls new()
    c0109 ..> c0050 : 3 relationships (see list)
    c0109 ..> c0083 : run_structure_phase() calls language_for_extension()
    c0110 ..> c0004 : type in run_pipeline
    c0110 ..> c0005 : type in run_pipeline
    c0110 ..> c0042 : run_pipeline() calls new()
    c0110 ..> c0102 : run_pipeline() calls build_result()
    c0128 ..> c0074 : export_mermaid() calls extract()
    c0128 ..> c0096 : export_mermaid() calls from_str()
    c0128 ..> c0097 : export_mermaid() calls export_mermaid_report()
    c0128 ..> c0110 : analyze() calls run_pipeline()
    c0128 ..> c0127 : type in analyze
    c0129 ..> c0129 : 2 relationships (see list)
    c0259 ..> c0042 : main() calls new()
    c0261 ..> c0261 : 2 relationships (see list)
    c0268 --> c0267 : field field_map_entries
    c0268 --> c0269 : field metadata
    c0268 --> c0271 : field external_scanner
    c0268 --> c0272 : field lex_modes
    c0268 --> c0273 : 2 relationships (see list)
    c0268 --> c0275 : field parse_actions
    c0268 --> c0277 : field symbol_metadata
    c0275 --> c0274 : field action
    c0278 ..> c0266 : type in set_contains
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
- c0031: `Base` — `crates/mycelium-core/src/declarations.rs`:56
- c0032: `Class` — `crates/mycelium-core/src/declarations.rs`:41
- c0033: `ClassDiagram` — `crates/mycelium-core/src/declarations.rs`:9
- c0034: `Member` — `crates/mycelium-core/src/declarations.rs`:63
- c0035: `Parameter` — `crates/mycelium-core/src/declarations.rs`:80
- c0036: `PythonBinding` — `crates/mycelium-core/src/declarations.rs`:27
- c0037: `PythonBindings` — `crates/mycelium-core/src/declarations.rs`:20
- c0038: `TestDetection` — `crates/mycelium-core/src/declarations.rs`:87
- c0039: `TestDiagnostic` — `crates/mycelium-core/src/declarations.rs`:111
- c0040: `TestEvidence` — `crates/mycelium-core/src/declarations.rs`:102
- c0041: `crates/mycelium-core/src/declarations.rs` — `crates/mycelium-core/src/declarations.rs`:1
- c0042: `AssemblyIndex` — `crates/mycelium-core/src/dotnet/assembly.rs`:9
- c0044: `ProjectFile` — `crates/mycelium-core/src/dotnet/project.rs`:7
- c0045: `crates/mycelium-core/src/dotnet/project.rs` —
  `crates/mycelium-core/src/dotnet/project.rs`:1
- c0046: `SlnProject` — `crates/mycelium-core/src/dotnet/solution.rs`:8
- c0047: `crates/mycelium-core/src/dotnet/solution.rs` —
  `crates/mycelium-core/src/dotnet/solution.rs`:1
- c0048: `CallInfo` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:138
- c0049: `EdgeData` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:74
- c0050: `KnowledgeGraph` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:114
- c0051: `NodeData` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:14
- c0052: `SymbolInfo` — `crates/mycelium-core/src/graph/knowledge_graph.rs`:123
- c0054: `NamespaceIndex` — `crates/mycelium-core/src/graph/namespace_index.rs`:6
- c0056: `crates/mycelium-core/src/graph/scoring.rs` — `crates/mycelium-core/src/graph/scoring.rs`:1
- c0057: `SymbolDefinition` — `crates/mycelium-core/src/graph/symbol_table.rs`:9
- c0058: `SymbolTable` — `crates/mycelium-core/src/graph/symbol_table.rs`:22
- c0060: `CAnalyser` — `crates/mycelium-core/src/languages/c_cpp.rs`:366
- c0061: `CppAnalyser` — `crates/mycelium-core/src/languages/c_cpp.rs`:425
- c0062: `crates/mycelium-core/src/languages/c_cpp.rs` —
  `crates/mycelium-core/src/languages/c_cpp.rs`:1
- c0063: `CSharpAnalyser` — `crates/mycelium-core/src/languages/csharp.rs`:195
- c0064: `crates/mycelium-core/src/languages/csharp.rs` —
  `crates/mycelium-core/src/languages/csharp.rs`:1
- c0065: `crates/mycelium-core/src/languages/declarations/c_cpp.rs` —
  `crates/mycelium-core/src/languages/declarations/c_cpp.rs`:1
- c0066: `crates/mycelium-core/src/languages/declarations/frameworks/attributes.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/attributes.rs`:1
- c0067: `crates/mycelium-core/src/languages/declarations/frameworks/java.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/java.rs`:1
- c0068: `crates/mycelium-core/src/languages/declarations/frameworks/javascript.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/javascript.rs`:1
- c0069: `Bindings` — `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs`:77
- c0070: `Detection` — `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs`:13
- c0071: `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/mod.rs`:1
- c0072: `crates/mycelium-core/src/languages/declarations/frameworks/python.rs` —
  `crates/mycelium-core/src/languages/declarations/frameworks/python.rs`:1
- c0073: `crates/mycelium-core/src/languages/declarations/go.rs` —
  `crates/mycelium-core/src/languages/declarations/go.rs`:1
- c0074: `crates/mycelium-core/src/languages/declarations/mod.rs` —
  `crates/mycelium-core/src/languages/declarations/mod.rs`:1
- c0075: `crates/mycelium-core/src/languages/declarations/nominal.rs` —
  `crates/mycelium-core/src/languages/declarations/nominal.rs`:1
- c0076: `crates/mycelium-core/src/languages/declarations/python.rs` —
  `crates/mycelium-core/src/languages/declarations/python.rs`:1
- c0077: `RustDetection` — `crates/mycelium-core/src/languages/declarations/test_detection.rs`:6
- c0078: `crates/mycelium-core/src/languages/declarations/test_detection.rs` —
  `crates/mycelium-core/src/languages/declarations/test_detection.rs`:1
- c0079: `crates/mycelium-core/src/languages/declarations/vbnet.rs` —
  `crates/mycelium-core/src/languages/declarations/vbnet.rs`:1
- c0080: `GoAnalyser` — `crates/mycelium-core/src/languages/go_lang.rs`:66
- c0081: `JavaAnalyser` — `crates/mycelium-core/src/languages/java.rs`:119
- c0082: `crates/mycelium-core/src/languages/java.rs` —
  `crates/mycelium-core/src/languages/java.rs`:1
- c0083: `AnalyserRegistry` — `crates/mycelium-core/src/languages/mod.rs`:55
- c0084: `LanguageAnalyser` — `crates/mycelium-core/src/languages/mod.rs`:20
- c0085: `PythonAnalyser` — `crates/mycelium-core/src/languages/python.rs`:93
- c0086: `RustAnalyser` — `crates/mycelium-core/src/languages/rust_lang.rs`:111
- c0087: `crates/mycelium-core/src/languages/rust_lang.rs` —
  `crates/mycelium-core/src/languages/rust_lang.rs`:1
- c0088: `TypeScriptAnalyser` — `crates/mycelium-core/src/languages/typescript.rs`:74
- c0089: `crates/mycelium-core/src/languages/typescript.rs` —
  `crates/mycelium-core/src/languages/typescript.rs`:1
- c0090: `VbNetAnalyser` — `crates/mycelium-core/src/languages/vbnet.rs`:142
- c0091: `crates/mycelium-core/src/languages/vbnet.rs` —
  `crates/mycelium-core/src/languages/vbnet.rs`:1
- c0092: `CallEndpoint` — `crates/mycelium-core/src/mermaid.rs`:424
- c0093: `ExportError` — `crates/mycelium-core/src/mermaid.rs`:64
- c0094: `MermaidExport` — `crates/mycelium-core/src/mermaid.rs`:32
- c0095: `MermaidOptions` — `crates/mycelium-core/src/mermaid.rs`:39
- c0096: `TestMode` — `crates/mycelium-core/src/mermaid.rs`:14
- c0097: `crates/mycelium-core/src/mermaid.rs` — `crates/mycelium-core/src/mermaid.rs`:1
- c0098: `TestFilter` — `crates/mycelium-core/src/mermaid/filtering.rs`:9
- c0099: `crates/mycelium-core/src/mermaid/filtering.rs` —
  `crates/mycelium-core/src/mermaid/filtering.rs`:1
- c0100: `PythonTypes` — `crates/mycelium-core/src/mermaid/python.rs`:7
- c0101: `Target` — `crates/mycelium-core/src/mermaid/python.rs`:14
- c0102: `crates/mycelium-core/src/output.rs` — `crates/mycelium-core/src/output.rs`:1
- c0103: `crates/mycelium-core/src/phases/calls.rs` — `crates/mycelium-core/src/phases/calls.rs`:1
- c0104: `AdjList` — `crates/mycelium-core/src/phases/communities.rs`:99
- c0105: `crates/mycelium-core/src/phases/communities.rs` —
  `crates/mycelium-core/src/phases/communities.rs`:1
- c0106: `crates/mycelium-core/src/phases/imports.rs` —
  `crates/mycelium-core/src/phases/imports.rs`:1
- c0107: `crates/mycelium-core/src/phases/parsing.rs` —
  `crates/mycelium-core/src/phases/parsing.rs`:1
- c0108: `crates/mycelium-core/src/phases/processes.rs` —
  `crates/mycelium-core/src/phases/processes.rs`:1
- c0109: `crates/mycelium-core/src/phases/structure.rs` —
  `crates/mycelium-core/src/phases/structure.rs`:1
- c0110: `crates/mycelium-core/src/pipeline.rs` — `crates/mycelium-core/src/pipeline.rs`:1
- c0127: `PyAnalysisConfig` — `crates/mycelium-python/src/lib.rs`:12
- c0128: `crates/mycelium-python/src/lib.rs` — `crates/mycelium-python/src/lib.rs`:1
- c0129: `mycelium/cli.py` — `mycelium/cli.py`:1
- c0259: `vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs` —
  `vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs`:1
- c0260: `vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs` —
  `vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs`:1
- c0261: `vendor/tree-sitter-vb-dotnet/grammar.js` — `vendor/tree-sitter-vb-dotnet/grammar.js`:1
- c0262: `BdistWheel` — `vendor/tree-sitter-vb-dotnet/setup.py`:38
- c0263: `Build` — `vendor/tree-sitter-vb-dotnet/setup.py`:30
- c0264: `EggInfo` — `vendor/tree-sitter-vb-dotnet/setup.py`:46
- c0265: `vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h` —
  `vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h`:1
- c0266: `TSCharacterRange` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:102
- c0267: `TSFieldMapEntry` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:28
- c0268: `TSLanguage` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:107
- c0269: `TSLanguageMetadata` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:21
- c0270: `TSLexMode` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:83
- c0271: `TSLexer` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:48
- c0272: `TSLexerMode` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:88
- c0273: `TSMapSlice` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:35
- c0274: `TSParseAction` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:66
- c0275: `TSParseActionEntry` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:94
- c0276: `TSParseActionType` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:59
- c0277: `TSSymbolMetadata` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:40
- c0278: `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h` —
  `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:1

## Relationships

- c0000 --> c0001: `field command`
- c0002 ..> c0002: `main() calls run_export()`
- c0002 ..> c0002: `main() calls run_quiet()`
- c0002 ..> c0002: `main() calls run_with_progress()`
- c0002 ..> c0004: `type in run_quiet`
- c0002 ..> c0004: `type in run_with_progress`
- c0002 ..> c0042: `run_export() calls new()`
- c0002 ..> c0097: `run_export() calls export_mermaid_report()`
- c0002 ..> c0102: `run_quiet() calls write_output()`
- c0002 ..> c0102: `run_with_progress() calls write_output()`
- c0002 ..> c0110: `run_quiet() calls run_pipeline()`
- c0002 ..> c0110: `run_with_progress() calls run_pipeline()`
- c0004 ..> c0127: `type in from`
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
- c0005 ..> c0038: `default() calls default()`
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
- c0032 --> c0040: `field test`
- c0033 --> c0032: `field classes`
- c0033 --> c0037: `field python_bindings`
- c0033 --> c0038: `field test_detection`
- c0034 --> c0035: `field parameters`
- c0034 --> c0040: `field test`
- c0037 --> c0036: `field names`
- c0038 --> c0039: `field diagnostics`
- c0042 ..> c0028: `resolve_namespace() calls as_str()`
- c0042 ..> c0042: `default() calls new()`
- c0045 ..> c0028: `extract_attr() calls as_str()`
- c0045 ..> c0042: `parse_project_file() calls new()`
- c0045 ..> c0044: `type in parse_project_file`
- c0045 ..> c0045: `extract_include_attrs() calls extract_attr()`
- c0045 ..> c0045: `extract_package_refs() calls extract_attr()`
- c0045 ..> c0045: `extract_package_refs() calls extract_element_text()`
- c0045 ..> c0045: `parse_project_file() calls extract_element_text()`
- c0045 ..> c0045: `parse_project_file() calls extract_include_attrs()`
- c0045 ..> c0045: `parse_project_file() calls extract_package_refs()`
- c0047 ..> c0046: `type in parse_solution`
- c0050 ..> c0006: `type in add_call`
- c0050 ..> c0008: `type in add_community`
- c0050 ..> c0010: `type in add_file`
- c0050 ..> c0012: `type in add_folder`
- c0050 ..> c0014: `type in add_import`
- c0050 ..> c0019: `type in add_package_reference`
- c0050 ..> c0020: `type in add_process`
- c0050 ..> c0023: `type in add_project_reference`
- c0050 ..> c0026: `type in add_symbol`
- c0050 ..> c0028: `add_symbol() calls as_str()`
- c0050 ..> c0042: `new() calls new()`
- c0050 ..> c0048: `type in get_callees`
- c0050 ..> c0048: `type in get_callers`
- c0050 --> c0049: `field graph`
- c0050 ..> c0049: `type in inner_graph`
- c0050 ..> c0050: `add_community() calls ensure_node()`
- c0050 ..> c0050: `add_file() calls ensure_node()`
- c0050 ..> c0050: `add_folder() calls ensure_node()`
- c0050 ..> c0050: `add_import() calls ensure_node()`
- c0050 ..> c0050: `add_package_reference() calls ensure_node()`
- c0050 ..> c0050: `add_process() calls ensure_node()`
- c0050 ..> c0050: `add_project_reference() calls ensure_node()`
- c0050 ..> c0050: `add_symbol() calls ensure_node()`
- c0050 ..> c0050: `default() calls new()`
- c0050 ..> c0050: `get_call_edges() calls node_id()`
- c0050 ..> c0050: `get_callees() calls node_id()`
- c0050 ..> c0050: `get_callers() calls node_id()`
- c0050 ..> c0050: `get_communities() calls node_id()`
- c0050 ..> c0050: `get_import_edges() calls node_id()`
- c0050 ..> c0050: `get_package_references() calls node_id()`
- c0050 ..> c0050: `get_processes() calls node_id()`
- c0050 ..> c0050: `get_project_references() calls node_id()`
- c0050 --> c0051: `field graph`
- c0050 ..> c0051: `type in ensure_node`
- c0050 ..> c0051: `type in get_files`
- c0050 ..> c0051: `type in get_folders`
- c0050 ..> c0051: `type in get_node_data`
- c0050 ..> c0051: `type in inner_graph`
- c0050 ..> c0052: `type in get_symbols`
- c0050 ..> c0052: `type in get_symbols_in_file`
- c0050 ..> c0104: `add_call() calls add_edge()`
- c0050 ..> c0104: `add_community() calls add_edge()`
- c0050 ..> c0104: `add_import() calls add_edge()`
- c0050 ..> c0104: `add_package_reference() calls add_edge()`
- c0050 ..> c0104: `add_process() calls add_edge()`
- c0050 ..> c0104: `add_project_reference() calls add_edge()`
- c0050 ..> c0104: `add_symbol() calls add_edge()`
- c0054 ..> c0054: `default() calls new()`
- c0056 ..> c0028: `score_entry_points() calls as_str()`
- c0056 ..> c0050: `probe_depth() calls get_callees()`
- c0056 ..> c0050: `score_entry_points() calls get_callees()`
- c0056 ..> c0050: `score_entry_points() calls get_callers()`
- c0056 ..> c0050: `score_entry_points() calls get_symbols()`
- c0056 ..> c0050: `type in probe_depth`
- c0056 ..> c0050: `type in score_entry_points`
- c0056 ..> c0056: `score_entry_points() calls probe_depth()`
- c0058 ..> c0026: `type in add`
- c0058 ..> c0028: `add() calls as_str()`
- c0058 ..> c0028: `lookup_exact() calls as_str()`
- c0058 --> c0057: `field global_index`
- c0058 ..> c0057: `type in global_index`
- c0058 ..> c0057: `type in lookup_fuzzy`
- c0058 ..> c0058: `default() calls new()`
- c0060 ..> c0016: `type in extract_imports`
- c0060 ..> c0024: `type in extract_calls`
- c0060 ..> c0026: `type in extract_symbols`
- c0060 ..|> c0084: `implements`
- c0061 ..> c0016: `type in extract_imports`
- c0061 ..> c0024: `type in extract_calls`
- c0061 ..> c0026: `type in extract_cpp_symbols`
- c0061 ..> c0026: `type in extract_symbols`
- c0061 ..> c0061: `extract_calls() calls builtin_exclusions()`
- c0061 ..> c0061: `extract_symbols() calls extract_cpp_symbols()`
- c0061 ..> c0062: `extract_calls() calls find_c_calls()`
- c0061 ..> c0062: `extract_cpp_symbols() calls extract_c_symbols()`
- c0061 ..> c0062: `extract_cpp_symbols() calls get_type_name()`
- c0061 ..> c0062: `extract_imports() calls extract_includes()`
- c0061 ..> c0062: `extract_symbols() calls extract_c_symbols()`
- c0061 ..|> c0084: `implements`
- c0062 ..> c0016: `type in extract_includes`
- c0062 ..> c0024: `type in find_c_calls`
- c0062 ..> c0026: `type in extract_c_symbols`
- c0062 ..> c0062: `extract_c_symbols() calls get_func_name()`
- c0062 ..> c0062: `extract_c_symbols() calls get_qualified_func_name()`
- c0062 ..> c0062: `extract_c_symbols() calls get_type_name()`
- c0062 ..> c0062: `extract_c_symbols() calls is_preproc_container()`
- c0062 ..> c0062: `find_c_calls() calls extract_c_callee()`
- c0062 ..> c0062: `find_c_calls() calls find_enclosing_func()`
- c0062 ..> c0062: `find_enclosing_func() calls get_qualified_func_name()`
- c0062 ..> c0062: `get_func_name() calls get_qualified_func_name()`
- c0063 ..> c0016: `type in extract_imports`
- c0063 ..> c0016: `type in extract_using`
- c0063 ..> c0024: `type in extract_calls`
- c0063 ..> c0024: `type in find_calls`
- c0063 ..> c0026: `type in extract_symbols`
- c0063 ..> c0026: `type in walk_node`
- c0063 ..> c0063: `extract_calls() calls builtin_exclusions()`
- c0063 ..> c0063: `extract_calls() calls find_calls()`
- c0063 ..> c0063: `extract_imports() calls extract_using()`
- c0063 ..> c0063: `extract_symbols() calls walk_node()`
- c0063 ..> c0064: `find_calls() calls extract_callee()`
- c0063 ..> c0064: `find_calls() calls find_enclosing_method()`
- c0063 ..> c0064: `walk_node() calls extract_parameter_types()`
- c0063 ..> c0064: `walk_node() calls get_name()`
- c0063 ..> c0064: `walk_node() calls get_visibility()`
- c0063 ..> c0064: `walk_node() calls is_container()`
- c0063 ..> c0064: `walk_node() calls node_to_symbol_type()`
- c0063 ..> c0080: `find_calls() calls find_calls()`
- c0063 ..> c0081: `walk_node() calls walk_node()`
- c0063 ..|> c0084: `implements`
- c0064 ..> c0028: `get_visibility() calls as_str()`
- c0064 ..> c0028: `type in node_to_symbol_type`
- c0064 ..> c0029: `type in get_visibility`
- c0065 ..> c0033: `type in walk`
- c0065 ..> c0065: `walk() calls declaration_type()`
- c0065 ..> c0065: `walk() calls declarator_name()`
- c0065 ..> c0065: `walk() calls find_function()`
- c0065 ..> c0073: `walk() calls walk()`
- c0065 ..> c0074: `declaration_type() calls children()`
- c0065 ..> c0074: `declaration_type() calls field()`
- c0065 ..> c0074: `declaration_type() calls text()`
- c0065 ..> c0074: `declarator_name() calls text()`
- c0065 ..> c0074: `walk() calls add_class()`
- c0065 ..> c0074: `walk() calls children()`
- c0065 ..> c0074: `walk() calls enum_member()`
- c0065 ..> c0074: `walk() calls field()`
- c0065 ..> c0074: `walk() calls module()`
- c0065 ..> c0074: `walk() calls text()`
- c0066 ..> c0028: `detect() calls as_str()`
- c0066 ..> c0066: `detect() calls ancestor()`
- c0066 ..> c0066: `detect() calls clean()`
- c0066 ..> c0066: `detect() calls guard_name()`
- c0066 ..> c0066: `detect() calls inherited_scope()`
- c0066 ..> c0066: `detect() calls key()`
- c0066 ..> c0066: `detect() calls rule()`
- c0066 ..> c0066: `detect() calls scope()`
- c0066 ..> c0066: `detect() calls type_node()`
- c0066 ..> c0066: `guard_name() calls key()`
- c0066 ..> c0066: `inherited_scope() calls type_node()`
- c0066 ..> c0066: `scope() calls scope_inner()`
- c0066 ..> c0066: `scope_inner() calls type_node()`
- c0066 ..> c0070: `detect() calls mark()`
- c0066 ..> c0070: `type in detect`
- c0066 ..> c0071: `detect() calls keyword()`
- c0066 ..> c0071: `detect() calls visit()`
- c0066 ..> c0074: `detect() calls children()`
- c0066 ..> c0074: `detect() calls field()`
- c0066 ..> c0074: `detect() calls text()`
- c0066 ..> c0074: `inherited_scope() calls children()`
- c0066 ..> c0074: `scope_inner() calls children()`
- c0066 ..> c0074: `scope_inner() calls field()`
- c0067 ..> c0004: `detect() calls from()`
- c0067 ..> c0028: `detect() calls as_str()`
- c0067 ..> c0066: `detect() calls inherited_scope()`
- c0067 ..> c0066: `detect() calls scope()`
- c0067 ..> c0066: `detect() calls type_node()`
- c0067 ..> c0070: `detect() calls mark()`
- c0067 ..> c0070: `type in detect`
- c0067 ..> c0071: `detect() calls visit()`
- c0067 ..> c0074: `detect() calls children()`
- c0067 ..> c0074: `detect() calls field()`
- c0067 ..> c0074: `detect() calls text()`
- c0068 ..> c0068: `detect() calls bound_names()`
- c0068 ..> c0068: `detect() calls inline()`
- c0068 ..> c0068: `detect() calls literal()`
- c0068 ..> c0070: `detect() calls mark()`
- c0068 ..> c0070: `type in detect`
- c0068 ..> c0071: `detect() calls keyword()`
- c0068 ..> c0071: `detect() calls visit()`
- c0068 ..> c0072: `bound_names() calls bound_names()`
- c0068 ..> c0074: `bound_names() calls children()`
- c0068 ..> c0074: `bound_names() calls text()`
- c0068 ..> c0074: `detect() calls children()`
- c0068 ..> c0074: `detect() calls field()`
- c0068 ..> c0074: `detect() calls text()`
- c0068 ..> c0098: `detect() calls retain()`
- c0069 --> c0040: `field guards`
- c0070 ..> c0004: `new() calls default()`
- c0070 --> c0040: `field marks`
- c0070 ..> c0040: `type in evidence`
- c0070 ..> c0066: `evidence() calls type_node()`
- c0070 ..> c0066: `new() calls detect()`
- c0070 --> c0069: `field bindings`
- c0070 ..> c0078: `new() calls malformed()`
- c0071 ..> c0065: `keyword() calls walk()`
- c0071 ..> c0069: `finish() calls extend()`
- c0071 ..> c0069: `type in finish`
- c0071 ..> c0070: `finish() calls new()`
- c0071 ..> c0074: `keyword() calls children()`
- c0071 ..> c0074: `visit() calls children()`
- c0072 ..> c0042: `detect() calls new()`
- c0072 ..> c0065: `detect() calls walk()`
- c0072 ..> c0068: `bound_names() calls bound_names()`
- c0072 ..> c0070: `detect() calls mark()`
- c0072 ..> c0070: `type in detect`
- c0072 ..> c0071: `detect() calls visit()`
- c0072 ..> c0072: `detect() calls bound_names()`
- c0072 ..> c0074: `bound_names() calls children()`
- c0072 ..> c0074: `bound_names() calls text()`
- c0072 ..> c0074: `detect() calls children()`
- c0072 ..> c0074: `detect() calls field()`
- c0072 ..> c0074: `detect() calls text()`
- c0072 ..> c0097: `detect() calls resolve()`
- c0072 ..> c0098: `detect() calls retain()`
- c0073 ..> c0033: `type in walk`
- c0073 ..> c0065: `walk() calls walk()`
- c0073 ..> c0074: `walk() calls add_class()`
- c0073 ..> c0074: `walk() calls children()`
- c0073 ..> c0074: `walk() calls field()`
- c0073 ..> c0074: `walk() calls module()`
- c0073 ..> c0074: `walk() calls text()`
- c0074 ..> c0004: `extract() calls default()`
- c0074 ..> c0033: `type in add_class`
- c0074 ..> c0033: `type in enum_member`
- c0074 ..> c0033: `type in extract`
- c0074 ..> c0033: `type in module`
- c0074 ..> c0033: `type in rust_walk`
- c0074 ..> c0042: `extract() calls new()`
- c0074 ..> c0065: `children() calls walk()`
- c0074 ..> c0065: `extract() calls walk()`
- c0074 ..> c0065: `rust_walk() calls walk()`
- c0074 ..> c0069: `extract() calls extend()`
- c0074 ..> c0074: `extract() calls rust_walk()`
- c0074 ..> c0074: `field() calls text()`
- c0074 ..> c0074: `module() calls add_class()`
- c0074 ..> c0074: `rust_walk() calls add_class()`
- c0074 ..> c0074: `rust_walk() calls children()`
- c0074 ..> c0074: `rust_walk() calls field()`
- c0074 ..> c0074: `rust_walk() calls module()`
- c0074 ..> c0074: `rust_walk() calls text()`
- c0074 ..> c0076: `extract() calls bindings()`
- c0074 ..> c0078: `rust_walk() calls rust_evidence()`
- c0075 ..> c0033: `type in add_field`
- c0075 ..> c0033: `type in walk`
- c0075 ..> c0065: `walk() calls walk()`
- c0075 ..> c0070: `add_field() calls evidence()`
- c0075 ..> c0070: `walk() calls evidence()`
- c0075 ..> c0074: `base_types() calls children()`
- c0075 ..> c0074: `type_field() calls field()`
- c0075 ..> c0074: `visibility() calls children()`
- c0075 ..> c0074: `visibility() calls text()`
- c0075 ..> c0074: `walk() calls add_class()`
- c0075 ..> c0074: `walk() calls children()`
- c0075 ..> c0074: `walk() calls enum_member()`
- c0075 ..> c0074: `walk() calls field()`
- c0075 ..> c0074: `walk() calls module()`
- c0075 ..> c0074: `walk() calls text()`
- c0075 ..> c0075: `add_field() calls visibility()`
- c0075 ..> c0075: `base_types() calls walk()`
- c0075 ..> c0075: `walk() calls add_field()`
- c0075 ..> c0075: `walk() calls base_types()`
- c0075 ..> c0075: `walk() calls type_field()`
- c0075 ..> c0075: `walk() calls visibility()`
- c0076 ..> c0032: `bindings() constructs Class`
- c0076 ..> c0033: `type in instance_fields`
- c0076 ..> c0033: `type in push_field`
- c0076 ..> c0033: `type in walk`
- c0076 ..> c0036: `type in insert_binding`
- c0076 ..> c0037: `type in bindings`
- c0076 ..> c0037: `type in block_bound_names`
- c0076 ..> c0037: `type in import_bindings`
- c0076 ..> c0037: `type in insert_binding`
- c0076 ..> c0037: `type in typing_guard`
- c0076 ..> c0037: `type in uncertain_bindings`
- c0076 ..> c0065: `walk() calls walk()`
- c0076 ..> c0069: `import_target() calls extend()`
- c0076 ..> c0069: `insert_binding() calls extend()`
- c0076 ..> c0070: `push_field() calls evidence()`
- c0076 ..> c0070: `walk() calls evidence()`
- c0076 ..> c0074: `bindings() calls children()`
- c0076 ..> c0074: `bindings() calls field()`
- c0076 ..> c0074: `block_bound_names() calls children()`
- c0076 ..> c0074: `block_bound_names() calls text()`
- c0076 ..> c0074: `import_bindings() calls field()`
- c0076 ..> c0074: `import_bindings() calls text()`
- c0076 ..> c0074: `instance_fields() calls children()`
- c0076 ..> c0074: `instance_fields() calls field()`
- c0076 ..> c0074: `push_field() calls field()`
- c0076 ..> c0074: `typing_guard() calls field()`
- c0076 ..> c0074: `uncertain_bindings() calls children()`
- c0076 ..> c0074: `uncertain_bindings() calls field()`
- c0076 ..> c0074: `uncertain_bindings() calls text()`
- c0076 ..> c0074: `walk() calls add_class()`
- c0076 ..> c0074: `walk() calls children()`
- c0076 ..> c0074: `walk() calls field()`
- c0076 ..> c0074: `walk() calls module()`
- c0076 ..> c0074: `walk() calls text()`
- c0076 ..> c0076: `bindings() calls import_bindings()`
- c0076 ..> c0076: `bindings() calls insert_binding()`
- c0076 ..> c0076: `bindings() calls typing_guard()`
- c0076 ..> c0076: `bindings() calls uncertain_bindings()`
- c0076 ..> c0076: `import_bindings() calls import_target()`
- c0076 ..> c0076: `import_bindings() calls insert_binding()`
- c0076 ..> c0076: `import_bindings() calls walk()`
- c0076 ..> c0076: `instance_fields() calls push_field()`
- c0076 ..> c0076: `uncertain_bindings() calls block_bound_names()`
- c0076 ..> c0076: `uncertain_bindings() calls walk()`
- c0076 ..> c0076: `walk() calls instance_fields()`
- c0076 ..> c0076: `walk() calls push_field()`
- c0077 ..> c0078: `new() calls malformed()`
- c0077 ..> c0078: `new() calls uncertain_import()`
- c0078 ..> c0040: `type in evidence`
- c0078 ..> c0040: `type in rust_evidence`
- c0078 ..> c0065: `binds_test() calls walk()`
- c0078 ..> c0065: `malformed() calls walk()`
- c0078 ..> c0065: `rust_evidence() calls walk()`
- c0078 ..> c0065: `tokens() calls walk()`
- c0078 ..> c0065: `uncertain_import() calls walk()`
- c0078 ..> c0074: `malformed() calls children()`
- c0078 ..> c0074: `tokens() calls children()`
- c0078 ..> c0077: `type in rust_evidence`
- c0078 ..> c0078: `rust_evidence() calls evidence()`
- c0078 ..> c0078: `rust_evidence() calls tokens()`
- c0078 ..> c0078: `uncertain_import() calls binds_test()`
- c0079 ..> c0033: `type in walk`
- c0079 ..> c0065: `walk() calls walk()`
- c0079 ..> c0070: `walk() calls evidence()`
- c0079 ..> c0074: `vb_type() calls children()`
- c0079 ..> c0074: `vb_type() calls field()`
- c0079 ..> c0074: `vb_type() calls text()`
- c0079 ..> c0074: `walk() calls add_class()`
- c0079 ..> c0074: `walk() calls children()`
- c0079 ..> c0074: `walk() calls enum_member()`
- c0079 ..> c0074: `walk() calls field()`
- c0079 ..> c0074: `walk() calls module()`
- c0079 ..> c0074: `walk() calls text()`
- c0079 ..> c0079: `walk() calls vb_type()`
- c0080 ..> c0016: `type in extract_imports`
- c0080 ..> c0024: `type in extract_calls`
- c0080 ..> c0024: `type in find_calls`
- c0080 ..> c0026: `type in extract_symbols`
- c0080 ..> c0063: `find_calls() calls find_calls()`
- c0080 ..> c0080: `extract_calls() calls builtin_exclusions()`
- c0080 ..> c0080: `extract_calls() calls find_calls()`
- c0080 ..> c0080: `extract_imports() calls extract_string()`
- c0080 ..> c0080: `extract_imports() calls extract_string_content()`
- c0080 ..> c0080: `extract_string() calls extract_string_content()`
- c0080 ..> c0080: `extract_symbols() calls get_name_by_kind()`
- c0080 ..> c0080: `extract_symbols() calls is_exported()`
- c0080 ..> c0080: `find_calls() calls extract_callee()`
- c0080 ..> c0080: `find_calls() calls find_enclosing()`
- c0080 ..> c0080: `find_enclosing() calls get_name_by_kind()`
- c0080 ..|> c0084: `implements`
- c0081 ..> c0016: `type in extract_imports`
- c0081 ..> c0024: `type in extract_calls`
- c0081 ..> c0024: `type in find_calls`
- c0081 ..> c0026: `type in extract_symbols`
- c0081 ..> c0026: `type in walk_node`
- c0081 ..> c0063: `find_calls() calls find_calls()`
- c0081 ..> c0063: `walk_node() calls walk_node()`
- c0081 ..> c0081: `extract_calls() calls builtin_exclusions()`
- c0081 ..> c0081: `extract_calls() calls find_calls()`
- c0081 ..> c0081: `extract_symbols() calls walk_node()`
- c0081 ..> c0081: `find_calls() calls extract_callee()`
- c0081 ..> c0081: `find_calls() calls find_enclosing()`
- c0081 ..> c0082: `find_enclosing() calls get_name()`
- c0081 ..> c0082: `walk_node() calls get_name()`
- c0081 ..> c0082: `walk_node() calls get_visibility()`
- c0081 ..> c0082: `walk_node() calls is_container()`
- c0081 ..> c0082: `walk_node() calls node_to_symbol_type()`
- c0081 ..|> c0084: `implements`
- c0082 ..> c0028: `get_visibility() calls as_str()`
- c0082 ..> c0028: `type in node_to_symbol_type`
- c0082 ..> c0029: `type in get_visibility`
- c0083 ..> c0028: `extensions() calls as_str()`
- c0083 ..> c0060: `language_for_extension() calls language_name()`
- c0083 ..> c0083: `default() calls new()`
- c0083 ..> c0083: `language_for_extension() calls get_by_extension()`
- c0083 ..> c0083: `new() calls extensions()`
- c0083 --> c0084: `field analysers`
- c0083 ..> c0084: `type in get_by_extension`
- c0083 ..> c0090: `new() calls is_available()`
- c0084 ..> c0016: `type in extract_imports`
- c0084 ..> c0024: `type in extract_calls`
- c0084 ..> c0026: `type in extract_symbols`
- c0085 ..> c0016: `type in extract_imports`
- c0085 ..> c0024: `type in extract_calls`
- c0085 ..> c0024: `type in find_calls`
- c0085 ..> c0026: `type in extract_symbols`
- c0085 ..> c0026: `type in walk_node`
- c0085 ..> c0063: `find_calls() calls find_calls()`
- c0085 ..> c0063: `walk_node() calls walk_node()`
- c0085 ..|> c0084: `implements`
- c0085 ..> c0085: `extract_calls() calls builtin_exclusions()`
- c0085 ..> c0085: `extract_calls() calls find_calls()`
- c0085 ..> c0085: `extract_symbols() calls walk_node()`
- c0085 ..> c0085: `find_calls() calls extract_callee()`
- c0085 ..> c0085: `find_calls() calls find_enclosing()`
- c0085 ..> c0085: `walk_node() calls get_name()`
- c0086 ..> c0016: `type in extract_imports`
- c0086 ..> c0024: `type in extract_calls`
- c0086 ..> c0024: `type in find_calls`
- c0086 ..> c0026: `type in extract_symbols`
- c0086 ..> c0026: `type in walk_node`
- c0086 ..> c0063: `find_calls() calls find_calls()`
- c0086 ..> c0063: `walk_node() calls walk_node()`
- c0086 ..|> c0084: `implements`
- c0086 ..> c0086: `extract_calls() calls builtin_exclusions()`
- c0086 ..> c0086: `extract_calls() calls find_calls()`
- c0086 ..> c0086: `extract_symbols() calls walk_node()`
- c0086 ..> c0086: `find_calls() calls extract_callee()`
- c0086 ..> c0086: `find_calls() calls find_enclosing()`
- c0086 ..> c0086: `find_enclosing() calls get_name()`
- c0086 ..> c0086: `walk_node() calls get_name()`
- c0086 ..> c0086: `walk_node() calls is_pub()`
- c0086 ..> c0087: `walk_node() calls node_to_symbol_type()`
- c0087 ..> c0028: `type in node_to_symbol_type`
- c0088 ..> c0016: `type in extract_imports`
- c0088 ..> c0024: `type in extract_calls`
- c0088 ..> c0024: `type in find_calls`
- c0088 ..> c0026: `type in extract_class_members`
- c0088 ..> c0026: `type in extract_symbols`
- c0088 ..> c0026: `type in walk_node`
- c0088 ..> c0063: `find_calls() calls find_calls()`
- c0088 ..|> c0084: `implements`
- c0088 ..> c0088: `extract_calls() calls builtin_exclusions()`
- c0088 ..> c0088: `extract_calls() calls find_calls()`
- c0088 ..> c0088: `extract_imports() calls extract_string_source()`
- c0088 ..> c0088: `extract_symbols() calls walk_node()`
- c0088 ..> c0088: `find_calls() calls extract_callee()`
- c0088 ..> c0088: `find_calls() calls find_enclosing()`
- c0088 ..> c0088: `get_language() calls get_ts_language()`
- c0088 ..> c0088: `get_language_for_ext() calls get_js_language()`
- c0088 ..> c0088: `get_language_for_ext() calls get_ts_language()`
- c0088 ..> c0088: `get_language_for_ext() calls get_tsx_language()`
- c0088 ..> c0088: `walk_node() calls extract_class_members()`
- c0088 ..> c0088: `walk_node() calls get_name()`
- c0088 ..> c0088: `walk_node() calls language_for_path()`
- c0088 ..> c0089: `walk_node() calls node_to_symbol_type()`
- c0089 ..> c0028: `type in node_to_symbol_type`
- c0090 ..> c0016: `type in extract_imports`
- c0090 ..> c0024: `type in extract_calls`
- c0090 ..> c0024: `type in find_calls`
- c0090 ..> c0026: `type in extract_symbols`
- c0090 ..> c0026: `type in walk_node`
- c0090 ..> c0063: `find_calls() calls find_calls()`
- c0090 ..> c0063: `walk_node() calls walk_node()`
- c0090 ..|> c0084: `implements`
- c0090 ..> c0090: `extract_calls() calls builtin_exclusions()`
- c0090 ..> c0090: `extract_calls() calls find_calls()`
- c0090 ..> c0090: `extract_symbols() calls walk_node()`
- c0090 ..> c0091: `find_calls() calls extract_callee()`
- c0090 ..> c0091: `find_calls() calls find_enclosing_method()`
- c0090 ..> c0091: `walk_node() calls get_name()`
- c0090 ..> c0091: `walk_node() calls get_visibility()`
- c0090 ..> c0091: `walk_node() calls is_container()`
- c0090 ..> c0091: `walk_node() calls node_to_symbol_type()`
- c0091 ..> c0028: `type in node_to_symbol_type`
- c0091 ..> c0029: `type in get_visibility`
- c0091 ..> c0091: `find_enclosing_method() calls get_name()`
- c0092 --> c0032: `field owner`
- c0092 --> c0034: `field member`
- c0095 --> c0096: `field tests`
- c0097 ..> c0005: `type in export_mermaid`
- c0097 ..> c0005: `type in export_mermaid_report`
- c0097 ..> c0028: `export_mermaid_report() calls as_str()`
- c0097 ..> c0028: `type_edges() calls as_str()`
- c0097 ..> c0028: `type_names() calls as_str()`
- c0097 ..> c0028: `unique_occurrences() calls as_str()`
- c0097 ..> c0032: `type in merge_classes`
- c0097 ..> c0032: `type in name_index`
- c0097 ..> c0032: `type in pages`
- c0097 ..> c0032: `type in resolve`
- c0097 ..> c0032: `type in type_edges`
- c0097 ..> c0032: `type in unique_occurrences`
- c0097 ..> c0034: `type in member_text`
- c0097 ..> c0034: `type in pages`
- c0097 ..> c0034: `type in plain_member`
- c0097 ..> c0042: `export_mermaid_report() calls new()`
- c0097 ..> c0042: `language_family() calls new()`
- c0097 ..> c0042: `resolve() calls new()`
- c0097 ..> c0042: `type_names() calls new()`
- c0097 ..> c0069: `merge_classes() calls extend()`
- c0097 ..> c0069: `type_names() calls extend()`
- c0097 ..> c0083: `language_family() calls language_for_extension()`
- c0097 ..> c0093: `type in export_mermaid`
- c0097 ..> c0093: `type in export_mermaid_report`
- c0097 ..> c0094: `type in export_mermaid_report`
- c0097 ..> c0095: `type in export_mermaid`
- c0097 ..> c0095: `type in export_mermaid_report`
- c0097 ..> c0097: `export_mermaid() calls export_mermaid_report()`
- c0097 ..> c0097: `export_mermaid_report() calls abbreviation()`
- c0097 ..> c0097: `export_mermaid_report() calls language_family()`
- c0097 ..> c0097: `export_mermaid_report() calls member_text()`
- c0097 ..> c0097: `export_mermaid_report() calls merge_classes()`
- c0097 ..> c0097: `export_mermaid_report() calls name_index()`
- c0097 ..> c0097: `export_mermaid_report() calls ordered_key()`
- c0097 ..> c0097: `export_mermaid_report() calls pages()`
- c0097 ..> c0097: `export_mermaid_report() calls plain_member()`
- c0097 ..> c0097: `export_mermaid_report() calls resolve()`
- c0097 ..> c0097: `export_mermaid_report() calls safe()`
- c0097 ..> c0097: `export_mermaid_report() calls type_edges()`
- c0097 ..> c0097: `export_mermaid_report() calls unique_occurrences()`
- c0097 ..> c0097: `export_mermaid_report() calls wrap_prose()`
- c0097 ..> c0097: `member_text() calls type_text()`
- c0097 ..> c0097: `member_text() calls visibility()`
- c0097 ..> c0097: `merge_classes() calls name_index()`
- c0097 ..> c0097: `merge_classes() calls resolve()`
- c0097 ..> c0097: `resolve() calls language_family()`
- c0097 ..> c0097: `type_edges() calls language_family()`
- c0097 ..> c0097: `type_edges() calls name_index()`
- c0097 ..> c0097: `type_edges() calls resolve()`
- c0097 ..> c0097: `type_edges() calls type_names()`
- c0097 ..> c0098: `export_mermaid_report() calls excludes_file()`
- c0097 ..> c0098: `export_mermaid_report() calls retain()`
- c0097 ..> c0098: `export_mermaid_report() calls summary()`
- c0097 ..> c0099: `export_mermaid_report() calls matches_path()`
- c0097 ..> c0099: `export_mermaid_report() calls normalize_path()`
- c0098 ..> c0005: `type in new`
- c0098 ..> c0028: `hidden() calls as_str()`
- c0098 ..> c0032: `type in retain`
- c0098 ..> c0040: `type in hidden`
- c0098 ..> c0041: `hidden() calls supported_test_rule()`
- c0098 ..> c0069: `new() calls extend()`
- c0098 ..> c0093: `type in new`
- c0098 ..> c0095: `type in new`
- c0098 ..> c0098: `hidden() calls valid_location()`
- c0098 ..> c0098: `retain() calls hidden()`
- c0098 ..> c0098: `retain() calls valid_location()`
- c0098 ..> c0099: `excludes_file() calls matches_path()`
- c0098 ..> c0099: `hidden() calls matches_path()`
- c0098 ..> c0099: `new() calls matches_path()`
- c0098 ..> c0099: `new() calls normalize_path()`
- c0098 ..> c0099: `valid_location() calls normalize_path()`
- c0099 ..> c0093: `type in normalize_path`
- c0100 ..> c0032: `type in new`
- c0100 --> c0037: `field bindings`
- c0100 ..> c0037: `type in new`
- c0100 ..> c0101: `type in imported`
- c0100 ..> c0101: `type in member`
- c0102 ..> c0004: `type in build_result`
- c0102 ..> c0005: `type in build_result`
- c0102 ..> c0005: `type in write_output`
- c0102 ..> c0042: `build_result() calls new()`
- c0102 ..> c0042: `get_commit_hash() calls new()`
- c0102 ..> c0042: `write_output() calls new()`
- c0102 ..> c0050: `build_result() calls get_call_edges()`
- c0102 ..> c0050: `build_result() calls get_communities()`
- c0102 ..> c0050: `build_result() calls get_files()`
- c0102 ..> c0050: `build_result() calls get_folders()`
- c0102 ..> c0050: `build_result() calls get_import_edges()`
- c0102 ..> c0050: `build_result() calls get_package_references()`
- c0102 ..> c0050: `build_result() calls get_processes()`
- c0102 ..> c0050: `build_result() calls get_project_references()`
- c0102 ..> c0050: `build_result() calls get_symbols()`
- c0102 ..> c0050: `count_languages() calls get_files()`
- c0102 ..> c0050: `type in build_result`
- c0102 ..> c0050: `type in count_languages`
- c0102 ..> c0058: `type in build_result`
- c0102 ..> c0102: `build_result() calls count_languages()`
- c0102 ..> c0102: `build_result() calls get_commit_hash()`
- c0103 ..> c0004: `type in run_calls_phase`
- c0103 ..> c0006: `type in resolve_call`
- c0103 ..> c0028: `call_target_in_file() calls as_str()`
- c0103 ..> c0028: `is_call_target() calls as_str()`
- c0103 ..> c0028: `run_calls_phase() calls as_str()`
- c0103 ..> c0042: `run_calls_phase() calls new()`
- c0103 ..> c0050: `build_field_type_map() calls get_symbols_in_file()`
- c0103 ..> c0050: `build_import_map() calls get_import_edges()`
- c0103 ..> c0050: `find_implementation() calls get_symbols()`
- c0103 ..> c0050: `is_call_target() calls get_node_data()`
- c0103 ..> c0050: `is_interface_method() calls get_symbols()`
- c0103 ..> c0050: `is_interface_self_call() calls get_symbols()`
- c0103 ..> c0050: `run_calls_phase() calls add_call()`
- c0103 ..> c0050: `run_calls_phase() calls get_files()`
- c0103 ..> c0050: `type in build_field_type_map`
- c0103 ..> c0050: `type in build_import_map`
- c0103 ..> c0050: `type in call_target_in_file`
- c0103 ..> c0050: `type in find_implementation`
- c0103 ..> c0050: `type in is_call_target`
- c0103 ..> c0050: `type in is_interface_method`
- c0103 ..> c0050: `type in is_interface_self_call`
- c0103 ..> c0050: `type in resolve_call`
- c0103 ..> c0050: `type in run_calls_phase`
- c0103 ..> c0054: `type in run_calls_phase`
- c0103 ..> c0058: `call_target_in_file() calls lookup_exact()`
- c0103 ..> c0058: `call_target_in_file() calls lookup_fuzzy()`
- c0103 ..> c0058: `find_implementation() calls lookup_fuzzy()`
- c0103 ..> c0058: `resolve_call() calls lookup_exact()`
- c0103 ..> c0058: `resolve_call() calls lookup_fuzzy()`
- c0103 ..> c0058: `type in call_target_in_file`
- c0103 ..> c0058: `type in find_implementation`
- c0103 ..> c0058: `type in resolve_call`
- c0103 ..> c0058: `type in run_calls_phase`
- c0103 ..> c0060: `run_calls_phase() calls extract_calls()`
- c0103 ..> c0083: `run_calls_phase() calls get_by_extension()`
- c0103 ..> c0088: `run_calls_phase() calls get_language_for_ext()`
- c0103 ..> c0090: `run_calls_phase() calls is_available()`
- c0103 ..> c0103: `call_target_in_file() calls is_call_target()`
- c0103 ..> c0103: `find_implementation() calls call_target_in_file()`
- c0103 ..> c0103: `find_implementation() calls is_call_target()`
- c0103 ..> c0103: `find_implementation() calls is_interface_method()`
- c0103 ..> c0103: `resolve_call() calls call_target_in_file()`
- c0103 ..> c0103: `resolve_call() calls find_implementation()`
- c0103 ..> c0103: `resolve_call() calls is_call_target()`
- c0103 ..> c0103: `resolve_call() calls is_interface_method()`
- c0103 ..> c0103: `resolve_call() calls is_interface_self_call()`
- c0103 ..> c0103: `run_calls_phase() calls build_field_type_map()`
- c0103 ..> c0103: `run_calls_phase() calls build_import_map()`
- c0103 ..> c0103: `run_calls_phase() calls resolve_call()`
- c0104 ..> c0104: `add_edge() calls ensure_node()`
- c0105 ..> c0004: `type in run_communities_phase`
- c0105 ..> c0028: `compute_cohesion() calls as_str()`
- c0105 ..> c0028: `disambiguate_label() calls as_str()`
- c0105 ..> c0028: `generate_label() calls as_str()`
- c0105 ..> c0028: `primary_language() calls as_str()`
- c0105 ..> c0028: `split_oversized() calls as_str()`
- c0105 ..> c0050: `disambiguate_label() calls get_symbols()`
- c0105 ..> c0050: `generate_label() calls get_symbols()`
- c0105 ..> c0050: `primary_language() calls get_symbols()`
- c0105 ..> c0050: `run_communities_phase() calls add_community()`
- c0105 ..> c0050: `run_communities_phase() calls get_call_edges()`
- c0105 ..> c0050: `type in disambiguate_label`
- c0105 ..> c0050: `type in generate_label`
- c0105 ..> c0050: `type in primary_language`
- c0105 ..> c0050: `type in run_communities_phase`
- c0105 ..> c0069: `run_communities_phase() calls extend()`
- c0105 ..> c0069: `split_oversized() calls extend()`
- c0105 ..> c0104: `louvain() calls total_weight()`
- c0105 ..> c0104: `run_communities_phase() calls add_edge()`
- c0105 ..> c0104: `run_communities_phase() calls new()`
- c0105 ..> c0104: `split_oversized() calls add_edge()`
- c0105 ..> c0104: `split_oversized() calls ensure_node()`
- c0105 ..> c0104: `split_oversized() calls new()`
- c0105 ..> c0104: `split_oversized() calls total_weight()`
- c0105 ..> c0104: `type in compute_cohesion`
- c0105 ..> c0104: `type in louvain`
- c0105 ..> c0104: `type in split_oversized`
- c0105 ..> c0105: `generate_label() calls common_prefix()`
- c0105 ..> c0105: `run_communities_phase() calls compute_cohesion()`
- c0105 ..> c0105: `run_communities_phase() calls disambiguate_label()`
- c0105 ..> c0105: `run_communities_phase() calls generate_label()`
- c0105 ..> c0105: `run_communities_phase() calls louvain()`
- c0105 ..> c0105: `run_communities_phase() calls primary_language()`
- c0105 ..> c0105: `run_communities_phase() calls split_oversized()`
- c0105 ..> c0105: `split_oversized() calls louvain()`
- c0106 ..> c0004: `type in process_dotnet_projects`
- c0106 ..> c0004: `type in process_source_imports`
- c0106 ..> c0004: `type in run_imports_phase`
- c0106 ..> c0028: `process_source_imports() calls as_str()`
- c0106 ..> c0042: `build_go_dir_index() calls new()`
- c0106 ..> c0042: `parse_go_mod() calls new()`
- c0106 ..> c0042: `process_dotnet_projects() calls new()`
- c0106 ..> c0042: `process_dotnet_projects() calls register()`
- c0106 ..> c0042: `process_source_imports() calls new()`
- c0106 ..> c0042: `resolve_c_include() calls new()`
- c0106 ..> c0042: `resolve_fallback() calls new()`
- c0106 ..> c0042: `resolve_fallback() calls resolve_namespace()`
- c0106 ..> c0042: `resolve_python_relative() calls new()`
- c0106 ..> c0042: `resolve_rust_import() calls new()`
- c0106 ..> c0042: `resolve_ts_import() calls new()`
- c0106 ..> c0042: `run_imports_phase() calls new()`
- c0106 ..> c0042: `type in process_dotnet_projects`
- c0106 ..> c0042: `type in process_source_imports`
- c0106 ..> c0042: `type in register_observed_namespaces`
- c0106 ..> c0042: `type in resolve_fallback`
- c0106 ..> c0045: `process_dotnet_projects() calls parse_project_file()`
- c0106 ..> c0047: `process_dotnet_projects() calls parse_solution()`
- c0106 ..> c0050: `process_dotnet_projects() calls add_package_reference()`
- c0106 ..> c0050: `process_dotnet_projects() calls add_project_reference()`
- c0106 ..> c0050: `process_dotnet_projects() calls get_files()`
- c0106 ..> c0050: `process_source_imports() calls add_import()`
- c0106 ..> c0050: `process_source_imports() calls get_files()`
- c0106 ..> c0050: `register_observed_namespaces() calls get_symbols()`
- c0106 ..> c0050: `resolve_fallback() calls get_files()`
- c0106 ..> c0050: `resolve_fallback() calls get_symbols_in_file()`
- c0106 ..> c0050: `type in process_dotnet_projects`
- c0106 ..> c0050: `type in process_source_imports`
- c0106 ..> c0050: `type in register_observed_namespaces`
- c0106 ..> c0050: `type in resolve_fallback`
- c0106 ..> c0050: `type in run_imports_phase`
- c0106 ..> c0054: `process_source_imports() calls get_files_for_namespace()`
- c0106 ..> c0054: `process_source_imports() calls register_file_import()`
- c0106 ..> c0054: `type in process_source_imports`
- c0106 ..> c0054: `type in run_imports_phase`
- c0106 ..> c0058: `resolve_fallback() calls lookup_fuzzy()`
- c0106 ..> c0058: `type in process_source_imports`
- c0106 ..> c0058: `type in resolve_fallback`
- c0106 ..> c0058: `type in run_imports_phase`
- c0106 ..> c0060: `process_source_imports() calls extract_imports()`
- c0106 ..> c0083: `process_source_imports() calls get_by_extension()`
- c0106 ..> c0088: `process_source_imports() calls get_language_for_ext()`
- c0106 ..> c0090: `process_source_imports() calls is_available()`
- c0106 ..> c0106: `process_dotnet_projects() calls normalize_path()`
- c0106 ..> c0106: `process_source_imports() calls build_go_dir_index()`
- c0106 ..> c0106: `process_source_imports() calls parse_go_mod()`
- c0106 ..> c0106: `process_source_imports() calls resolve_c_include()`
- c0106 ..> c0106: `process_source_imports() calls resolve_fallback()`
- c0106 ..> c0106: `process_source_imports() calls resolve_go_import()`
- c0106 ..> c0106: `process_source_imports() calls resolve_java_import()`
- c0106 ..> c0106: `process_source_imports() calls resolve_python_import()`
- c0106 ..> c0106: `process_source_imports() calls resolve_rust_import()`
- c0106 ..> c0106: `process_source_imports() calls resolve_ts_import()`
- c0106 ..> c0106: `resolve_c_include() calls normalize_path()`
- c0106 ..> c0106: `resolve_python_import() calls resolve_python_relative()`
- c0106 ..> c0106: `resolve_ts_import() calls normalize_path()`
- c0106 ..> c0106: `run_imports_phase() calls process_dotnet_projects()`
- c0106 ..> c0106: `run_imports_phase() calls process_source_imports()`
- c0106 ..> c0106: `run_imports_phase() calls register_observed_namespaces()`
- c0107 ..> c0004: `run_parsing_phase() calls default()`
- c0107 ..> c0004: `type in run_parsing_phase`
- c0107 ..> c0028: `run_parsing_phase() calls as_str()`
- c0107 ..> c0042: `run_parsing_phase() calls new()`
- c0107 ..> c0042: `run_parsing_phase() calls register()`
- c0107 ..> c0050: `run_parsing_phase() calls add_symbol()`
- c0107 ..> c0050: `run_parsing_phase() calls get_files()`
- c0107 ..> c0050: `type in run_parsing_phase`
- c0107 ..> c0054: `type in run_parsing_phase`
- c0107 ..> c0058: `run_parsing_phase() calls add()`
- c0107 ..> c0058: `type in run_parsing_phase`
- c0107 ..> c0060: `run_parsing_phase() calls extract_symbols()`
- c0107 ..> c0060: `run_parsing_phase() calls language_name()`
- c0107 ..> c0069: `run_parsing_phase() calls extend()`
- c0107 ..> c0071: `run_parsing_phase() calls finish()`
- c0107 ..> c0074: `run_parsing_phase() calls extract()`
- c0107 ..> c0083: `run_parsing_phase() calls get_by_extension()`
- c0107 ..> c0088: `run_parsing_phase() calls get_language_for_ext()`
- c0108 ..> c0004: `type in run_processes_phase`
- c0108 ..> c0028: `classify_process() calls as_str()`
- c0108 ..> c0028: `deduplicate() calls as_str()`
- c0108 ..> c0042: `bfs_traces() calls new()`
- c0108 ..> c0050: `bfs_traces() calls get_callees()`
- c0108 ..> c0050: `build_community_map() calls get_communities()`
- c0108 ..> c0050: `compute_total_confidence() calls get_callees()`
- c0108 ..> c0050: `run_processes_phase() calls add_process()`
- c0108 ..> c0050: `type in bfs_traces`
- c0108 ..> c0050: `type in build_community_map`
- c0108 ..> c0050: `type in compute_total_confidence`
- c0108 ..> c0050: `type in run_processes_phase`
- c0108 ..> c0056: `run_processes_phase() calls score_entry_points()`
- c0108 ..> c0069: `run_processes_phase() calls extend()`
- c0108 ..> c0108: `run_processes_phase() calls bfs_traces()`
- c0108 ..> c0108: `run_processes_phase() calls build_community_map()`
- c0108 ..> c0108: `run_processes_phase() calls classify_process()`
- c0108 ..> c0108: `run_processes_phase() calls compute_total_confidence()`
- c0108 ..> c0108: `run_processes_phase() calls deduplicate()`
- c0108 ..> c0108: `run_processes_phase() calls sort_key()`
- c0109 ..> c0004: `type in run_structure_phase`
- c0109 ..> c0028: `run_structure_phase() calls as_str()`
- c0109 ..> c0042: `run_structure_phase() calls new()`
- c0109 ..> c0050: `run_structure_phase() calls add_file()`
- c0109 ..> c0050: `run_structure_phase() calls add_folder()`
- c0109 ..> c0050: `type in run_structure_phase`
- c0109 ..> c0083: `run_structure_phase() calls language_for_extension()`
- c0110 ..> c0004: `type in run_pipeline`
- c0110 ..> c0005: `type in run_pipeline`
- c0110 ..> c0042: `run_pipeline() calls new()`
- c0110 ..> c0102: `run_pipeline() calls build_result()`
- c0128 ..> c0074: `export_mermaid() calls extract()`
- c0128 ..> c0096: `export_mermaid() calls from_str()`
- c0128 ..> c0097: `export_mermaid() calls export_mermaid_report()`
- c0128 ..> c0110: `analyze() calls run_pipeline()`
- c0128 ..> c0127: `type in analyze`
- c0129 ..> c0129: `analyze_cmd() calls _run_quiet()`
- c0129 ..> c0129: `analyze_cmd() calls _run_with_progress()`
- c0259 ..> c0042: `main() calls new()`
- c0261 ..> c0261: `commaSep() calls commaSep1()`
- c0261 ..> c0261: `kw() calls ci()`
- c0268 --> c0267: `field field_map_entries`
- c0268 --> c0269: `field metadata`
- c0268 --> c0271: `field external_scanner`
- c0268 --> c0272: `field lex_modes`
- c0268 --> c0273: `field field_map_slices`
- c0268 --> c0273: `field supertype_map_slices`
- c0268 --> c0275: `field parse_actions`
- c0268 --> c0277: `field symbol_metadata`
- c0275 --> c0274: `field action`
- c0278 ..> c0266: `type in set_contains`

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
- Type15: `BTreeMap<String, PythonBindings>`
- Type16: `(String)`
- Type17: `(BTreeSet<String>)`
- Type18: `(usize)`
- Type19: `BTreeMap<String, PythonBinding>`
- Type20: `HashMap<String, String>`
- Type21: `Option<&str>`
- Type22: `&HashMap<String, String>`
- Type23: `Vec<(String, String)>`
- Type24: `{ statement: String, }`
- Type25: `{ confidence: f64, tier: String, reason: String, line: usize, }`
- Type26: `{ ref_type: String, }`
- Type27: `{ version: String, }`
- Type28: `{ order: usize, }`
- Type29: `DiGraph<NodeData, EdgeData>`
- Type30: `HashMap<String, NodeIndex>`
- Type31: `Option<&NodeData>`
- Type32: `&FileNode`
- Type33: `&FolderNode`
- Type34: `&Symbol`
- Type35: `&CallEdge`
- Type36: `&ImportEdge`
- Type37: `&ProjectReference`
- Type38: `&PackageReference`
- Type39: `&Community`
- Type40: `&Process`
- Type41: `Vec<&NodeData>`
- Type42: `Vec<(String, String, f64, String, String, usize)>`
- Type43: `Vec<(String, String, String)>`
- Type44: `Vec<(String, String, Vec<String>, f64, String)>`
- Type45: `Vec<(String, String, String, Vec<String>, String, f64)>`
- Type46: `&DiGraph<NodeData, EdgeData>`
- Type47: `&HashMap<String, NodeIndex>`
- Type48: `{ path: String, language: Option<String>, size: u64, lines: usize, }`
- Type49: `{ path: String, file_count: usize, }`
- Type50: `{ id: String, name: String, symbol_type: String, file: String, line: usize, visibility:
  String, exported: bool, parent: Option<String>, language: Option<String>, parameter_types:
  Option<Vec<(String, String)>>, }`
- Type51: `{ id: String, label: String, cohesion: f64, primary_language: String, }`
- Type52: `{ id: String, entry: String, terminal: String, process_type: String, total_confidence:
  f64, }`
- Type53: `{ name: String, }`
- Type54: `HashMap<String, Vec<String>>`
- Type55: `&[String]`
- Type56: `&KnowledgeGraph`
- Type57: `Vec<(String, f64)>`
- Type58: `HashMap<String, HashMap<String, String>>`
- Type59: `HashMap<String, Vec<SymbolDefinition>>`
- Type60: `&[SymbolDefinition]`
- Type61: `Option<&HashMap<String, String>>`
- Type62: `&HashMap<String, HashMap<String, String>>`
- Type63: `&HashMap<String, Vec<SymbolDefinition>>`
- Type64: `&[&str]`
- Type65: `&Tree`
- Type66: `&[u8]`
- Type67: `&HashSet<String>`
- Type68: `&Node`
- Type69: `&mut Vec<Symbol>`
- Type70: `&mut Vec<RawCall>`
- Type71: `(Option<String>, Option<String>)`
- Type72: `Node<'_>`
- Type73: `&mut ClassDiagram`
- Type74: `Option<Node<'_>>`
- Type75: `&mut Detection`
- Type76: `&mut BTreeSet<String>`
- Type77: `Option<&'static str>`
- Type78: `Vec<(TestEvidence, BTreeSet<String>)>`
- Type79: `BTreeMap<usize, TestEvidence>`
- Type80: `Node<'tree>`
- Type81: `&mut impl FnMut(Node<'tree>)`
- Type82: `&mut crate::declarations::ClassDiagram`
- Type83: `&mut std::collections::BTreeSet<String>`
- Type84: `&mut frameworks::Bindings`
- Type85: `Vec<Node<'_>>`
- Type86: `&mut test_detection::RustDetection`
- Type87: `&frameworks::Detection`
- Type88: `&mut PythonBindings`
- Type89: `&PythonBindings`
- Type90: `&BTreeSet<usize>`
- Type91: `&mut RustDetection`
- Type92: `HashMap<String, usize>`
- Type93: `Option<&dyn LanguageAnalyser>`
- Type94: `Vec<&str>`
- Type95: `*const ()`
- Type96: `&'a Class`
- Type97: `Option<&'a Member>`
- Type98: `&'a str`
- Type99: `(&'a str, &'a str, usize, &'a str)`
- Type100: `&mut fmt::Formatter<'_>`
- Type101: `Result<Self, Self::Err>`
- Type102: `Result<String, ExportError>`
- Type103: `&AnalysisResult`
- Type104: `&MermaidOptions`
- Type105: `Result<MermaidExport, ExportError>`
- Type106: `&[Class]`
- Type107: `&BTreeSet<&str>`
- Type108: `&python::PythonTypes<'_>`
- Type109: `BTreeMap<String, Vec<usize>>`
- Type110: `&BTreeMap<String, Vec<usize>>`
- Type111: `&Member`
- Type112: `&mut BTreeMap<String, String>`
- Type113: `Vec<Vec<(usize, &[Member])>>`
- Type114: `BTreeMap<String, String>`
- Type115: `BTreeMap<(String, String), usize>`
- Type116: `Result<Self, ExportError>`
- Type117: `Option<&TestEvidence>`
- Type118: `&BTreeSet<String>`
- Type119: `&'a BTreeMap<String, PythonBindings>`
- Type120: `BTreeMap<String, Vec<&'a str>>`
- Type121: `BTreeMap<String, Vec<(&'a str, &'a str)>>`
- Type122: `BTreeMap<(&'a str, &'a str, usize), Option<usize>>`
- Type123: `&'a [Class]`
- Type124: `Option<&'a str>`
- Type125: `&'b str`
- Type126: `Vec<&'a str>`
- Type127: `Option<Target<'a>>`
- Type128: `&mut BTreeSet<(String, String)>`
- Type129: `(&'a str)`
- Type130: `&SymbolTable`
- Type131: `&HashMap<String, f64>`
- Type132: `std::io::Result<()>`
- Type133: `&mut KnowledgeGraph`
- Type134: `&mut SymbolTable`
- Type135: `&mut NamespaceIndex`
- Type136: `&'a SymbolTable`
- Type137: `&HashMap<String, Vec<String>>`
- Type138: `&crate::config::RawCall`
- Type139: `Vec<Vec<(usize, f64)>>`
- Type140: `&AdjList`
- Type141: `&mut AssemblyIndex`
- Type142: `&AssemblyIndex`
- Type143: `(f64, usize)`
- Type144: `Result<AnalysisResult, Box<dyn std::error::Error>>`
- Type145: `Python<'_>`
- Type146: `&Bound<'_, PyDict>`
- Type147: `PyResult<()>`
- Type148: `&Bound<'_, PyModule>`
- Type149: `str | None`
- Type150: `tuple[str, ...]`
- Type151: `void*`
- Type152: `uint32_t*`
- Type153: `const void*`
- Type154: `const uint16_t*`
- Type155: `const uint32_t*`
- Type156: `const TSParseActionEntry*`
- Type157: `const char**`
- Type158: `const TSMapSlice*`
- Type159: `const TSFieldMapEntry*`
- Type160: `const TSSymbolMetadata*`
- Type161: `const TSSymbol*`
- Type162: `const TSLexerMode*`
- Type163: `struct { const bool *states; const TSSymbol *symbol_map; void *(*create)(void); void
  (*destroy)(void *); bool (*scan)(void *, TSLexer *, const bool *symbol_whitelist); unsigned
  (*serialize)(void *, char *); void (*deserialize)(void *, const char *, unsigned); }`
- Type164: `const bool*`
- Type165: `const TSStateId*`
- Type166: `const char*`
- Type167: `struct { uint8_t type; TSStateId state; bool extra; bool repetition; }`
- Type168: `struct { uint8_t type; uint8_t child_count; TSSymbol symbol; int16_t dynamic_precedence;
  uint16_t production_id; }`
- Type169: `struct { uint8_t count; bool reusable; }`
- Type170: `const TSCharacterRange*`

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
- Signature11: `-uncertain_bindings(node: Node<'_>, root: Node<'_>, source: &[u8], bindings: &mut
  PythonBindings, annotation_imports: &BTreeSet<usize>) ()`
- Signature12: `-push_field(node: Node<'_>, source: &[u8], index: usize, name: String, diagram: &mut
  ClassDiagram, detection: &frameworks::Detection) ()`
- Signature13: `-instance_fields(node: Node<'_>, source: &[u8], index: usize, diagram: &mut
  ClassDiagram, detection: &frameworks::Detection) ()`
- Signature14: `+rust_evidence(node: Node<'_>, source: &[u8], file: &str, detection: &mut
  RustDetection) Option<TestEvidence>`
- Signature15: `-extract_class_members(body_node: &Node, file_path: &str, source: &[u8], symbols:
  &mut Vec<Symbol>, parent_name: &str, lang: &str) ()`
- Signature16: `-type_edges(classes: &[Class], all_classes: &[Class], hidden_ids: &BTreeSet<&str>,
  python_types: &python::PythonTypes<'_>, warnings: &mut BTreeSet<String>) BTreeSet<DiagramEdge>`
- Signature17: `+build_result(config: &AnalysisConfig, kg: &KnowledgeGraph, _st: &SymbolTable,
  timings: &HashMap<String, f64>, total_ms: f64) AnalysisResult`
- Signature18: `-call_target_in_file(st: &'a SymbolTable, kg: &KnowledgeGraph, source_id: &str,
  file: &str, name: &str) Option<&'a str>`
- Signature19: `-is_interface_self_call(caller_name: &str, callee_name: &str, target_id: &str, kg:
  &KnowledgeGraph) bool`
- Signature20: `-find_implementation(callee_name: &str, interface_target_id: &str, st: &SymbolTable,
  import_map: &HashMap<String, Vec<String>>, file_path: &str, kg: &KnowledgeGraph) Option<String>`
- Signature21: `-resolve_call(raw_call: &crate::config::RawCall, file_path: &str, st: &SymbolTable,
  import_map: &HashMap<String, Vec<String>>, kg: &KnowledgeGraph, field_type_map: &HashMap<String,
  String>) Option<CallEdge>`
- Signature22: `-disambiguate_label(label: &str, members: &[String], kg: &KnowledgeGraph,
  used_labels: &HashSet<String>) String`
- Signature23: `-process_source_imports(config: &AnalysisConfig, kg: &mut KnowledgeGraph, st: &mut
  SymbolTable, assembly_index: &AssemblyIndex, ns_index: &mut NamespaceIndex) ()`
- Signature24: `-resolve_python_import(target_name: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature25: `-resolve_python_relative(target_name: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature26: `-resolve_ts_import(target_name: &str, source_file: &str, file_set: &HashSet<String>)
  Option<String>`
- Signature27: `-resolve_java_import(target_name: &str, source_file: &str, file_set:
  &HashSet<String>, basename_index: &HashMap<String, Vec<String>>) Option<String>`
- Signature28: `-resolve_go_import(target_name: &str, go_module: Option<&str>, go_dir_index:
  &HashMap<String, Vec<String>>) Vec<String>`
- Signature29: `-resolve_rust_import(target_name: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature30: `-resolve_c_include(target_name: &str, statement: &str, source_file: &str, file_set:
  &HashSet<String>) Option<String>`
- Signature31: `-resolve_fallback(target_name: &str, _source_file: &str, st: &SymbolTable,
  assembly_index: &AssemblyIndex, kg: &KnowledgeGraph) Option<String>`
- Signature32: `-bfs_traces(kg: &KnowledgeGraph, start: &str, max_depth: usize, max_branching:
  usize, min_steps: usize) Vec<Vec<String>>`
- Signature33: `-new(repo_path: String, output_path: Option<String>, languages: Option<Vec<String>>,
  resolution: f64, max_processes: usize, max_depth: usize, max_branching: usize, min_steps: usize,
  exclude_patterns: Vec<String>, verbose: bool, quiet: bool, max_file_size: u64, max_community_size:
  usize) Self`
- Signature34: `-analyze(py: Python<'_>, path: &str, config: Option<PyAnalysisConfig>, progress:
  Option<PyObject>) PyResult<Py<PyDict>>`
- Signature35: `-export_mermaid(py: Python<'_>, result: &Bound<'_, PyDict>, path: &str, max_classes:
  usize, tests: &str, test_paths: Option<Vec<String>>, keep_paths: Option<Vec<String>>,
  explain_tests: bool) PyResult<String>`
- Signature36: `+export_cmd(input_path: unknown, output_path: unknown, output_format: unknown, path:
  unknown, max_classes: unknown, tests: unknown, test_paths: unknown, keep_paths: unknown,
  explain_tests: unknown) unknown`
- Signature37: `+analyze_cmd(path: str, output_path: str | None, languages: str | None, resolution:
  float, max_processes: int, max_depth: int, exclude: tuple[str, ...], verbose: bool, quiet: bool)
  None`
- Signature38: `_array__erase(self_contents: void*, size: uint32_t*, element_size: size_t, index:
  uint32_t) void`
- Signature39: `_array__reserve(contents: void*, capacity: uint32_t*, element_size: size_t,
  new_capacity: uint32_t) void*`
- Signature40: `_array__assign(self_contents: void*, self_size: uint32_t*, self_capacity: uint32_t*,
  other_contents: const void*, other_size: uint32_t, element_size: size_t) void*`
- Signature41: `_array__swap(self_size: uint32_t*, self_capacity: uint32_t*, other_size: uint32_t*,
  other_capacity: uint32_t*) void`
- Signature42: `_array__grow(contents: void*, size: uint32_t, capacity: uint32_t*, count: uint32_t,
  element_size: size_t) void*`
- Signature43: `_array__splice(self_contents: void*, size: uint32_t*, capacity: uint32_t*,
  element_size: size_t, index: uint32_t, old_count: uint32_t, new_count: uint32_t, elements: const
  void*) void*`

## Extraction warnings

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
