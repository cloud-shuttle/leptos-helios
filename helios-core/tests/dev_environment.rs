//! TDD tests for development environment setup
//!
//! This module tests the development environment components including:
//! - Pre-commit hooks configuration
//! - VS Code workspace settings
//! - Editor configurations
//! - Development scripts and tools
//! - Code quality tools integration

use std::fs;
use std::path::Path;
use std::process::Command;

/// Test that pre-commit hooks are configured
#[test]
fn test_pre_commit_hooks_exist() {
    let pre_commit_config = Path::new("../.pre-commit-config.yaml");
    let git_hooks_dir = Path::new("../.git/hooks");

    // Check if pre-commit config exists
    if pre_commit_config.exists() {
        let content = fs::read_to_string(pre_commit_config)
            .expect("Should be able to read pre-commit config");

        assert!(
            content.contains("repos:"),
            "Pre-commit config should have repos section"
        );
        assert!(
            content.contains("cargo fmt"),
            "Pre-commit should include cargo fmt"
        );
        assert!(
            content.contains("cargo clippy"),
            "Pre-commit should include cargo clippy"
        );
        assert!(
            content.contains("cargo test"),
            "Pre-commit should include cargo test"
        );
    } else {
        println!("Pre-commit config not found - will be created during setup");
    }

    // Check if git hooks directory exists
    assert!(git_hooks_dir.exists(), "Git hooks directory should exist");
}

/// Test VS Code workspace configuration
#[test]
fn test_vscode_workspace_config() {
    let vscode_dir = Path::new("../.vscode");
    let settings_json = vscode_dir.join("settings.json");
    let extensions_json = vscode_dir.join("extensions.json");
    let tasks_json = vscode_dir.join("tasks.json");
    let launch_json = vscode_dir.join("launch.json");

    if vscode_dir.exists() {
        // Check settings.json
        if settings_json.exists() {
            let content = fs::read_to_string(&settings_json)
                .expect("Should be able to read VS Code settings");

            assert!(
                content.contains("rust-analyzer"),
                "Settings should include rust-analyzer config"
            );
            assert!(
                content.contains("cargo"),
                "Settings should include cargo config"
            );
        }

        // Check extensions.json
        if extensions_json.exists() {
            let content = fs::read_to_string(&extensions_json)
                .expect("Should be able to read VS Code extensions");

            assert!(
                content.contains("rust-lang.rust-analyzer"),
                "Should recommend rust-analyzer"
            );
            assert!(
                content.contains("vadimcn.vscode-lldb"),
                "Should recommend LLDB debugger"
            );
        }

        // Check tasks.json
        if tasks_json.exists() {
            let content =
                fs::read_to_string(&tasks_json).expect("Should be able to read VS Code tasks");

            assert!(
                content.contains("cargo build"),
                "Should include cargo build task"
            );
            assert!(
                content.contains("cargo test"),
                "Should include cargo test task"
            );
        }

        // Check launch.json
        if launch_json.exists() {
            let content = fs::read_to_string(&launch_json)
                .expect("Should be able to read VS Code launch config");

            assert!(
                content.contains("cargo"),
                "Should include cargo launch config"
            );
        }
    } else {
        println!("VS Code directory not found - will be created during setup");
    }
}

/// Test editor configurations
#[test]
fn test_editor_configurations() {
    let editor_configs = [
        ("../.editorconfig", "EditorConfig"),
        ("../.rustfmt.toml", "Rustfmt"),
        ("../.clippy.toml", "Clippy"),
        ("../.gitignore", "Git ignore"),
    ];

    for (config_path, config_name) in editor_configs {
        let path = Path::new(config_path);

        if path.exists() {
            let content = fs::read_to_string(path)
                .expect(&format!("Should be able to read {} config", config_name));

            match config_name {
                "EditorConfig" => {
                    assert!(
                        content.contains("root = true"),
                        "EditorConfig should have root = true"
                    );
                    assert!(
                        content.contains("[*.rs]"),
                        "EditorConfig should have Rust section"
                    );
                }
                "Rustfmt" => {
                    assert!(
                        content.contains("edition = "),
                        "Rustfmt should specify edition"
                    );
                }
                "Clippy" => {
                    assert!(content.contains("deny"), "Clippy should have deny rules");
                }
                "Git ignore" => {
                    assert!(
                        content.contains("target/"),
                        "Gitignore should ignore target directory"
                    );
                    assert!(
                        content.contains("*.wasm"),
                        "Gitignore should ignore WASM files"
                    );
                }
                _ => {}
            }
        } else {
            println!(
                "{} config not found - will be created during setup",
                config_name
            );
        }
    }
}

/// Test development scripts
#[test]
fn test_development_scripts() {
    let scripts = [
        ("../scripts/dev-setup.sh", "Development setup script"),
        ("../scripts/install-tools.sh", "Tool installation script"),
        ("../scripts/format-code.sh", "Code formatting script"),
        ("../scripts/run-tests.sh", "Test runner script"),
    ];

    for (script_path, script_name) in scripts {
        let path = Path::new(script_path);

        if path.exists() {
            // Check if script is executable
            let metadata = fs::metadata(path)
                .expect(&format!("Should be able to read {} metadata", script_name));

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let permissions = metadata.permissions();
                assert!(
                    permissions.mode() & 0o111 != 0,
                    "{} should be executable",
                    script_name
                );
            }

            // Check script content
            let content = fs::read_to_string(path)
                .expect(&format!("Should be able to read {} content", script_name));

            assert!(!content.is_empty(), "{} should not be empty", script_name);
        } else {
            println!("{} not found - will be created during setup", script_name);
        }
    }
}

