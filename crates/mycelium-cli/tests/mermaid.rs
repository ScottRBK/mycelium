use std::process::Command;

#[test]
fn cli_exports_saved_map_and_reports_old_maps_without_creating_output() {
    // Arrange: analyse through the public CLI, then export its saved JSON.
    let temp = std::env::temp_dir().join(format!("mycelium-cli-{}", std::process::id()));
    std::fs::create_dir_all(&temp).unwrap();
    let source = temp.join("source");
    std::fs::create_dir_all(&source).unwrap();
    std::fs::write(source.join("user.py"), "class User:\n    name: str\n").unwrap();
    let map = temp.join("map.json");
    let output = temp.join("diagrams.md");
    let binary = env!("CARGO_BIN_EXE_mycelium-map");
    assert!(Command::new(binary)
        .args(["analyze", source.to_str().unwrap(), "--quiet", "-o"])
        .arg(&map)
        .status()
        .unwrap()
        .success());
    // Act.
    let exported = Command::new(binary)
        .arg("export")
        .arg(&map)
        .args(["--format", "mermaid", "-o"])
        .arg(&output)
        .output()
        .unwrap();
    // Assert.
    assert!(
        exported.status.success(),
        "{}",
        String::from_utf8_lossy(&exported.stderr)
    );
    assert!(std::fs::read_to_string(&output)
        .unwrap()
        .contains("+name: str"));
    std::fs::remove_file(&output).unwrap();
    std::fs::write(&map, "{}").unwrap();
    let rejected = Command::new(binary)
        .arg("export")
        .arg(&map)
        .arg("-o")
        .arg(&output)
        .output()
        .unwrap();
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("rerun analysis"));
    assert!(!output.exists());
    std::fs::remove_dir_all(temp).unwrap();
}

#[test]
fn cli_test_options_filter_and_report_validation_without_overwriting_output() {
    // Arrange.
    let temp = std::env::temp_dir().join(format!("mycelium-cli-tests-{}", std::process::id()));
    std::fs::create_dir_all(&temp).unwrap();
    std::fs::write(temp.join("lib.rs"), "struct App {}\n#[test] fn check() {}").unwrap();
    let map = temp.join("map.json");
    let output = temp.join("diagram.md");
    let binary = env!("CARGO_BIN_EXE_mycelium-map");
    assert!(Command::new(binary)
        .arg("analyze")
        .arg(&temp)
        .args(["--quiet", "-o"])
        .arg(&map)
        .status()
        .unwrap()
        .success());
    // Act / Assert.
    for (args, visible) in [
        (vec!["--tests", "exclude"], false),
        (vec!["--tests", "include"], true),
        (
            vec![
                "--test-path",
                ".",
                "--keep-path",
                "lib.rs",
                "--explain-tests",
            ],
            true,
        ),
    ] {
        let run = Command::new(binary)
            .arg("export")
            .arg(&map)
            .arg("-o")
            .arg(&output)
            .args(args)
            .output()
            .unwrap();
        assert!(
            run.status.success(),
            "{}",
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(
            std::fs::read_to_string(&output)
                .unwrap()
                .contains("check()"),
            visible
        );
    }
    let before = std::fs::read(&output).unwrap();
    let run = Command::new(binary)
        .arg("export")
        .arg(&map)
        .arg("-o")
        .arg(&output)
        .args(["--test-path", "missing"])
        .output()
        .unwrap();
    assert!(!run.status.success());
    assert!(String::from_utf8_lossy(&run.stderr).contains("matches no analysed file"));
    assert_eq!(before, std::fs::read(&output).unwrap());
    std::fs::remove_dir_all(temp).unwrap();
}
