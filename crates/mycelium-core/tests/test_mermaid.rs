use mycelium_core::config::AnalysisConfig;
use mycelium_core::mermaid::{export_mermaid, DetailMode, MermaidOptions};
use mycelium_core::pipeline::run_pipeline;

// Preserve the historical detailed-output assertions explicitly.
fn full_options() -> MermaidOptions {
    MermaidOptions {
        detail: DetailMode::Full,
        ..Default::default()
    }
}

#[test]
fn source_exports_typed_classes_trait_methods_and_calls() {
    // Arrange: the declarations are the independent specification for the output.
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("service.rs"),
        r#"
pub trait Store { fn save(&self, id: u64) -> String; }
pub struct Repository { pub name: String }
impl Store for Repository {
    fn save(&self, id: u64) -> String { id.to_string() }
}
pub struct Service { pub repository: Repository }
impl Service {
    pub fn run(&self, id: u64) -> String { self.repository.save(id) }
}
pub fn launch(service: Service) -> String { service.run(1) }
"#,
    )
    .unwrap();
    let config = AnalysisConfig {
        repo_path: repo.path().to_string_lossy().into_owned(),
        ..Default::default()
    };

    // Act: exercise the same analysis and export boundary used by the CLI.
    let result = run_pipeline(&config, None).unwrap();
    let markdown = export_mermaid(&result, &full_options()).unwrap();

    // Assert: check meaning, not merely that the exporter can snapshot itself.
    for expected in [
        "classDiagram",
        "[\"Repository\"]",
        "[\"Service\"]",
        "<<trait>>",
        "+repository: Repository",
        "+save(id: u64) String",
        "+run(id: u64) String",
        "..|>",
        " --> ",
        "run() calls save()",
        "<<module>>",
        "launch(service: Service) String",
    ] {
        assert!(
            markdown.contains(expected),
            "Missing {expected}:\n{markdown}"
        );
    }
    assert!(box_members(&markdown, "Repository").contains("+save(id: u64) String"));
    assert!(box_members(&markdown, "Service").contains("+run(id: u64) String"));
    let repeated = run_pipeline(&config, None).unwrap();
    assert_eq!(
        markdown,
        export_mermaid(&repeated, &full_options()).unwrap()
    );
}

fn export_source(filename: &str, source: &str) -> String {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(repo.path().join(filename), source).unwrap();
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    export_mermaid(&result, &full_options()).unwrap()
}

#[test]
fn python_exports_dataclass_fields_inheritance_and_unknown_annotations() {
    // Arrange / Act: source covers the patterns used in Forgetful and eval-harness.
    let output = export_source(
        "models.py",
        r#"
class Base: pass
class User(Base):
    name: str
    tags: list[str]
    def __init__(self, name: str):
        self.name: str = name
    def greet(self, other: str) -> str:
        return other
    def dynamic(self, value): return value
"#,
    );
    // Assert: unknown annotations stay unknown, and receivers aren't parameters.
    for value in [
        "[\"User\"]",
        "+name: str",
        "+greet(other: str) str",
        "+dynamic(value: unknown) unknown",
        "--|>",
        "list[str]",
    ] {
        assert!(output.contains(value), "Missing {value}:\n{output}");
    }
    assert!(!output.contains("self:"));
}

#[test]
fn csharp_exports_interface_implementation_fields_and_properties() {
    // Arrange / Act: common SurelyCRM class, interface and signature constructs.
    let output = export_source(
        "Service.cs",
        r#"
public interface IStore { string Save(int id); }
public class Store : IStore {
    private readonly string name;
    public int Count { get; set; }
    public string Save(int id) { return name; }
}
"#,
    );
    // Assert: distinguish declared implementation from field associations.
    for value in [
        "<<interface>>",
        "[\"Store\"]",
        "-name: string",
        "+Count: int",
        "+Save(id: int) string",
        "..|>",
    ] {
        assert!(output.contains(value), "Missing {value}:\n{output}");
    }
    assert!(box_members(&output, "Store").contains("+Save(id: int) string"));
}

