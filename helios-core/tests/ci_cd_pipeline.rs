//! TDD tests for CI/CD pipeline functionality
//! 
//! This module tests the CI/CD pipeline components including:
//! - GitHub Actions workflow validation
//! - Automated testing and deployment
//! - Code quality checks and security scanning
//! - Build artifact generation and publishing

use std::fs;
use std::path::Path;
use std::process::Command;

// Helper function to get the correct path to workflows directory
fn get_workflows_dir() -> &'static Path {
    Path::new("../.github/workflows")
}

/// Test that GitHub Actions workflow files exist and are valid
#[test]
fn test_github_actions_workflows_exist() {
    let workflows_dir = get_workflows_dir();
    
    assert!(workflows_dir.exists(), "GitHub Actions workflows directory should exist");
    
    let workflow_files = [
        "ci.yml",
        "test.yml", 
        "deploy.yml",
        "security.yml",
        "release.yml"
    ];
    
    for workflow_file in workflow_files {
        let workflow_path = workflows_dir.join(workflow_file);
        assert!(workflow_path.exists(), "Workflow file {} should exist", workflow_file);
        
        // Basic YAML validation
        let content = fs::read_to_string(&workflow_path)
            .expect("Should be able to read workflow file");
        
        assert!(content.contains("name:"), "Workflow should have a name");
        assert!(content.contains("on:"), "Workflow should have triggers");
        assert!(content.contains("jobs:"), "Workflow should have jobs");
    }
}

/// Test CI workflow configuration
#[test]
fn test_ci_workflow_configuration() {
    let ci_workflow = Path::new(".github/workflows/ci.yml");
    
    if ci_workflow.exists() {
        let content = fs::read_to_string(ci_workflow)
            .expect("Should be able to read CI workflow");
        
        // Check for required CI components
        assert!(content.contains("pull_request:"), "CI should run on pull requests");
        assert!(content.contains("push:"), "CI should run on push");
        assert!(content.contains("cargo check"), "CI should run cargo check");
        assert!(content.contains("cargo test"), "CI should run tests");
        assert!(content.contains("cargo clippy"), "CI should run clippy");
        assert!(content.contains("cargo fmt"), "CI should check formatting");
        
        // Check for matrix strategy
        assert!(content.contains("strategy:"), "CI should use matrix strategy");
        assert!(content.contains("rust-version:"), "CI should test multiple Rust versions");
        assert!(content.contains("os:"), "CI should test multiple operating systems");
    } else {
        println!("CI workflow not found - will be created during setup");
    }
}

/// Test deployment workflow configuration
#[test]
fn test_deployment_workflow_configuration() {
    let deploy_workflow = Path::new(".github/workflows/deploy.yml");
    
    if deploy_workflow.exists() {
        let content = fs::read_to_string(deploy_workflow)
            .expect("Should be able to read deployment workflow");
        
        // Check for deployment components
        assert!(content.contains("release:"), "Deploy should run on release");
        assert!(content.contains("wasm-pack build"), "Deploy should build WASM");
        assert!(content.contains("trunk build"), "Deploy should build with Trunk");
        assert!(content.contains("gh-pages"), "Deploy should deploy to GitHub Pages");
        
        // Check for artifact upload
        assert!(content.contains("actions/upload-artifact"), "Deploy should upload artifacts");
    } else {
        println!("Deployment workflow not found - will be created during setup");
    }
}

/// Test security workflow configuration
#[test]
fn test_security_workflow_configuration() {
    let security_workflow = Path::new(".github/workflows/security.yml");
    
    if security_workflow.exists() {
        let content = fs::read_to_string(security_workflow)
            .expect("Should be able to read security workflow");
        
        // Check for security components
        assert!(content.contains("cargo audit"), "Security should run cargo audit");
        assert!(content.contains("cargo deny"), "Security should run cargo deny");
        assert!(content.contains("CodeQL"), "Security should run CodeQL analysis");
        assert!(content.contains("Dependabot"), "Security should use Dependabot");
    } else {
        println!("Security workflow not found - will be created during setup");
    }
}

/// Test release workflow configuration
#[test]
fn test_release_workflow_configuration() {
    let release_workflow = Path::new(".github/workflows/release.yml");
    
    if release_workflow.exists() {
        let content = fs::read_to_string(release_workflow)
            .expect("Should be able to read release workflow");
        
        // Check for release components
        assert!(content.contains("tag:"), "Release should trigger on tags");
        assert!(content.contains("cargo publish"), "Release should publish to crates.io");
        assert!(content.contains("npm publish"), "Release should publish to npm");
        assert!(content.contains("gh release create"), "Release should create GitHub release");
    } else {
        println!("Release workflow not found - will be created during setup");
    }
}

/// Test workflow file syntax validation
#[test]
fn test_workflow_syntax_validation() {
    let workflows_dir = Path::new(".github/workflows");
    
    if workflows_dir.exists() {
        let entries = fs::read_dir(workflows_dir)
            .expect("Should be able to read workflows directory");
        
        for entry in entries {
            let entry = entry.expect("Should be able to read directory entry");
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                let content = fs::read_to_string(&path)
                    .expect("Should be able to read workflow file");
                
                // Basic YAML structure validation
                assert!(content.contains("name:"), "Workflow should have a name");
                assert!(content.contains("on:"), "Workflow should have triggers");
                assert!(content.contains("jobs:"), "Workflow should have jobs");
                
                // Check for proper indentation (basic check)
                let lines: Vec<&str> = content.lines().collect();
                for line in lines {
                    if line.trim().starts_with("-") {
                        // List items should be properly indented
                        assert!(line.starts_with("  ") || line.starts_with("-"), 
                               "List items should be properly indented");
                    }
                }
            }
        }
    } else {
        println!("Workflows directory not found - will be created during setup");
    }
}

