# Mermaid class diagrams

Declared types and member names; signatures and member types are hidden.
Fields are associations, not lifetime ownership. Calls are static heuristic estimates.
Members and connections within each view are uncapped.
Parallel arrows are grouped by relationship meaning.
Connections between diagrams are listed separately.

Included: 131 boxes. Calls without in-scope endpoints: 0.

## Test filtering

Mode: exclude.
Saved detector version: 2.
Test paths: `crates/mycelium-cli/tests`, `crates/mycelium-core/tests`, `tests`.
Keep paths: none.
Calls removed by test filtering: 1451. Type relationships removed: 214.

- `crates/mycelium-cli/tests/mermaid.rs`: 4 source occurrences (test-path).
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
- `crates/mycelium-core/tests/test_compact_mermaid.rs`: 15 source occurrences (test-path).
- `crates/mycelium-core/tests/test_construction_calls.rs`: 17 source occurrences (test-path).
- `crates/mycelium-core/tests/test_framework_filtering.rs`: 25 source occurrences (test-path).
- `crates/mycelium-core/tests/test_imports.rs`: 56 source occurrences (test-path).
- `crates/mycelium-core/tests/test_languages.rs`: 196 source occurrences (test-path).
- `crates/mycelium-core/tests/test_mermaid.rs`: 41 source occurrences (test-path).
- `crates/mycelium-core/tests/test_namespace_index.rs`: 7 source occurrences (test-path).
- `crates/mycelium-core/tests/test_parsing.rs`: 28 source occurrences (test-path).
- `crates/mycelium-core/tests/test_pipeline.rs`: 14 source occurrences (test-path).
- `crates/mycelium-core/tests/test_processes.rs`: 19 source occurrences (test-path).
- `crates/mycelium-core/tests/test_python_type_bindings.rs`: 24 source occurrences (test-path).
- `crates/mycelium-core/tests/test_structure.rs`: 9 source occurrences (test-path).
- `crates/mycelium-core/tests/test_test_filtering.rs`: 21 source occurrences (test-path).
- `tests/check_wheel.py`: 2 source occurrences (test-path).
- `tests/checkpoints/check_mermaid.py`: 4 source occurrences (test-path).
- `tests/fixtures/c_simple/main.c`: 9 source occurrences (test-path).
- `tests/fixtures/c_simple/repository.c`: 7 source occurrences (test-path).
- `tests/fixtures/c_simple/repository.h`: 11 source occurrences (test-path).
- `tests/fixtures/c_simple/service.c`: 9 source occurrences (test-path).
- `tests/fixtures/c_simple/service.h`: 15 source occurrences (test-path).
- `tests/fixtures/c_simple/types.c`: 4 source occurrences (test-path).
- `tests/fixtures/c_simple/types.h`: 12 source occurrences (test-path).
- `tests/fixtures/compact_csharp/Model.cs`: 19 source occurrences (test-path).
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
- `tests/test_bindings.py`: 11 source occurrences (test-path).
- `tests/test_checkpoint_exports.py`: 3 source occurrences (test-path).
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
        -command
    }
    class c0001["Commands"] {
        <<enum>>
        -Export
        -Analyze
    }
    class c0002["crates/mycelium-cli/src/main.rs"] {
        <<module>>
        -main()
        -run_quiet()
        -run_with_progress()
        -run_export()
    }
    class c0004["AnalysisConfig"] {
        <<struct>>
        +repo_path
        +output_path
        +languages
        +resolution
        +max_processes
        +max_depth
        +max_branching
        +min_steps
        +exclude_patterns
        +verbose
        +quiet
        +max_file_size
        +max_community_size
        +default()
        +from()
    }
    class c0005["AnalysisResult"] {
        <<struct>>
        +class_diagram
        +version
        +metadata
        +stats
        +structure
        +symbols
        +imports
        +calls
        +communities
        +processes
        +default()
    }
    class c0006["CallEdge"] {
        <<struct>>
        +from_symbol
        +to_symbol
        +confidence
        +tier
        +reason
        +line
    }
    class c0007["CallOutput"] {
        <<struct>>
        +from
        +to
        +confidence
        +tier
        +reason
        +line
    }
    class c0008["Community"] {
        <<struct>>
        +id
        +label
        +members
        +cohesion
        +primary_language
    }
    class c0009["CommunityOutput"] {
        <<struct>>
        +id
        +label
        +members
        +cohesion
        +primary_language
    }
    class c0010["FileNode"] {
        <<struct>>
        +path
        +language
        +size
        +lines
    }
    class c0011["FileOutput"] {
        <<struct>>
        +path
        +language
        +size
        +lines
    }
    class c0012["FolderNode"] {
        <<struct>>
        +path
        +file_count
    }
    class c0013["FolderOutput"] {
        <<struct>>
        +path
        +file_count
    }
    class c0014["ImportEdge"] {
        <<struct>>
        +from_file
        +to_file
        +statement
    }
    class c0015["ImportOutput"] {
        <<struct>>
        +from
        +to
        +statement
    }
    class c0016["ImportStatement"] {
        <<struct>>
        +file
        +statement
        +target_name
        +line
    }
    class c0017["ImportsOutput"] {
        <<struct>>
        +file_imports
        +project_references
        +package_references
    }
    class c0018["PackageRefOutput"] {
        <<struct>>
        +project
        +package
        +version
    }
    class c0019["PackageReference"] {
        <<struct>>
        +project
        +package
        +version
    }
    class c0020["Process"] {
        <<struct>>
        +id
        +entry
        +terminal
        +steps
        +process_type
        +total_confidence
    }
    class c0021["ProcessOutput"] {
        <<struct>>
        +id
        +entry
        +terminal
        +steps
        +process_type
        +total_confidence
    }
    class c0022["ProjectRefOutput"] {
        <<struct>>
        +from
        +to
        +ref_type
    }
    class c0023["ProjectReference"] {
        <<struct>>
        +from_project
        +to_project
        +ref_type
    }
    class c0024["RawCall"] {
        <<struct>>
        +caller_file
        +caller_name
        +callee_name
        +line
        +qualifier
    }
    class c0025["StructureOutput"] {
        <<struct>>
        +files
        +folders
    }
    class c0026["Symbol"] {
        <<struct>>
        +id
        +name
        +symbol_type
        +file
        +line
        +visibility
        +exported
        +parent
        +language
        +byte_range
        +parameter_types
    }
    class c0027["SymbolOutput"] {
        <<struct>>
        +id
        +name
        +symbol_type
        +file
        +line
        +visibility
        +exported
        +parent
        +language
    }
    class c0028["SymbolType"] {
        <<enum>>
        -Class
        -Function
        -Method
        -Interface
        -Struct
        -Enum
        -Namespace
        -Property
        -Constructor
        -Module
        -Record
        -Delegate
        -TypeAlias
        -Constant
        -Variable
        -Trait
        -Impl
        -Macro
        -Template
        -Typedef
        -Annotation
        -Static
        +as_str()
        +from_str_value()
        +fmt()
    }
    class c0029["Visibility"] {
        <<enum>>
        -Public
        -Private
        -Internal
        -Protected
        -Friend
        -Unknown
        +as_str()
        +fmt()
    }
    class c0030["crates/mycelium-core/src/config.rs"] {
        <<module>>
        -default_project_ref_type()
        -default_process_type()
        -default_resolution()
        -default_max_processes()
        -default_max_depth()
        -default_max_branching()
        -default_min_steps()
        -default_max_file_size()
        -default_max_community_size()
        -default_version()
    }
    class c0031["Base"] {
        <<struct>>
        +name
        +relation
    }
    class c0032["Class"] {
        <<struct>>
        +id
        +name
        +kind
        +file
        +line
        +test
        +members
        +bases
    }
    class c0033["ClassDiagram"] {
        <<struct>>
        +classes
        +warnings
        +test_detection
        +python_bindings
    }
    class c0034["Member"] {
        <<struct>>
        +file
        +name
        +kind
        +visibility
        +parameters
        +value_type
        +line
        +end_line
        +test
    }
    class c0035["Parameter"] {
        <<struct>>
        +name
        +value_type
    }
    class c0036["PythonBinding"] {
        <<enum>>
        -Import
        -Module
        -Modules
        -Class
        -Unknown
    }
    class c0037["PythonBindings"] {
        <<struct>>
        +names
        +uncertain
    }
    class c0038["TestDetection"] {
        <<struct>>
        +version
        +diagnostics
        +default()
    }
    class c0039["TestDiagnostic"] {
        <<struct>>
        +file
        +message
    }
    class c0040["TestEvidence"] {
        <<struct>>
        +rule
        +file
        +line
    }
    class c0041["crates/mycelium-core/src/declarations.rs"] {
        <<module>>
        +supported_test_rule()
    }
    class c0042["AssemblyIndex"] {
        <<struct>>
        -ns_to_project
        +new()
        +register()
        +resolve_namespace()
        +get_all_namespaces()
        +default()
    }
    class c0044["ProjectFile"] {
        <<struct>>
        +name
        +target_framework
        +root_namespace
        +assembly_name
        +project_references
        +package_references
    }
    class c0045["crates/mycelium-core/src/dotnet/project.rs"] {
        <<module>>
        +parse_project_file()
        -extract_element_text()
        -extract_include_attrs()
        -extract_package_refs()
        -extract_attr()
    }
    class c0046["SlnProject"] {
        <<struct>>
        +name
        +path
        +project_type_guid
        +project_guid
    }
    class c0047["crates/mycelium-core/src/dotnet/solution.rs"] {
        <<module>>
        +parse_solution()
    }
    class c0048["CallInfo"] {
        <<struct>>
        +id
        +confidence
        +tier
        +reason
        +line
    }
    class c0049["EdgeData"] {
        <<enum>>
        -Defines
        -Imports
        -Calls
        -ProjectReference
        -PackageReference
        -MemberOf
        -Step
        -Contains
        +edge_type()
    }
    class c0050["KnowledgeGraph"] {
        <<struct>>
        +class_diagram
        -graph
        -id_index
        +new()
        -ensure_node()
        +get_node_index()
        +get_node_data()
        +has_node()
        +add_file()
        +add_folder()
        +add_symbol()
        +add_call()
        +add_import()
        +add_project_reference()
        +add_package_reference()
        +add_community()
        +add_process()
        +get_files()
        +get_folders()
        +get_symbols()
        +get_symbols_in_file()
        +get_callers()
        +get_callees()
        +get_call_edges()
        +get_import_edges()
        +get_project_references()
        +get_package_references()
        +get_communities()
        +get_processes()
        +symbol_count()
        +file_count()
        +folder_count()
        -node_id()
        +inner_graph()
        +id_index()
        +default()
    }
    class c0051["NodeData"] {
        <<enum>>
        -File
        -Folder
        -Symbol
        -Community
        -Process
        -Package
        -Project
        +node_type()
    }
    class c0052["SymbolInfo"] {
        <<struct>>
        +id
        +name
        +symbol_type
        +file
        +line
        +visibility
        +exported
        +parent
        +language
        +parameter_types
    }
    class c0054["NamespaceIndex"] {
        <<struct>>
        -ns_to_files
        -file_to_ns
        -file_imports
        +new()
        +register()
        +get_files_for_namespace()
        +register_file_import()
        +get_imported_namespaces()
        +get_namespaces_for_file()
        +default()
    }
    class c0056["crates/mycelium-core/src/graph/scoring.rs"] {
        <<module>>
        -probe_depth()
        +score_entry_points()
    }
    class c0057["SymbolDefinition"] {
        <<struct>>
        +symbol_id
        +name
        +file
        +symbol_type
        +language
        +parent
    }
    class c0058["SymbolTable"] {
        <<struct>>
        -file_index
        -global_index
        +new()
        +add()
        +lookup_exact()
        +lookup_fuzzy()
        +get_symbols_in_file()
        +file_index()
        +global_index()
        +default()
    }
    class c0060["CAnalyser"] {
        <<struct>>
        +default()
        +new()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0061["CppAnalyser"] {
        <<struct>>
        +default()
        +new()
        -extract_cpp_symbols()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0062["crates/mycelium-core/src/languages/c_cpp.rs"] {
        <<module>>
        -is_preproc_container()
        -get_func_name()
        -get_qualified_func_name()
        -get_type_name()
        -extract_c_symbols()
        -extract_includes()
        -find_c_calls()
        -extract_c_callee()
        -find_enclosing_func()
    }
    class c0063["CSharpAnalyser"] {
        <<struct>>
        +default()
        +new()
        -walk_node()
        -extract_using()
        -find_calls()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0064["crates/mycelium-core/src/languages/csharp.rs"] {
        <<module>>
        -node_to_symbol_type()
        -is_container()
        -get_visibility()
        -get_name()
        -extract_parameter_types()
        -extract_callee()
        -find_enclosing_method()
    }
    class c0065["crates/mycelium-core/src/languages/declarations/c_cpp.rs"] {
        <<module>>
        +walk()
        -find_function()
        -declarator_name()
        -declaration_type()
    }
    class c0066["crates/mycelium-core/src/languages/declarations/frameworks/attributes.rs"] {
        <<module>>
        +detect()
        -guard_name()
        -key()
        +type_node()
        +scope()
        -scope_inner()
        -ancestor()
        +inherited_scope()
        -clean()
        -rule()
    }
    class c0067["crates/mycelium-core/src/languages/declarations/frameworks/java.rs"] {
        <<module>>
        +detect()
    }
    class c0068["crates/mycelium-core/src/languages/declarations/frameworks/javascript.rs"] {
        <<module>>
        +detect()
        -inline()
        -literal()
        -bound_names()
    }
    class c0069["Bindings"] {
        <<struct>>
        -declared
        -guards
        +extend()
    }
    class c0070["Detection"] {
        <<struct>>
        -marks
        +bindings
        +syntax_valid
        +new()
        -mark()
        +evidence()
    }
    class c0071["crates/mycelium-core/src/languages/declarations/frameworks/mod.rs"] {
        <<module>>
        -visit()
        +finish()
        -keyword()
    }
    class c0072["crates/mycelium-core/src/languages/declarations/frameworks/python.rs"] {
        <<module>>
        +detect()
        -bound_names()
    }
    class c0073["crates/mycelium-core/src/languages/declarations/go.rs"] {
        <<module>>
        +walk()
    }
    class c0074["crates/mycelium-core/src/languages/declarations/mod.rs"] {
        <<module>>
        +extract()
        -text()
        -field()
        -children()
        -add_class()
        -module()
        -rust_walk()
        -enum_member()
    }
    class c0075["crates/mycelium-core/src/languages/declarations/nominal.rs"] {
        <<module>>
        +walk()
        -visibility()
        -add_field()
        -type_field()
        -base_types()
    }
    class c0076["crates/mycelium-core/src/languages/declarations/python.rs"] {
        <<module>>
        +bindings()
        -import_bindings()
        -typing_guard()
        -uncertain_bindings()
        -block_bound_names()
        -import_target()
        -insert_binding()
        +walk()
        -push_field()
        -instance_fields()
    }
    class c0077["RustDetection"] {
        <<struct>>
        +syntax_valid
        +uncertain_test_binding
        +skipped_test
        +new()
    }
    class c0078["crates/mycelium-core/src/languages/declarations/test_detection.rs"] {
        <<module>>
        +malformed()
        -uncertain_import()
        -binds_test()
        +rust_evidence()
        -evidence()
        -tokens()
    }
    class c0079["crates/mycelium-core/src/languages/declarations/vbnet.rs"] {
        <<module>>
        +walk()
        -vb_type()
    }
    class c0080["GoAnalyser"] {
        <<struct>>
        +default()
        +new()
        -get_name_by_kind()
        -is_exported()
        -extract_string()
        -extract_string_content()
        -find_calls()
        -extract_callee()
        -find_enclosing()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0081["JavaAnalyser"] {
        <<struct>>
        +default()
        +new()
        -walk_node()
        -find_calls()
        -extract_callee()
        -find_enclosing()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0082["crates/mycelium-core/src/languages/java.rs"] {
        <<module>>
        -node_to_symbol_type()
        -is_container()
        -get_visibility()
        -get_name()
    }
    class c0083["AnalyserRegistry"] {
        <<struct>>
        -analysers
        -extension_map
        +new()
        +get_by_extension()
        +language_for_extension()
        +extensions()
        +default()
    }
    class c0084["LanguageAnalyser"] {
        <<trait>>
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
        +get_language_for_ext()
        +is_available()
    }
    class c0085["PythonAnalyser"] {
        <<struct>>
        +default()
        +new()
        -get_name()
        -walk_node()
        -find_calls()
        -extract_callee()
        -find_enclosing()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0086["RustAnalyser"] {
        <<struct>>
        +default()
        +new()
        -get_name()
        -is_pub()
        -walk_node()
        -find_calls()
        -extract_callee()
        -find_enclosing()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0087["crates/mycelium-core/src/languages/rust_lang.rs"] {
        <<module>>
        -node_to_symbol_type()
    }
    class c0088["TypeScriptAnalyser"] {
        <<struct>>
        +default()
        +new()
        -get_ts_language()
        -get_tsx_language()
        -get_js_language()
        -language_for_path()
        -get_name()
        -walk_node()
        -extract_class_members()
        -extract_string_source()
        -find_calls()
        -extract_callee()
        -find_enclosing()
        +extensions()
        +language_name()
        +get_language()
        +get_language_for_ext()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
    }
    class c0089["crates/mycelium-core/src/languages/typescript.rs"] {
        <<module>>
        -node_to_symbol_type()
    }
    class c0090["VbNetAnalyser"] {
        <<struct>>
        +default()
        +new()
        -walk_node()
        -find_calls()
        +extensions()
        +language_name()
        +get_language()
        +extract_symbols()
        +extract_imports()
        +extract_calls()
        +builtin_exclusions()
        +is_available()
    }
    class c0091["crates/mycelium-core/src/languages/vbnet.rs"] {
        <<module>>
        -tree_sitter_vb_dotnet()
        -node_to_symbol_type()
        -is_container()
        -get_visibility()
        -get_name()
        -extract_callee()
        -find_enclosing_method()
    }
    class c0092["CallEndpoint"] {
        <<struct>>
        -owner
        -member
        -name()
        -location()
    }
    class c0093["DetailMode"] {
        <<enum>>
        -Compact
        -Full
        +from_str()
    }
    class c0094["ExportError"] {
        <<enum>>
        -MissingDeclarations
        -InvalidOptions
        -NoMatchingPath
        -NoMatchingTestPath
        -InvalidTestMode
        -InvalidDetailMode
        +fmt()
    }
    class c0095["MermaidExport"] {
        <<struct>>
        +markdown
        +notices
    }
    class c0096["MermaidOptions"] {
        <<struct>>
        +path
        +max_classes
        +tests
        +explain_tests
        +test_paths
        +keep_paths
        +detail
        +default()
    }
    class c0097["RelationKind"] {
        <<enum>>
        -Inherits
        -Implements
        -Field
        -UsesType
        -Calls
        -Constructs
        -arrow()
        -label()
    }
    class c0098["TestMode"] {
        <<enum>>
        -Exclude
        -Include
        +from_str()
    }
    class c0099["crates/mycelium-core/src/mermaid.rs"] {
        <<module>>
        +export_mermaid()
        +export_mermaid_report()
        -type_edges()
        -unique_occurrences()
        -merge_classes()
        -name_index()
        -resolve()
        -member_text()
        -type_text()
        -safe()
        -compact_name()
        -pages()
        -abbreviation()
        -wrap_prose()
        -visibility()
        -type_names()
        -language_family()
        -ordered_key()
        -markdown_code()
        -plain_member()
    }
    class c0100["TestFilter"] {
        <<struct>>
        -test_paths
        -keep_paths
        -files
        -include
        -detector_version
        -explain
        +notices
        -counts
        -explanations
        +new()
        -hidden()
        -valid_location()
        +excludes_file()
        +retain()
        +summary()
    }
    class c0101["crates/mycelium-core/src/mermaid/filtering.rs"] {
        <<module>>
        +normalize_path()
        +matches_path()
    }
    class c0102["PythonTypes"] {
        <<struct>>
        -bindings
        -modules
        -module_suffixes
        -classes
        +new()
        +resolve()
        -module()
        +reference_name()
        -module_files()
        -imported()
        -member()
        -class()
    }
    class c0103["Target"] {
        <<enum>>
        -Class
        -Module
    }
    class c0104["crates/mycelium-core/src/output.rs"] {
        <<module>>
        -get_commit_hash()
        -count_languages()
        +build_result()
        +write_output()
    }
    class c0105["crates/mycelium-core/src/phases/calls.rs"] {
        <<module>>
        +run_calls_phase()
        -is_call_target()
        -call_target_in_file()
        -build_import_map()
        -build_field_type_map()
        -is_interface_self_call()
        -is_interface_method()
        -find_implementation()
        -resolve_call()
    }
    class c0106["AdjList"] {
        <<struct>>
        -node_map
        -nodes
        -adj
        -new()
        -ensure_node()
        -add_edge()
        -total_weight()
    }
    class c0107["crates/mycelium-core/src/phases/communities.rs"] {
        <<module>>
        +run_communities_phase()
        -louvain()
        -split_oversized()
        -generate_label()
        -disambiguate_label()
        -compute_cohesion()
        -primary_language()
        -common_prefix()
    }
    class c0108["crates/mycelium-core/src/phases/imports.rs"] {
        <<module>>
        +run_imports_phase()
        -process_dotnet_projects()
        -register_observed_namespaces()
        -process_source_imports()
        -resolve_python_import()
        -resolve_python_relative()
        -resolve_ts_import()
        -resolve_java_import()
        -parse_go_mod()
        -build_go_dir_index()
        -resolve_go_import()
        -resolve_rust_import()
        -resolve_c_include()
        -resolve_fallback()
        -normalize_path()
    }
    class c0109["crates/mycelium-core/src/phases/parsing.rs"] {
        <<module>>
        +run_parsing_phase()
    }
    class c0110["crates/mycelium-core/src/phases/processes.rs"] {
        <<module>>
        +run_processes_phase()
        -bfs_traces()
        -deduplicate()
        -build_community_map()
        -classify_process()
        -compute_total_confidence()
        -sort_key()
    }
    class c0111["crates/mycelium-core/src/phases/structure.rs"] {
        <<module>>
        +run_structure_phase()
    }
    class c0112["crates/mycelium-core/src/pipeline.rs"] {
        <<module>>
        +run_pipeline()
    }
    class c0130["PyAnalysisConfig"] {
        <<struct>>
        -repo_path
        -output_path
        -languages
        -resolution
        -max_processes
        -max_depth
        -max_branching
        -min_steps
        -exclude_patterns
        -verbose
        -quiet
        -max_file_size
        -max_community_size
        -new()
    }
    class c0131["crates/mycelium-python/src/lib.rs"] {
        <<module>>
        -analyze()
        -export_mermaid()
        -version()
        -_mycelium_rust()
    }
    class c0132["mycelium/cli.py"] {
        <<module>>
        +cli()
        -_run_with_progress()
        -_run_quiet()
        +export_cmd()
        +analyze_cmd()
    }
    class c0270["vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs"] {
        <<module>>
        -main()
    }
    class c0271["vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs"] {
        <<module>>
        -tree_sitter_tree_sitter_vb_dotnet()
    }
    class c0272["vendor/tree-sitter-vb-dotnet/grammar.js"] {
        <<module>>
        commaSep()
        kw()
        commaSep1()
        ci()
    }
    class c0273["BdistWheel"] {
        <<class>>
        +get_tag()
    }
    class c0274["Build"] {
        <<class>>
        +run()
    }
    class c0275["EggInfo"] {
        <<class>>
        +find_sources()
    }
    class c0276["vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h"] {
        <<module>>
        _array__erase()
        _array__reserve()
        _array__assign()
        _array__swap()
        _array__grow()
        _array__splice()
    }
    class c0277["TSCharacterRange"] {
        <<struct>>
        start
        end
    }
    class c0278["TSFieldMapEntry"] {
        <<struct>>
        field_id
        child_index
        inherited
    }
    class c0279["TSLanguage"] {
        <<struct>>
        abi_version
        symbol_count
        alias_count
        token_count
        external_token_count
        state_count
        large_state_count
        production_id_count
        field_count
        max_alias_sequence_length
        parse_table
        small_parse_table
        small_parse_table_map
        parse_actions
        symbol_names
        field_names
        field_map_slices
        field_map_entries
        symbol_metadata
        public_symbol_map
        alias_map
        alias_sequences
        lex_modes
        keyword_capture_token
        external_scanner
        states
        symbol_map
        primary_state_ids
        name
        reserved_words
        max_reserved_word_set_size
        supertype_count
        supertype_symbols
        supertype_map_slices
        supertype_map_entries
        metadata
    }
    class c0280["TSLanguageMetadata"] {
        <<struct>>
        major_version
        minor_version
        patch_version
    }
    class c0281["TSLexMode"] {
        <<struct>>
        lex_state
        external_lex_state
    }
    class c0282["TSLexer"] {
        <<struct>>
        lookahead
        result_symbol
    }
    class c0283["TSLexerMode"] {
        <<struct>>
        lex_state
        external_lex_state
        reserved_word_set_id
    }
    class c0284["TSMapSlice"] {
        <<struct>>
        index
        length
    }
    class c0285["TSParseAction"] {
        <<union>>
        shift
        type
        state
        extra
        repetition
        reduce
        child_count
        symbol
        dynamic_precedence
        production_id
    }
    class c0286["TSParseActionEntry"] {
        <<union>>
        action
        entry
        count
        reusable
    }
    class c0287["TSParseActionType"] {
        <<enum>>
        TSParseActionTypeShift
        TSParseActionTypeReduce
        TSParseActionTypeAccept
        TSParseActionTypeRecover
    }
    class c0288["TSSymbolMetadata"] {
        <<struct>>
        visible
        named
        supertype
    }
    class c0289["vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h"] {
        <<module>>
        set_contains()
    }
    c0000 --> c0001 : field
    c0001 --> c0034 : field
    c0002 ..> c0002 : calls
    c0002 ..> c0004 : uses type
    c0002 ..> c0042 : calls
    c0002 ..> c0099 : calls
    c0002 ..> c0104 : calls
    c0002 ..> c0112 : calls
    c0004 ..> c0130 : uses type
    c0005 --> c0007 : field
    c0005 --> c0009 : field
    c0005 --> c0017 : field
    c0005 --> c0021 : field
    c0005 --> c0025 : field
    c0005 --> c0027 : field
    c0005 ..> c0030 : calls
    c0005 ..> c0038 : calls
    c0017 --> c0015 : field
    c0017 --> c0018 : field
    c0017 --> c0022 : field
    c0025 --> c0011 : field
    c0025 --> c0013 : field
    c0026 --> c0028 : field
    c0026 --> c0029 : field
    c0029 ..> c0029 : calls
    c0032 --> c0031 : field
    c0032 --> c0034 : field
    c0032 --> c0040 : field
    c0033 --> c0032 : field
    c0033 --> c0037 : field
    c0033 --> c0038 : field
    c0034 --> c0035 : field
    c0034 --> c0040 : field
    c0037 --> c0036 : field
    c0038 --> c0039 : field
    c0042 ..> c0028 : calls
    c0042 ..> c0042 : calls
    c0045 ..> c0028 : calls
    c0045 ..> c0042 : calls
    c0045 ..> c0044 : uses type
    c0045 ..> c0045 : calls
    c0047 ..> c0046 : uses type
    c0050 ..> c0006 : uses type
    c0050 ..> c0008 : uses type
    c0050 ..> c0010 : uses type
    c0050 ..> c0012 : uses type
    c0050 ..> c0014 : uses type
    c0050 ..> c0019 : uses type
    c0050 ..> c0020 : uses type
    c0050 ..> c0023 : uses type
    c0050 ..> c0026 : uses type
    c0050 ..> c0028 : calls
    c0050 ..> c0042 : calls
    c0050 ..> c0048 : uses type
    c0050 --> c0049 : field
    c0050 ..> c0049 : uses type
    c0050 ..> c0050 : calls
    c0050 --> c0051 : field
    c0050 ..> c0051 : uses type
    c0050 ..> c0052 : uses type
    c0050 ..> c0106 : calls
    c0054 ..> c0054 : calls
    c0056 ..> c0028 : calls
    c0056 ..> c0050 : uses type
    c0056 ..> c0050 : calls
    c0056 ..> c0056 : calls
    c0058 ..> c0026 : uses type
    c0058 ..> c0028 : calls
    c0058 --> c0057 : field
    c0058 ..> c0057 : uses type
    c0058 ..> c0058 : calls
    c0060 ..> c0016 : uses type
    c0060 ..> c0024 : uses type
    c0060 ..> c0026 : uses type
    c0060 ..|> c0084 : implements
    c0061 ..> c0016 : uses type
    c0061 ..> c0024 : uses type
    c0061 ..> c0026 : uses type
    c0061 ..> c0061 : calls
    c0061 ..> c0062 : calls
    c0061 ..|> c0084 : implements
    c0062 ..> c0016 : uses type
    c0062 ..> c0024 : uses type
    c0062 ..> c0026 : uses type
    c0062 ..> c0062 : calls
    c0063 ..> c0016 : uses type
    c0063 ..> c0024 : uses type
    c0063 ..> c0026 : uses type
    c0063 ..> c0063 : calls
    c0063 ..> c0064 : calls
    c0063 ..> c0080 : calls
    c0063 ..> c0081 : calls
    c0063 ..|> c0084 : implements
    c0064 ..> c0028 : uses type
    c0064 ..> c0028 : calls
    c0064 ..> c0029 : uses type
    c0065 ..> c0033 : uses type
    c0065 ..> c0065 : calls
    c0065 ..> c0073 : calls
    c0065 ..> c0074 : calls
    c0066 ..> c0028 : calls
    c0066 ..> c0066 : calls
    c0066 ..> c0070 : uses type
    c0066 ..> c0070 : calls
    c0066 ..> c0071 : calls
    c0066 ..> c0074 : calls
    c0067 ..> c0004 : calls
    c0067 ..> c0028 : calls
    c0067 ..> c0066 : calls
    c0067 ..> c0070 : uses type
    c0067 ..> c0070 : calls
    c0067 ..> c0071 : calls
    c0067 ..> c0074 : calls
    c0068 ..> c0068 : calls
    c0068 ..> c0070 : uses type
    c0068 ..> c0070 : calls
    c0068 ..> c0071 : calls
    c0068 ..> c0072 : calls
    c0068 ..> c0074 : calls
    c0068 ..> c0100 : calls
    c0069 --> c0040 : field
    c0070 ..> c0004 : calls
    c0070 --> c0040 : field
    c0070 ..> c0040 : uses type
    c0070 ..> c0066 : calls
    c0070 --> c0069 : field
    c0070 ..> c0078 : calls
    c0071 ..> c0065 : calls
    c0071 ..> c0069 : uses type
    c0071 ..> c0069 : calls
    c0071 ..> c0070 : calls
    c0071 ..> c0074 : calls
    c0072 ..> c0042 : calls
    c0072 ..> c0065 : calls
    c0072 ..> c0068 : calls
    c0072 ..> c0070 : uses type
    c0072 ..> c0070 : calls
    c0072 ..> c0071 : calls
    c0072 ..> c0072 : calls
    c0072 ..> c0074 : calls
    c0072 ..> c0099 : calls
    c0072 ..> c0100 : calls
    c0073 ..> c0033 : uses type
    c0073 ..> c0065 : calls
    c0073 ..> c0074 : calls
    c0074 ..> c0004 : calls
    c0074 ..> c0033 : uses type
    c0074 ..> c0042 : calls
    c0074 ..> c0065 : calls
    c0074 ..> c0069 : calls
    c0074 ..> c0074 : calls
    c0074 ..> c0076 : calls
    c0074 ..> c0078 : calls
    c0075 ..> c0033 : uses type
    c0075 ..> c0065 : calls
    c0075 ..> c0070 : calls
    c0075 ..> c0074 : calls
    c0075 ..> c0075 : calls
    c0076 ..> c0032 : constructs
    c0076 ..> c0033 : uses type
    c0076 ..> c0036 : uses type
    c0076 ..> c0037 : uses type
    c0076 ..> c0065 : calls
    c0076 ..> c0069 : calls
    c0076 ..> c0070 : calls
    c0076 ..> c0074 : calls
    c0076 ..> c0076 : calls
    c0077 ..> c0078 : calls
    c0078 ..> c0040 : uses type
    c0078 ..> c0065 : calls
    c0078 ..> c0074 : calls
    c0078 ..> c0077 : uses type
    c0078 ..> c0078 : calls
    c0079 ..> c0033 : uses type
    c0079 ..> c0065 : calls
    c0079 ..> c0070 : calls
    c0079 ..> c0074 : calls
    c0079 ..> c0079 : calls
    c0080 ..> c0016 : uses type
    c0080 ..> c0024 : uses type
    c0080 ..> c0026 : uses type
    c0080 ..> c0063 : calls
    c0080 ..> c0080 : calls
    c0080 ..|> c0084 : implements
    c0081 ..> c0016 : uses type
    c0081 ..> c0024 : uses type
    c0081 ..> c0026 : uses type
    c0081 ..> c0063 : calls
    c0081 ..> c0081 : calls
    c0081 ..> c0082 : calls
    c0081 ..|> c0084 : implements
    c0082 ..> c0028 : uses type
    c0082 ..> c0028 : calls
    c0082 ..> c0029 : uses type
    c0083 ..> c0028 : calls
    c0083 ..> c0060 : calls
    c0083 ..> c0083 : calls
    c0083 --> c0084 : field
    c0083 ..> c0084 : uses type
    c0083 ..> c0090 : calls
    c0084 ..> c0016 : uses type
    c0084 ..> c0024 : uses type
    c0084 ..> c0026 : uses type
    c0085 ..> c0016 : uses type
    c0085 ..> c0024 : uses type
    c0085 ..> c0026 : uses type
    c0085 ..> c0063 : calls
    c0085 ..|> c0084 : implements
    c0085 ..> c0085 : calls
    c0086 ..> c0016 : uses type
    c0086 ..> c0024 : uses type
    c0086 ..> c0026 : uses type
    c0086 ..> c0063 : calls
    c0086 ..|> c0084 : implements
    c0086 ..> c0086 : calls
    c0086 ..> c0087 : calls
    c0087 ..> c0028 : uses type
    c0088 ..> c0016 : uses type
    c0088 ..> c0024 : uses type
    c0088 ..> c0026 : uses type
    c0088 ..> c0063 : calls
    c0088 ..|> c0084 : implements
    c0088 ..> c0088 : calls
    c0088 ..> c0089 : calls
    c0089 ..> c0028 : uses type
    c0090 ..> c0016 : uses type
    c0090 ..> c0024 : uses type
    c0090 ..> c0026 : uses type
    c0090 ..> c0063 : calls
    c0090 ..|> c0084 : implements
    c0090 ..> c0090 : calls
    c0090 ..> c0091 : calls
    c0091 ..> c0028 : uses type
    c0091 ..> c0029 : uses type
    c0091 ..> c0091 : calls
    c0092 --> c0032 : field
    c0092 --> c0034 : field
    c0096 --> c0093 : field
    c0096 --> c0098 : field
    c0099 ..> c0005 : uses type
    c0099 ..> c0028 : calls
    c0099 ..> c0032 : uses type
    c0099 ..> c0034 : uses type
    c0099 ..> c0042 : calls
    c0099 ..> c0069 : calls
    c0099 ..> c0083 : calls
    c0099 ..> c0094 : uses type
    c0099 ..> c0095 : uses type
    c0099 ..> c0096 : uses type
    c0099 ..> c0097 : calls
    c0099 ..> c0099 : calls
    c0099 ..> c0100 : calls
    c0099 ..> c0101 : calls
    c0100 ..> c0005 : uses type
    c0100 ..> c0028 : calls
    c0100 ..> c0032 : uses type
    c0100 ..> c0040 : uses type
    c0100 ..> c0041 : calls
    c0100 ..> c0069 : calls
    c0100 ..> c0094 : uses type
    c0100 ..> c0096 : uses type
    c0100 ..> c0100 : calls
    c0100 ..> c0101 : calls
    c0101 ..> c0094 : uses type
    c0102 ..> c0032 : uses type
    c0102 --> c0037 : field
    c0102 ..> c0037 : uses type
    c0102 ..> c0103 : uses type
    c0104 ..> c0004 : uses type
    c0104 ..> c0005 : uses type
    c0104 ..> c0042 : calls
    c0104 ..> c0050 : uses type
    c0104 ..> c0050 : calls
    c0104 ..> c0058 : uses type
    c0104 ..> c0104 : calls
    c0105 ..> c0004 : uses type
    c0105 ..> c0006 : uses type
    c0105 ..> c0028 : calls
    c0105 ..> c0042 : calls
    c0105 ..> c0050 : uses type
    c0105 ..> c0050 : calls
    c0105 ..> c0054 : uses type
    c0105 ..> c0058 : uses type
    c0105 ..> c0058 : calls
    c0105 ..> c0060 : calls
    c0105 ..> c0083 : calls
    c0105 ..> c0088 : calls
    c0105 ..> c0090 : calls
    c0105 ..> c0105 : calls
    c0106 ..> c0106 : calls
    c0107 ..> c0004 : uses type
    c0107 ..> c0028 : calls
    c0107 ..> c0050 : uses type
    c0107 ..> c0050 : calls
    c0107 ..> c0069 : calls
    c0107 ..> c0106 : uses type
    c0107 ..> c0106 : calls
    c0107 ..> c0107 : calls
    c0108 ..> c0004 : uses type
    c0108 ..> c0028 : calls
    c0108 ..> c0042 : uses type
    c0108 ..> c0042 : calls
    c0108 ..> c0045 : calls
    c0108 ..> c0047 : calls
    c0108 ..> c0050 : uses type
    c0108 ..> c0050 : calls
    c0108 ..> c0054 : uses type
    c0108 ..> c0054 : calls
    c0108 ..> c0058 : uses type
    c0108 ..> c0058 : calls
    c0108 ..> c0060 : calls
    c0108 ..> c0083 : calls
    c0108 ..> c0088 : calls
    c0108 ..> c0090 : calls
    c0108 ..> c0108 : calls
    c0109 ..> c0004 : uses type
    c0109 ..> c0004 : calls
    c0109 ..> c0028 : calls
    c0109 ..> c0042 : calls
    c0109 ..> c0050 : uses type
    c0109 ..> c0050 : calls
    c0109 ..> c0054 : uses type
    c0109 ..> c0058 : uses type
    c0109 ..> c0058 : calls
    c0109 ..> c0060 : calls
    c0109 ..> c0069 : calls
    c0109 ..> c0071 : calls
    c0109 ..> c0074 : calls
    c0109 ..> c0083 : calls
    c0109 ..> c0088 : calls
    c0110 ..> c0004 : uses type
    c0110 ..> c0028 : calls
    c0110 ..> c0042 : calls
    c0110 ..> c0050 : uses type
    c0110 ..> c0050 : calls
    c0110 ..> c0056 : calls
    c0110 ..> c0069 : calls
    c0110 ..> c0110 : calls
    c0111 ..> c0004 : uses type
    c0111 ..> c0028 : calls
    c0111 ..> c0042 : calls
    c0111 ..> c0050 : uses type
    c0111 ..> c0050 : calls
    c0111 ..> c0083 : calls
    c0112 ..> c0004 : uses type
    c0112 ..> c0005 : uses type
    c0112 ..> c0042 : calls
    c0112 ..> c0104 : calls
    c0131 ..> c0074 : calls
    c0131 ..> c0098 : calls
    c0131 ..> c0099 : calls
    c0131 ..> c0112 : calls
    c0131 ..> c0130 : uses type
    c0132 ..> c0132 : calls
    c0270 ..> c0042 : calls
    c0272 ..> c0272 : calls
    c0279 --> c0278 : field
    c0279 --> c0280 : field
    c0279 --> c0282 : field
    c0279 --> c0283 : field
    c0279 --> c0284 : field
    c0279 --> c0286 : field
    c0279 --> c0288 : field
    c0286 --> c0285 : field
    c0289 ..> c0277 : uses type
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
- c0092: `CallEndpoint` — `crates/mycelium-core/src/mermaid.rs`:559
- c0093: `DetailMode` — `crates/mycelium-core/src/mermaid.rs`:51
- c0094: `ExportError` — `crates/mycelium-core/src/mermaid.rs`:85
- c0095: `MermaidExport` — `crates/mycelium-core/src/mermaid.rs`:32
- c0096: `MermaidOptions` — `crates/mycelium-core/src/mermaid.rs`:39
- c0097: `RelationKind` — `crates/mycelium-core/src/mermaid.rs`:527
- c0098: `TestMode` — `crates/mycelium-core/src/mermaid.rs`:14
- c0099: `crates/mycelium-core/src/mermaid.rs` — `crates/mycelium-core/src/mermaid.rs`:1
- c0100: `TestFilter` — `crates/mycelium-core/src/mermaid/filtering.rs`:9
- c0101: `crates/mycelium-core/src/mermaid/filtering.rs` —
  `crates/mycelium-core/src/mermaid/filtering.rs`:1
- c0102: `PythonTypes` — `crates/mycelium-core/src/mermaid/python.rs`:7
- c0103: `Target` — `crates/mycelium-core/src/mermaid/python.rs`:14
- c0104: `crates/mycelium-core/src/output.rs` — `crates/mycelium-core/src/output.rs`:1
- c0105: `crates/mycelium-core/src/phases/calls.rs` — `crates/mycelium-core/src/phases/calls.rs`:1
- c0106: `AdjList` — `crates/mycelium-core/src/phases/communities.rs`:99
- c0107: `crates/mycelium-core/src/phases/communities.rs` —
  `crates/mycelium-core/src/phases/communities.rs`:1
- c0108: `crates/mycelium-core/src/phases/imports.rs` —
  `crates/mycelium-core/src/phases/imports.rs`:1
- c0109: `crates/mycelium-core/src/phases/parsing.rs` —
  `crates/mycelium-core/src/phases/parsing.rs`:1
- c0110: `crates/mycelium-core/src/phases/processes.rs` —
  `crates/mycelium-core/src/phases/processes.rs`:1
- c0111: `crates/mycelium-core/src/phases/structure.rs` —
  `crates/mycelium-core/src/phases/structure.rs`:1
- c0112: `crates/mycelium-core/src/pipeline.rs` — `crates/mycelium-core/src/pipeline.rs`:1
- c0130: `PyAnalysisConfig` — `crates/mycelium-python/src/lib.rs`:12
- c0131: `crates/mycelium-python/src/lib.rs` — `crates/mycelium-python/src/lib.rs`:1
- c0132: `mycelium/cli.py` — `mycelium/cli.py`:1
- c0270: `vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs` —
  `vendor/tree-sitter-vb-dotnet/bindings/rust/build.rs`:1
- c0271: `vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs` —
  `vendor/tree-sitter-vb-dotnet/bindings/rust/lib.rs`:1
- c0272: `vendor/tree-sitter-vb-dotnet/grammar.js` — `vendor/tree-sitter-vb-dotnet/grammar.js`:1
- c0273: `BdistWheel` — `vendor/tree-sitter-vb-dotnet/setup.py`:38
- c0274: `Build` — `vendor/tree-sitter-vb-dotnet/setup.py`:30
- c0275: `EggInfo` — `vendor/tree-sitter-vb-dotnet/setup.py`:46
- c0276: `vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h` —
  `vendor/tree-sitter-vb-dotnet/src/tree_sitter/array.h`:1
- c0277: `TSCharacterRange` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:102
- c0278: `TSFieldMapEntry` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:28
- c0279: `TSLanguage` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:107
- c0280: `TSLanguageMetadata` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:21
- c0281: `TSLexMode` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:83
- c0282: `TSLexer` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:48
- c0283: `TSLexerMode` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:88
- c0284: `TSMapSlice` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:35
- c0285: `TSParseAction` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:66
- c0286: `TSParseActionEntry` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:94
- c0287: `TSParseActionType` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:59
- c0288: `TSSymbolMetadata` — `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:40
- c0289: `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h` —
  `vendor/tree-sitter-vb-dotnet/src/tree_sitter/parser.h`:1

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
- `Unresolved or out-of-scope base: DetailMode -> std::str::FromStr`
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