#[test]
fn typescript_exports_interfaces_callbacks_and_free_functions() {
    // Arrange / Act: Pi extensions mix interfaces, classes and module functions.
    let output = export_source(
        "extension.ts",
        r#"
interface Store { save(id: number): string; }
class Client implements Store {
    count: number;
    callback: (value: string) => void;
    save(id: number): string { return String(id); }
}
export function create(count: number): Client { return new Client(); }
export const load = (id: string): Client => new Client();
"#,
    );
    // Assert: callbacks stay fields; free functions stay in their module.
    for value in [
        "<<interface>>",
        "count: number",
        "callback: Type",
        "save(id: number) string",
        "create(count: number) Client",
        "load(id: string) Client",
        "..|>",
        "<<module>>",
    ] {
        assert!(output.contains(value), "Missing {value}:\n{output}");
    }
    assert!(box_members(&output, "Client").contains("save(id: number) string"));
    assert!(box_members(&output, "extension.ts").contains("load(id: string) Client"));
    assert!(!box_members(&output, "Client").contains("load("));
}

#[test]
fn java_and_javascript_export_declared_members() {
    // Arrange / Act: both languages use class syntax with different type availability.
    let java = export_source(
        "Service.java",
        r#"
interface Store { String save(int id); }
class Service implements Store {
    private String name;
    public String save(int id) { return name; }
}
"#,
    );
    let js = export_source(
        "service.js",
        r#"
class Service {
    count = 0;
    save(id) { return id; }
}
export function create() { return new Service(); }
"#,
    );
    // Assert: declared Java types survive; JavaScript types aren't invented.
    for value in ["-name: String", "+save(id: int) String", "..|>"] {
        assert!(java.contains(value), "Missing {value}:\n{java}");
    }
    for value in [
        "[\"Service\"]",
        "count: unknown",
        "save(id: unknown) unknown",
        "create() unknown",
    ] {
        assert!(js.contains(value), "Missing {value}:\n{js}");
    }
}

#[test]
fn go_exports_receiver_ownership_without_claiming_implicit_implementation() {
    // Arrange / Act.
    let output = export_source(
        "service.go",
        r#"
package service
type Store interface { Save(id int) string }
type Service struct { Name string }
func (s *Service) Save(id int) string { return s.Name }
func Create(name string) *Service { return &Service{Name: name} }
"#,
    );
    // Assert: receiver methods are attached to their struct, not the file module.
    for value in [
        "[\"Service\"]",
        "<<interface>>",
        "+Name: string",
        "+Save(id: int) string",
        "+Create(name: string)",
    ] {
        assert!(output.contains(value), "Missing {value}:\n{output}");
    }
    assert!(!output.contains("..|>"));
    assert!(box_members(&output, "Service").contains("+Save(id: int) string"));
    assert!(!box_members(&output, "service.go").contains("Save("));
}

#[test]
fn vbnet_exports_fields_signatures_and_implementation() {
    // Arrange / Act.
    let output = export_source(
        "Store.vb",
        r#"
Public Interface IStore
    Function Save(id As Integer) As String
End Interface
Public Class Store
    Implements IStore
    Private name As String
    Public Function Save(id As Integer) As String
        Return name
    End Function
End Class
"#,
    );
    // Assert.
    for value in [
        "<<interface>>",
        "[\"Store\"]",
        "-name: String",
        "+Save(id: Integer) String",
        "..|>",
    ] {
        assert!(output.contains(value), "Missing {value}:\n{output}");
    }
    assert!(box_members(&output, "Store").contains("+Save(id: Integer) String"));
}