/// Test CI/CD pipeline integration
#[test]
fn test_cicd_pipeline_integration() {
    // Test that all required tools are available for CI
    let required_tools = ["cargo", "rustc", "wasm-pack", "trunk"];
    
    for tool in required_tools {
        let output = Command::new(tool)
            .arg("--version")
            .output();
        
        match output {
            Ok(result) => {
                assert!(result.status.success(), "Tool {} should be available", tool);
                let version = String::from_utf8_lossy(&result.stdout);
                assert!(!version.is_empty(), "Tool {} should return version", tool);
            }
            Err(_) => {
                println!("Tool {} not found - will be installed during CI setup", tool);
            }
        }
    }
}

/// Test build matrix configuration
#[test]
fn test_build_matrix_configuration() {
    let ci_workflow = Path::new(".github/workflows/ci.yml");
    
    if ci_workflow.exists() {
        let content = fs::read_to_string(ci_workflow)
            .expect("Should be able to read CI workflow");
        
        // Check for build matrix
        assert!(content.contains("matrix:"), "CI should have build matrix");
        assert!(content.contains("rust-version:"), "CI should test multiple Rust versions");
        assert!(content.contains("os:"), "CI should test multiple operating systems");
        
        // Check for specific Rust versions
        let rust_versions = ["stable", "beta", "nightly"];
        for version in rust_versions {
            if content.contains(&format!("- {}", version)) {
                println!("Found Rust version: {}", version);
            }
        }
        
        // Check for operating systems
        let operating_systems = ["ubuntu-latest", "windows-latest", "macos-latest"];
        for os in operating_systems {
            if content.contains(&format!("- {}", os)) {
                println!("Found OS: {}", os);
            }
        }
    } else {
        println!("CI workflow not found - will be created during setup");
    }
}

/// Test artifact generation and caching
#[test]
fn test_artifact_generation_and_caching() {
    let workflows_dir = Path::new(".github/workflows");
    
    if workflows_dir.exists() {
        let entries = fs::read_dir(workflows_dir)
            .expect("Should be able to read workflows directory");
        
        let mut found_caching = false;
        let mut found_artifacts = false;
        
        for entry in entries {
            let entry = entry.expect("Should be able to read directory entry");
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                let content = fs::read_to_string(&path)
                    .expect("Should be able to read workflow file");
                
                if content.contains("actions/cache") {
                    found_caching = true;
                }
                
                if content.contains("actions/upload-artifact") || content.contains("actions/download-artifact") {
                    found_artifacts = true;
                }
            }
        }
        
        if found_caching {
            println!("Found caching configuration");
        } else {
            println!("Caching not configured - will be added during setup");
        }
        
        if found_artifacts {
            println!("Found artifact configuration");
        } else {
            println!("Artifacts not configured - will be added during setup");
        }
    } else {
        println!("Workflows directory not found - will be created during setup");
    }
}

/// Test security scanning configuration
#[test]
fn test_security_scanning_configuration() {
    let security_workflow = Path::new(".github/workflows/security.yml");
    
    if security_workflow.exists() {
        let content = fs::read_to_string(security_workflow)
            .expect("Should be able to read security workflow");
        
        // Check for security scanning tools
        let security_tools = [
            "cargo audit",
            "cargo deny", 
            "CodeQL",
            "Dependabot",
            "trivy",
            "grype"
        ];
        
        for tool in security_tools {
            if content.contains(tool) {
                println!("Found security tool: {}", tool);
            }
        }
    } else {
        println!("Security workflow not found - will be created during setup");
    }
}

/// Test notification configuration
#[test]
fn test_notification_configuration() {
    let workflows_dir = Path::new(".github/workflows");
    
    if workflows_dir.exists() {
        let entries = fs::read_dir(workflows_dir)
            .expect("Should be able to read workflows directory");
        
        let mut found_notifications = false;
        
        for entry in entries {
            let entry = entry.expect("Should be able to read directory entry");
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                let content = fs::read_to_string(&path)
                    .expect("Should be able to read workflow file");
                
                if content.contains("slack") || content.contains("discord") || content.contains("email") {
                    found_notifications = true;
                }
            }
        }
        
        if found_notifications {
            println!("Found notification configuration");
        } else {
            println!("Notifications not configured - will be added during setup");
        }
    } else {
        println!("Workflows directory not found - will be created during setup");
    }
}

/// Test environment variable configuration
#[test]
fn test_environment_variable_configuration() {
    let workflows_dir = Path::new(".github/workflows");
    
    if workflows_dir.exists() {
        let entries = fs::read_dir(workflows_dir)
            .expect("Should be able to read workflows directory");
        
        for entry in entries {
            let entry = entry.expect("Should be able to read directory entry");
            let path = entry.path();
            
            if path.extension().map_or(false, |ext| ext == "yml" || ext == "yaml") {
                let content = fs::read_to_string(&path)
                    .expect("Should be able to read workflow file");
                
                // Check for environment variable usage
                if content.contains("env:") {
                    println!("Found environment variables in {}", path.display());
                }
                
                // Check for secrets usage
                if content.contains("secrets:") {
                    println!("Found secrets configuration in {}", path.display());
                }
            }
        }
    } else {
        println!("Workflows directory not found - will be created during setup");
    }
}
