//! .sln text format parser.

use regex::Regex;
use std::sync::LazyLock;

/// A project entry from a .sln file.
#[derive(Debug, Clone)]
pub struct SlnProject {
    pub name: String,
    pub path: String,
    pub project_type_guid: String,
    pub project_guid: String,
}

/// Solution folder GUID — these are virtual organising projects, not real projects.
const SOLUTION_FOLDER_GUID: &str = "2150E333-8FDC-42A3-9474-1A3956D46DE8";

static PROJECT_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r#"(?m)^Project\("\{([^}]+)\}"\)\s*=\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*"\{([^}]+)\}""#,
    )
    .unwrap()
});

/// Parse a .sln file content and extract project entries.
///
/// Excludes solution folders (virtual projects for organising).
pub fn parse_solution(content: &str) -> Vec<SlnProject> {
    let mut projects = Vec::new();

    for cap in PROJECT_RE.captures_iter(content) {
        let type_guid = cap[1].to_uppercase();
        let name = cap[2].to_string();
        let path = cap[3].replace('\\', "/");
        let project_guid = cap[4].to_uppercase();

        // Skip solution folders
        if type_guid == SOLUTION_FOLDER_GUID {
            continue;
        }

        projects.push(SlnProject {
            name,
            path,
            project_type_guid: type_guid,
            project_guid,
        });
    }

    projects
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_SLN: &str = r#"
Microsoft Visual Studio Solution File, Format Version 12.00
# Visual Studio Version 17
Project("{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}") = "WebApp", "src\WebApp\WebApp.csproj", "{12345678-1234-1234-1234-123456789ABC}"
EndProject
Project("{FAE04EC0-301F-11D3-BF4B-00C04F79EFBC}") = "Core", "src\Core\Core.csproj", "{87654321-4321-4321-4321-CBA987654321}"
EndProject
Project("{2150E333-8FDC-42A3-9474-1A3956D46DE8}") = "Solution Items", "Solution Items", "{AAAA1111-BBBB-CCCC-DDDD-EEEE22223333}"
EndProject
"#;

    #[test]
    fn parse_projects() {
        let projects = parse_solution(SAMPLE_SLN);
        assert_eq!(projects.len(), 2);
        assert_eq!(projects[0].name, "WebApp");
        assert_eq!(projects[1].name, "Core");
    }

    #[test]
    fn skip_solution_folders() {
        let projects = parse_solution(SAMPLE_SLN);
        assert!(
            projects.iter().all(|p| p.name != "Solution Items"),
            "Solution folders should be skipped"
        );
    }

    #[test]
    fn normalize_backslashes() {
        let projects = parse_solution(SAMPLE_SLN);
        assert_eq!(projects[0].path, "src/WebApp/WebApp.csproj");
        assert!(!projects[0].path.contains('\\'));
    }

    #[test]
    fn empty_solution() {
        let projects = parse_solution("# empty file\n");
        assert!(projects.is_empty());
    }
}