#[test]
fn c_and_cpp_export_fields_functions_and_out_of_line_methods() {
    // Arrange / Act.
    let c = export_source(
        "service.c",
        r#"
struct User { int id; const char *name; };
int save(int id) { return id; }
"#,
    );
    let cpp = export_source(
        "service.cpp",
        r#"
class Base { public: virtual int save(int id) = 0; };
class Store : public Base {
public:
    int count;
    int save(int id);
};
int Store::save(int id) { return id; }
"#,
    );
    // Assert.
    for value in ["[\"User\"]", "id: int", "save(id: int) int"] {
        assert!(c.contains(value), "Missing {value}:\n{c}");
    }
    for value in ["[\"Store\"]", "count: int", "save(id: int) int", "--|>"] {
        assert!(cpp.contains(value), "Missing {value}:\n{cpp}");
    }
    assert_eq!(
        box_members(&cpp, "Store")
            .matches("save(id: int) int")
            .count(),
        1
    );
    assert!(box_members(&c, "User").contains("id: int"));
}

#[test]
fn calls_in_local_initializers_belong_to_the_enclosing_function() {
    // Arrange / Act: reduced from the pinned Pi web-search checkpoint.
    let output = export_source(
        "browser.ts",
        r#"
export async function sharedBrowser(): Promise<string> { return "browser"; }
export async function openContext(): Promise<string> {
    const browser = await sharedBrowser();
    return browser;
}
"#,
    );
    // Assert.
    assert!(
        output.contains("openContext() calls sharedBrowser()"),
        "{output}"
    );
}

#[test]
fn all_connections_between_visible_boxes_are_drawn() {
    // Arrange: 85 targets exceed both the former arrow limit and the member limit.
    let repo = tempfile::tempdir().unwrap();
    let mut source = String::from("class Hub:\n");
    for i in 0..85 {
        source.push_str(&format!("    target_{i}: Target{i}\n"));
    }
    for i in 0..85 {
        source.push_str(&format!("class Target{i}: pass\n"));
    }
    std::fs::write(repo.path().join("models.py"), source).unwrap();
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();

    // Act: export a saved map into one diagram, as with the full Ferox view.
    let saved = serde_json::to_string(&result).unwrap();
    let restored = serde_json::from_str(&saved).unwrap();
    let output = export_mermaid(
        &restored,
        &MermaidOptions {
            detail: DetailMode::Full,
            max_classes: 100,
            ..Default::default()
        },
    )
    .unwrap();

    // Assert: each source field has an arrow to its own target inside the Mermaid block.
    assert_eq!(output.matches("```mermaid").count(), 1);
    let diagram = output
        .split("```mermaid")
        .nth(1)
        .unwrap()
        .split("```")
        .next()
        .unwrap();
    let box_id = |name: &str| {
        diagram
            .lines()
            .find_map(|line| {
                let (id, label) = line.trim().strip_prefix("class ")?.split_once('[')?;
                (label == format!("\"{name}\"] {{")).then_some(id)
            })
            .unwrap()
    };
    let hub = box_id("Hub");
    for i in 0..85 {
        let target = box_id(&format!("Target{i}"));
        let expected = format!("{hub} --> {target} : field target_{i}");
        assert!(
            diagram.lines().any(|line| line.trim() == expected),
            "Missing {expected}"
        );
    }
}

#[test]
fn large_classes_keep_all_members_together_and_respect_the_box_limit() {
    // Arrange: both fields and methods exceed the former 40-member limit.
    let repo = tempfile::tempdir().unwrap();
    let mut source = String::from("class User:\n");
    for i in 0..85 {
        source.push_str(&format!("    field_{i}: str\n"));
    }
    for i in 0..85 {
        source.push_str(&format!(
            "    def method_{i}(self) -> str: return self.field_{i}\n"
        ));
    }
    source.push_str("class Service:\n    user: User\n");
    std::fs::write(repo.path().join("models.py"), source).unwrap();
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    // Act: the box limit alone controls whether these two types share a diagram.
    for (max_classes, expected_diagrams) in [(2, 1), (1, 2)] {
        let options = MermaidOptions {
            detail: DetailMode::Full,
            max_classes,
            ..Default::default()
        };
        let markdown = export_mermaid(&result, &options).unwrap();

        // Assert: each type appears once, with every member in the same box.
        assert_eq!(markdown.matches("```mermaid").count(), expected_diagrams);
        assert_eq!(markdown.matches("[\"User\"]").count(), 1);
        assert_eq!(markdown.matches("[\"Service\"]").count(), 1);
        let members = box_members(&markdown, "User");
        for i in 0..85 {
            assert!(members.contains(&format!("+field_{i}: str")));
            assert!(members.contains(&format!("+method_{i}() str")));
        }
        assert!(markdown.contains("field user"));
        assert_eq!(markdown, export_mermaid(&result, &options).unwrap());
    }
}