/// Test code quality tools integration
#[test]
fn test_code_quality_tools() {
    let tools = [
        ("cargo", "Cargo package manager"),
        ("rustfmt", "Rust formatter"),
        ("clippy", "Rust linter"),
        ("cargo-audit", "Security auditor"),
        ("cargo-deny", "License checker"),
    ];

    for (tool, tool_name) in tools {
        let output = Command::new(tool).arg("--version").output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    let version = String::from_utf8_lossy(&result.stdout);
                    assert!(!version.is_empty(), "{} should return version", tool_name);
                    println!("Found {}: {}", tool_name, version.trim());
                } else {
                    println!(
                        "{} not available - will be installed during setup",
                        tool_name
                    );
                }
            }
            Err(_) => {
                println!("{} not found - will be installed during setup", tool_name);
            }
        }
    }
}

/// Test development environment validation
#[test]
fn test_dev_environment_validation() {
    // Check if we're in a git repository
    let git_dir = Path::new("../.git");
    assert!(git_dir.exists(), "Should be in a git repository");

    // Check if Cargo.toml exists
    let cargo_toml = Path::new("../Cargo.toml");
    assert!(
        cargo_toml.exists(),
        "Should have Cargo.toml in project root"
    );

    // Check if we have a workspace structure
    let workspace_members = [
        "../helios-core",
        "../helios-leptos",
        "../helios-wasm",
        "../helios-macros",
        "../helios-examples",
        "../helios-benchmarks",
    ];

    for member in workspace_members {
        let path = Path::new(member);
        if path.exists() {
            let cargo_toml = path.join("Cargo.toml");
            assert!(
                cargo_toml.exists(),
                "Workspace member {} should have Cargo.toml",
                member
            );
        }
    }
}

/// Test pre-commit hooks functionality
#[test]
fn test_pre_commit_hooks_functionality() {
    let pre_commit_config = Path::new("../.pre-commit-config.yaml");

    if pre_commit_config.exists() {
        let content = fs::read_to_string(pre_commit_config)
            .expect("Should be able to read pre-commit config");

        // Check for required hooks
        let required_hooks = ["cargo fmt", "cargo clippy", "cargo test", "cargo check"];

        for hook in required_hooks {
            assert!(content.contains(hook), "Pre-commit should include {}", hook);
        }

        // Check for proper YAML structure
        assert!(content.contains("repos:"), "Should have repos section");
        assert!(content.contains("hooks:"), "Should have hooks section");
    } else {
        println!("Pre-commit config not found - will be created during setup");
    }
}

/// Test VS Code integration
#[test]
fn test_vscode_integration() {
    let vscode_dir = Path::new("../.vscode");

    if vscode_dir.exists() {
        let settings_json = vscode_dir.join("settings.json");

        if settings_json.exists() {
            let content = fs::read_to_string(&settings_json)
                .expect("Should be able to read VS Code settings");

            // Check for Rust-specific settings
            let rust_settings = ["rust-analyzer", "cargo", "rustfmt", "clippy"];

            for setting in rust_settings {
                if content.contains(setting) {
                    println!("Found VS Code setting: {}", setting);
                }
            }
        }
    } else {
        println!("VS Code directory not found - will be created during setup");
    }
}

/// Test development workflow scripts
#[test]
fn test_development_workflow_scripts() {
    let workflow_scripts = [
        ("../scripts/setup-dev.sh", "Development environment setup"),
        ("../scripts/run-all-tests.sh", "Run all tests"),
        ("../scripts/build-all.sh", "Build all packages"),
        ("../scripts/clean-all.sh", "Clean all build artifacts"),
    ];

    for (script_path, script_description) in workflow_scripts {
        let path = Path::new(script_path);

        if path.exists() {
            let content = fs::read_to_string(path)
                .expect(&format!("Should be able to read {}", script_description));

            // Basic script validation
            assert!(
                !content.is_empty(),
                "{} should not be empty",
                script_description
            );

            // Check for shebang
            if content.starts_with("#!") {
                println!("Found shebang in {}", script_description);
            }
        } else {
            println!(
                "{} not found - will be created during setup",
                script_description
            );
        }
    }
}

/// Test development environment completeness
#[test]
fn test_dev_environment_completeness() {
    let required_files = [
        ("../Cargo.toml", "Workspace Cargo.toml"),
        ("../Makefile", "Makefile for development commands"),
        ("../build.sh", "Build script"),
        ("../Trunk.toml", "Trunk configuration"),
        ("../.gitignore", "Git ignore file"),
    ];

    let mut found_files = 0;

    for (file_path, file_description) in required_files {
        let path = Path::new(file_path);

        if path.exists() {
            found_files += 1;
            println!("Found {}", file_description);
        } else {
            println!("Missing {}", file_description);
        }
    }

    // At least 80% of required files should exist
    let required_count = required_files.len();
    let found_percentage = (found_files as f64 / required_count as f64) * 100.0;

    assert!(
        found_percentage >= 80.0,
        "Should have at least 80% of required files (found {}/{} = {:.1}%)",
        found_files,
        required_count,
        found_percentage
    );
}

/// Test development tools availability
#[test]
fn test_development_tools_availability() {
    let tools = [
        ("git", "Git version control"),
        ("cargo", "Cargo package manager"),
        ("rustc", "Rust compiler"),
        ("make", "Make build tool"),
    ];

    let mut available_tools = 0;

    for (tool, tool_description) in tools {
        let output = Command::new(tool).arg("--version").output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    available_tools += 1;
                    let version = String::from_utf8_lossy(&result.stdout);
                    println!("Available: {} - {}", tool_description, version.trim());
                }
            }
            Err(_) => {
                println!("Not available: {}", tool_description);
            }
        }
    }

    // All essential tools should be available
    assert_eq!(
        available_tools,
        tools.len(),
        "All essential development tools should be available"
    );
}
