//! .csproj/.vbproj XML parser.

use std::path::Path;

/// Parsed project file data.
#[derive(Debug, Clone, Default)]
pub struct ProjectFile {
    pub name: String,
    pub target_framework: Option<String>,
    pub root_namespace: Option<String>,
    pub assembly_name: Option<String>,
    pub project_references: Vec<String>,
    pub package_references: Vec<(String, String)>, // (name, version)
}

/// Parse a .csproj or .vbproj XML file content.
///
/// Handles both SDK-style and legacy project formats.
/// The `project_path` is used to derive default namespace/assembly name.
pub fn parse_project_file(content: &str, project_path: &str) -> ProjectFile {
    let project_name = Path::new(project_path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let mut info = ProjectFile {
        name: project_name.clone(),
        ..Default::default()
    };

    // Simple XML parsing using quick approach — avoid pulling in a full XML library.
    // We parse the key elements we need: PropertyGroup children, ProjectReference, PackageReference.

    // Extract RootNamespace
    if let Some(val) = extract_element_text(content, "RootNamespace") {
        info.root_namespace = Some(val);
    }

    // Extract AssemblyName
    if let Some(val) = extract_element_text(content, "AssemblyName") {
        info.assembly_name = Some(val);
    }

    // Extract TargetFramework
    if let Some(val) = extract_element_text(content, "TargetFramework") {
        info.target_framework = Some(val);
    } else if let Some(val) = extract_element_text(content, "TargetFrameworks") {
        // Use the first framework listed
        info.target_framework = Some(val.split(';').next().unwrap_or("").to_string());
    }

    // Extract ProjectReference Include attributes
    for include in extract_include_attrs(content, "ProjectReference") {
        info.project_references.push(include.replace('\\', "/"));
    }

    // Extract PackageReference Include + Version attributes
    for (name, version) in extract_package_refs(content) {
        info.package_references.push((name, version));
    }

    // Defaults: if no RootNamespace/AssemblyName, derive from file name
    if info.root_namespace.is_none() {
        info.root_namespace = Some(project_name.clone());
    }
    if info.assembly_name.is_none() {
        info.assembly_name = Some(project_name);
    }

    info
}

/// Extract text content of a simple XML element like `<Tag>value</Tag>`.
fn extract_element_text(content: &str, tag: &str) -> Option<String> {
    let open = format!("<{}>", tag);
    let close = format!("</{}>", tag);
    if let Some(start) = content.find(&open) {
        let after = start + open.len();
        if let Some(end) = content[after..].find(&close) {
            let text = content[after..after + end].trim().to_string();
            if !text.is_empty() {
                return Some(text);
            }
        }
    }
    // Also check self-closing with attributes (won't have text)
    None
}

/// Extract Include attribute values from elements like `<ProjectReference Include="..."/>`.
fn extract_include_attrs(content: &str, tag: &str) -> Vec<String> {
    let mut results = Vec::new();
    let pattern = format!("<{}", tag);
    let mut search_from = 0;

    while let Some(pos) = content[search_from..].find(&pattern) {
        let abs_pos = search_from + pos;
        let rest = &content[abs_pos..];

        // Find the end of this element (either /> or >)
        if let Some(end) = rest.find('>') {
            let element = &rest[..=end];
            if let Some(include) = extract_attr(element, "Include") {
                results.push(include);
            }
        }
        search_from = abs_pos + pattern.len();
    }
    results
}

/// Extract PackageReference entries with Include and Version attributes.
fn extract_package_refs(content: &str) -> Vec<(String, String)> {
    let mut results = Vec::new();
    let pattern = "<PackageReference";
    let mut search_from = 0;

    while let Some(pos) = content[search_from..].find(pattern) {
        let abs_pos = search_from + pos;
        let rest = &content[abs_pos..];

        // Find the end of this element — could be self-closing or have children
        let end_pos = if let Some(sc) = rest.find("/>") {
            if let Some(gt) = rest.find('>') {
                if sc < gt {
                    sc + 2
                } else {
                    gt + 1
                }
            } else {
                sc + 2
            }
        } else if let Some(gt) = rest.find('>') {
            gt + 1
        } else {
            search_from = abs_pos + pattern.len();
            continue;
        };

        let element = &rest[..end_pos];
        let name = extract_attr(element, "Include").unwrap_or_default();
        let mut version = extract_attr(element, "Version").unwrap_or_default();

        // If Version is not an attribute, check for child element
        if version.is_empty() {
            // Look for </PackageReference> closing tag
            if let Some(close_pos) = rest.find("</PackageReference>") {
                let inner = &rest[end_pos..close_pos];
                if let Some(v) = extract_element_text(inner, "Version") {
                    version = v;
                }
            }
        }

        if !name.is_empty() {
            results.push((name, version));
        }
        search_from = abs_pos + pattern.len();
    }
    results
}

/// Extract an attribute value from an XML element string.
#[allow(clippy::manual_strip)]
fn extract_attr(element: &str, attr: &str) -> Option<String> {
    let patterns = [format!("{}=\"", attr), format!("{}='", attr)];

    for pat in &patterns {
        if let Some(start) = element.find(pat.as_str()) {
            let after = start + pat.len();
            let quote = element.as_bytes()[after - 1] as char;
            if let Some(end) = element[after..].find(quote) {
                return Some(element[after..after + end].to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CSPROJ: &str = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
    <RootNamespace>Absence.Services</RootNamespace>
    <AssemblyName>Absence.Services</AssemblyName>
  </PropertyGroup>
  <ItemGroup>
    <ProjectReference Include="..\Absence.Core\Absence.Core.csproj" />
    <ProjectReference Include="..\Absence.Data\Absence.Data.csproj" />
  </ItemGroup>
  <ItemGroup>
    <PackageReference Include="Newtonsoft.Json" Version="13.0.1" />
    <PackageReference Include="Serilog" Version="3.1.1" />
  </ItemGroup>
</Project>"#;

    #[test]
    fn parse_root_namespace() {
        let info = parse_project_file(SAMPLE_CSPROJ, "Services/Services.csproj");
        assert_eq!(info.root_namespace.as_deref(), Some("Absence.Services"));
    }

    #[test]
    fn parse_project_references() {
        let info = parse_project_file(SAMPLE_CSPROJ, "Services/Services.csproj");
        assert_eq!(info.project_references.len(), 2);
        assert!(info.project_references[0].contains("Absence.Core"));
        assert!(info.project_references[1].contains("Absence.Data"));
    }

    #[test]
    fn parse_package_references() {
        let info = parse_project_file(SAMPLE_CSPROJ, "Services/Services.csproj");
        assert_eq!(info.package_references.len(), 2);
        assert_eq!(info.package_references[0].0, "Newtonsoft.Json");
        assert_eq!(info.package_references[0].1, "13.0.1");
    }

    #[test]
    fn default_namespace_from_filename() {
        let minimal = r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
</Project>"#;
        let info = parse_project_file(minimal, "MyProject/MyProject.csproj");
        // When no RootNamespace, defaults to project file stem
        assert_eq!(info.root_namespace.as_deref(), Some("MyProject"));
        assert_eq!(info.assembly_name.as_deref(), Some("MyProject"));
    }
}