#[test]
fn long_class_and_module_names_are_shown_in_full() {
    // Arrange: both names exceed the old display-label cutoff.
    let filename = "openai_compatible_gateway_integration_with_explicit_defaults.py";
    let class_name = "OpenAiCompatibleGatewayIntegrationConfigurationWithExplicitDefaults";
    let source = format!("class {class_name}:\n    pass\ndef run(): pass\n");

    // Act: source declarations reach the diagram through the public export seam.
    let output = export_source(filename, &source);

    // Assert: the names appear on their boxes, not only in the source index.
    assert!(box_members(&output, filename).contains("<<module>>"));
    assert!(box_members(&output, class_name).contains("<<class>>"));
}

#[test]
fn long_member_names_remain_visible_when_signatures_use_a_key() {
    // Arrange: long method and field names inside signatures that need a key.
    let method = "complete_a_request_using_the_gateway_and_preserve_the_full_method_name";
    let field = "configuration_for_the_gateway_integration_with_explicit_defaults";
    let source = format!(
        "class Gateway:
    {field}: GatewayIntegrationConfiguration
    def {method}(self, request: CompletionRequest): pass
"
    );

    // Act.
    let output = export_source("gateway.py", &source);

    // Assert: abbreviating the types/parameters retains each member's original name.
    let members = box_members(&output, "Gateway");
    assert!(
        members.contains(&format!("+{field}: Signature")),
        "{output}"
    );
    assert!(
        members.contains(&format!("+{method}(Signature")),
        "{output}"
    );
}

#[test]
fn long_signatures_are_preserved_in_a_key_with_bounded_lines() {
    // Arrange / Act.
    let output = export_source(
        "long.py",
        &format!(
            "class Example:\n    def calculate(self, {}): pass\n",
            (0..30)
                .map(|i| format!("parameter_{i}: str"))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    );
    // Assert: shortening the diagram does not discard the full signature.
    assert!(
        output.lines().all(|line| line.chars().count() <= 100),
        "{output}"
    );
    assert!(output.contains("parameter_29"));
    assert!(output.contains("Signature key"));
}

#[test]
fn implementation_methods_keep_their_source_file_for_call_endpoints() {
    // Arrange: the owning struct is declared in a different file from its implementation.
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(repo.path().join("model.rs"), "pub struct Service;\n").unwrap();
    std::fs::write(
        repo.path().join("service.rs"),
        r#"
use crate::model::Service;
impl Service {
    pub fn run(&self) { helper(); }
}
fn helper() {}
"#,
    )
    .unwrap();
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    // Act.
    let output = export_mermaid(&result, &full_options()).unwrap();
    // Assert.
    assert!(output.contains("run() calls helper()"), "{output}");
}

#[test]
fn type_aliases_are_included_without_inventing_inheritance() {
    // Arrange / Act: exported unions and object aliases are common Pi contracts.
    let output = export_source(
        "contracts.ts",
        r#"
export type Scope = "project" | "global";
export type Config = { name: string; run(id: number): void; };
"#,
    );
    // Assert.
    for value in [
        "[\"Scope\"]",
        "[\"Config\"]",
        "<<type_alias>>",
        "name: string",
        "run(id: number) void",
    ] {
        assert!(output.contains(value), "Missing {value}:\n{output}");
    }
    assert!(!output.contains("--|>"));
}

#[test]
fn scoping_and_ambiguous_type_names_never_select_an_arbitrary_owner() {
    // Arrange: duplicate names in different folders must remain distinct.
    let repo = tempfile::tempdir().unwrap();
    for path in ["a", "b", "c"] {
        std::fs::create_dir(repo.path().join(path)).unwrap();
    }
    for path in ["a/model.py", "b/model.py"] {
        std::fs::write(repo.path().join(path), "class User:\n    name: str\n").unwrap();
    }
    std::fs::write(
        repo.path().join("c/service.py"),
        "class Service:\n    user: User\n",
    )
    .unwrap();
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    // Act.
    let all = export_mermaid(&result, &full_options()).unwrap();
    let scoped = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            path: "c".into(),
            ..Default::default()
        },
    )
    .unwrap();
    // Assert: filtering must not invent a resolution or leak other boxes.
    assert_eq!(all.matches("[\"User\"]").count(), 2);
    assert!(!all.contains(" --> "));
    assert!(all.contains("Ambiguous type"), "{all}");
    assert!(scoped.contains("[\"Service\"]"));
    assert!(!scoped.contains("[\"User\"]"));
}

#[test]
fn export_is_order_independent_and_does_not_allow_source_text_to_add_statements() {
    // Arrange: JSON maps may arrive in any order, with source-controlled text.
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("types.py"),
        "class A:\n    value: str\nclass B(A): pass\n",
    )
    .unwrap();
    let mut result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    let first = export_mermaid(&result, &full_options()).unwrap();
    result.class_diagram.as_mut().unwrap().classes.reverse();
    result.symbols.reverse();
    result.calls.reverse();
    // Act / Assert.
    assert_eq!(first, export_mermaid(&result, &full_options()).unwrap());
    result.class_diagram.as_mut().unwrap().classes[0].kind = "class>>\nclick exploit".into();
    let escaped = export_mermaid(&result, &full_options()).unwrap();
    assert!(!escaped.contains("\nclick exploit"));
}

#[test]
fn parallel_relationships_are_summarized_in_diagrams_and_kept_in_the_list() {
    // Arrange / Act: parallel labels overlap in Mermaid's class diagram layout.
    let output = export_source(
        "model.py",
        r#"
class User: pass
class Repository:
    def get(self) -> User: pass
    def save(self, user: User) -> User: pass
"#,
    );
    // Assert: full labels remain below the picture, with one pictured dependency per pair.
    let diagram = output
        .split("```mermaid")
        .nth(1)
        .unwrap()
        .split("```")
        .next()
        .unwrap();
    assert_eq!(
        diagram
            .lines()
            .filter(|line| line.contains(" ..> "))
            .count(),
        1
    );
    assert!(output.contains("type in get"));
    assert!(output.contains("type in save"));
}

#[test]
fn identical_sources_export_identically_regardless_of_file_creation_order() {
    // Arrange: the same committed source can be materialized in either filesystem order.
    let files = [
        ("a.py", "def shared() -> str: return 'a'\n"),
        ("b.py", "def shared() -> str: return 'b'\n"),
        ("main.py", "def run() -> str: return shared()\n"),
    ];
    let mut outputs = Vec::new();
    // Act: use a fresh analysis for each differently materialized checkout.
    for order in [vec![0, 1, 2], vec![2, 1, 0]] {
        let repo = tempfile::tempdir().unwrap();
        for index in order {
            std::fs::write(repo.path().join(files[index].0), files[index].1).unwrap();
        }
        let result = run_pipeline(
            &AnalysisConfig {
                repo_path: repo.path().to_string_lossy().into_owned(),
                ..Default::default()
            },
            None,
        )
        .unwrap();
        outputs.push(export_mermaid(&result, &full_options()).unwrap());
    }
    // Assert: even ambiguous heuristic calls need a repeatable selection.
    assert_eq!(outputs[0], outputs[1]);
}

#[test]
fn qualified_and_literal_types_do_not_link_to_unrelated_local_names() {
    // Arrange / Act: a remote qualified type and a string literal are not local User references.
    let output = export_source(
        "remote.ts",
        r#"
class User {}
class Client {
    remote: external.User;
    label: "User";
}
"#,
    );
    // Assert: keep the types but avoid false relationships from splitting them into words.
    assert!(!output.contains(" --> "), "{output}");
}

fn box_members(markdown: &str, name: &str) -> String {
    let mut collecting = false;
    let mut lines = Vec::new();
    for line in markdown.lines() {
        if line.contains(&format!("[\"{name}\"] {{")) {
            collecting = true;
        } else if collecting && line.trim() == "}" {
            collecting = false;
        } else if collecting {
            lines.push(line);
        }
    }
    lines.join("\n")
}

#[test]
fn c_references_are_not_definitions_and_typedef_structs_keep_their_fields() {
    // Arrange / Act: reviewer reproducer includes a self-reference and anonymous typedef.
    let output = export_source(
        "types.c",
        r#"
struct node { int v; struct node *next; };
struct list { struct node *head; };
typedef struct { int x; } Point;
struct pointers { int *a, b; };
"#,
    );
    // Assert: require the right members in the right owning boxes.
    assert_eq!(output.matches("[\"node\"]").count(), 1, "{output}");
    assert!(box_members(&output, "Point").contains("x: int"), "{output}");
    assert!(
        box_members(&output, "pointers").contains("b: int"),
        "{output}"
    );
    assert!(output.contains("field head"), "{output}");
}

#[test]
fn abstract_typescript_members_and_parameter_properties_keep_their_owner() {
    let output = export_source(
        "abstract.ts",
        r#"
class Model {}
class Base<T> {}
abstract class Store extends Base<Model> {
    protected cache: Model;
    constructor(private model: Model) {}
    abstract get(): Model;
    save(value: Model): void {}
}
"#,
    );
    let members = box_members(&output, "Store");
    assert!(members.contains("#cache: Model"), "{output}");
    assert!(members.contains("-model: Model"), "{output}");
    assert!(members.contains("get() Model"), "{output}");
    assert!(members.contains("save(value: Model) void"), "{output}");
    assert_eq!(output.matches(" --|> ").count(), 2, "{output}");
    assert!(
        !output.contains("Unresolved or out-of-scope base"),
        "{output}"
    );
}

#[test]
fn generic_receivers_and_bases_resolve_to_the_declared_type() {
    let output = export_source(
        "generic.go",
        r#"
package sample
type Stack[T any] struct { value T }
func (s *Stack[T]) Push(value T) {}
func (s *Stack[T]) Pop() T { return s.value }
"#,
    );
    assert!(
        box_members(&output, "Stack").contains("Push(value: T)"),
        "{output}"
    );
    assert!(
        box_members(&output, "Stack").contains("Pop() T"),
        "{output}"
    );
    assert!(!output.contains("unresolved_impl"), "{output}");
    let output = export_source(
        "generic.py",
        r#"
class User: pass
class Base: pass
class Derived(Base[User]): pass
"#,
    );
    assert!(output.contains(" --|> "), "{output}");
    assert!(
        !output.contains("Unresolved or out-of-scope base"),
        "{output}"
    );
}

#[test]
fn type_references_do_not_cross_language_boundaries() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::write(
        repo.path().join("client.ts"),
        "class Client { alias: Response; }",
    )
    .unwrap();
    std::fs::write(repo.path().join("server.py"), "class Response: pass").unwrap();
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    let output = export_mermaid(&result, &full_options()).unwrap();
    assert!(!output.contains(" --> "), "{output}");
    let output = export_source(
        "client.ts",
        "class Response {} class Client { alias: Response; }",
    );
    assert!(output.contains("field alias"), "{output}");
}

#[test]
fn cpp_definitions_do_not_repeat_declared_methods_or_lose_calls() {
    let output = export_source(
        "service.cpp",
        r#"
struct Service { int run(); int value(); };
int Service::run() { return value(); }
int Service::value() { return 1; }
"#,
    );
    assert_eq!(
        box_members(&output, "Service").matches("run() int").count(),
        1,
        "{output}"
    );
    assert_eq!(
        box_members(&output, "Service")
            .matches("value() int")
            .count(),
        1,
        "{output}"
    );
    assert!(output.contains("run() calls value()"), "{output}");
}

#[test]
fn enum_variants_are_members_of_their_enum_in_each_nominal_language() {
    for (file, source) in [
        ("state.cs", "enum State { Ready, Done = 2 }"),
        ("state.java", "enum State { Ready, Done }"),
        ("state.ts", "enum State { Ready, Done = 2 }"),
        (
            "state.vb",
            "Public Enum State\n Ready\n Done = 2\nEnd Enum\n",
        ),
        ("state.c", "enum State { Ready, Done = 2 };"),
        ("state.cpp", "enum class State { Ready, Done = 2 };"),
    ] {
        let output = export_source(file, source);
        let members = box_members(&output, "State");
        assert!(members.contains("Ready:"), "{file}: {output}");
        assert!(members.contains("Done:"), "{file}: {output}");
    }
}

#[test]
fn positional_records_preserve_their_declared_fields() {
    for (file, source, expected) in [
        (
            "point.cs",
            "public record Point(int X, string Name);",
            ["X: int", "Name: string"],
        ),
        (
            "point.java",
            "public record Point(int X, String Name) {}",
            ["X: int", "Name: String"],
        ),
    ] {
        let output = export_source(file, source);
        for member in expected {
            assert!(
                box_members(&output, "Point").contains(member),
                "{file}: {output}"
            );
        }
    }
}

#[test]
fn go_embedded_types_and_rust_tuple_payloads_are_retained() {
    let output = export_source(
        "embedded.go",
        r#"
package sample
type Base struct { ID int }
type Service struct { *Base }
type Reader interface { Read() int }
type ReadCloser interface { Reader; Close() }
"#,
    );
    assert!(
        box_members(&output, "Service").contains("Base: Type"),
        "{output}"
    );
    assert!(output.contains("field Base"), "{output}");
    assert!(output.contains(" --|> "), "{output}");
    let output = export_source(
        "tuple.rs",
        r#"
pub struct User { pub id: u64 }
pub struct Wrapper(pub User, u32);
pub enum Message { Ready, Item(User), Named { value: User } }
"#,
    );
    assert!(
        box_members(&output, "Wrapper").contains("+0: User"),
        "{output}"
    );
    assert!(
        box_members(&output, "Wrapper").contains("-1: u32"),
        "{output}"
    );
    assert!(output.contains("field Item"), "{output}");
    assert!(output.contains("field Named"), "{output}");
}

#[test]
fn common_generics_read_inline_and_complex_type_keys_preserve_source_spelling() {
    let output = export_source(
        "readable.rs",
        r#"
pub struct User;
pub struct Store {
    pub users: Option<Vec<User>>,
    pub lookup: std::collections::HashMap<String, User>,
}
"#,
    );
    assert!(
        box_members(&output, "Store").contains("users: Option~Vec~User~~"),
        "{output}"
    );
    assert!(
        output.contains("`std::collections::HashMap<String, User>`"),
        "{output}"
    );
    assert!(!output.contains("&#"), "{output}");
}

#[test]
fn scoped_exports_normalize_relative_paths_and_report_no_matches() {
    let repo = tempfile::tempdir().unwrap();
    std::fs::create_dir(repo.path().join("src")).unwrap();
    std::fs::write(repo.path().join("src/model.py"), "class User: pass").unwrap();
    let result = run_pipeline(
        &AnalysisConfig {
            repo_path: repo.path().to_string_lossy().into_owned(),
            ..Default::default()
        },
        None,
    )
    .unwrap();
    for path in ["./src", "src\\", "src/model.py"] {
        let output = export_mermaid(
            &result,
            &MermaidOptions {
                detail: DetailMode::Full,
                path: path.into(),
                ..Default::default()
            },
        )
        .unwrap();
        assert!(output.contains("[\"User\"]"), "{output}");
    }
    let error = export_mermaid(
        &result,
        &MermaidOptions {
            detail: DetailMode::Full,
            path: "missing".into(),
            ..Default::default()
        },
    )
    .unwrap_err();
    assert!(error.to_string().contains("no declarations match"));
}

#[test]
fn embedded_go_pointers_keep_the_declared_type_and_comments_are_not_parameters() {
    let output = export_source(
        "embedded.go",
        r#"
package sample
type Base struct { ID int }
type Service struct { *Base }
"#,
    );
    assert!(
        box_members(&output, "Service").contains("Base: Type"),
        "{output}"
    );
    assert!(output.contains("`*Base`"), "{output}");
    let output = export_source(
        "comments.py",
        r#"
class Service:
    def run(self, value: str, # explanation
            count: int) -> str: pass
"#,
    );
    assert!(
        box_members(&output, "Service").contains("run(value: str, count: int) str"),
        "{output}"
    );
    assert!(!output.contains("explanation"), "{output}");
}

#[test]
fn python_forward_annotations_and_rust_lifetimes_preserve_type_relationships() {
    let output = export_source(
        "forward.py",
        r#"
from typing import Literal
class Repo:
    def owner(self) -> "User": pass
    items: list["Item"]
    literal: Literal["Label"]
class User: pass
class Item: pass
class Label: pass
"#,
    );
    assert!(output.contains("type in owner"), "{output}");
    assert!(output.contains("field items"), "{output}");
    assert!(!output.contains("field literal"), "{output}");
    let output = export_source(
        "lifetimes.rs",
        r#"
pub struct User;
pub struct Group;
pub fn pair<'a>() -> (&'a User, &'a Group) { unimplemented!() }
"#,
    );
    // Only the return type supplies these references; parameters cannot mask a regression.
    assert!(output.contains("c0002 ..> c0000"), "{output}");
    assert!(output.contains("c0002 ..> c0001"), "{output}");
}

#[test]
fn cpp_unqualified_method_calls_prefer_the_enclosing_class() {
    let output = export_source(
        "owners.cpp",
        r#"
struct A { void run(); };
struct B { void run(); void go(); };
void B::run() {}
void B::go() { run(); }
void A::run() {}
"#,
    );
    assert!(
        output.contains("c0001 ..> c0001 : go() calls run()"),
        "{output}"
    );
    assert!(!output.contains("c0001 ..> c0000"), "{output}");
}

#[test]
fn cpp_forward_declarations_do_not_hide_calls_from_methods() {
    let output = export_source(
        "forward.cpp",
        r#"
static int helper(int);
struct B { void go(); };
void B::go() { helper(1); }
static int helper(int x) { return x; }
"#,
    );
    assert!(output.contains("go() calls helper()"), "{output}");
}

#[test]
fn python_annotation_metadata_does_not_create_type_relationships() {
    let output = export_source(
        "metadata.py",
        r#"
class User: pass
class Repo:
    value: Annotated[str, "Owning User id"]
    literal: typing_extensions.Literal["User"]
"#,
    );
    assert!(!output.contains(" --> "), "{output}");
}

#[test]
fn java_basename_fallback_uses_a_stable_path_order_across_fresh_analyses() {
    let repo = tempfile::tempdir().unwrap();
    for folder in ["b", "a"] {
        std::fs::create_dir(repo.path().join(folder)).unwrap();
        std::fs::write(
            repo.path().join(folder).join("Helper.java"),
            "class Helper { public static void ping() {} }",
        )
        .unwrap();
    }
    std::fs::write(
        repo.path().join("Main.java"),
        "import sample.Helper; class Main { void run() { Helper.ping(); } }",
    )
    .unwrap();
    let config = AnalysisConfig {
        repo_path: repo.path().to_string_lossy().into_owned(),
        ..Default::default()
    };
    let mut previous = None;
    // Every run builds a new randomized HashSet. Its order must not choose the import target.
    for _ in 0..16 {
        let result = run_pipeline(&config, None).unwrap();
        let output = export_mermaid(&result, &full_options()).unwrap();
        assert!(
            output.contains("c0000 ..> c0001 : run() calls ping()"),
            "{output}"
        );
        if let Some(prior) = previous {
            assert_eq!(output, prior);
        }
        previous = Some(output);
    }
}
