//! Browser Build Integration Tests
//! Tests for ensuring the build pipeline works correctly for browser deployment

use leptos_helios::*;
use std::fs;
use std::path::Path;
use std::process::Command;

#[test]
fn test_trunk_build_creates_dist_directory() {
    // Given: Clean build environment
    let dist_dir = Path::new("../dist");
    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir).unwrap();
    }

    // When: Running trunk build
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to execute trunk build");

    // Then: dist directory should be created
    assert!(
        dist_dir.exists(),
        "dist directory should be created after trunk build"
    );
    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn test_trunk_build_creates_required_files() {
    // Given: Clean build environment
    let dist_dir = Path::new("../dist");
    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir).unwrap();
    }

    // When: Running trunk build
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to execute trunk build");

    assert!(output.status.success(), "trunk build should succeed");

    // Then: Required files should exist
    let required_files = vec![
        "../dist/index.html",
        "../dist/helios.js",
        "../dist/helios_bg.wasm",
    ];

    for file in required_files {
        assert!(
            Path::new(file).exists(),
            "Required file {} should exist",
            file
        );
    }
}

#[test]
fn test_wasm_package_has_correct_structure() {
    // Given: WASM package should be built
    let pkg_dir = Path::new("pkg");

    // When: Checking package structure
    // Then: Required WASM files should exist
    let required_files = vec!["../pkg/helios.js", "../pkg/helios_bg.wasm"];

    for file in required_files {
        assert!(Path::new(file).exists(), "WASM file {} should exist", file);
    }
}

#[test]
fn test_wasm_file_size_is_reasonable() {
    // Given: WASM file exists
    let wasm_path = Path::new("../pkg/helios_bg.wasm");

    // When: Checking file size
    if wasm_path.exists() {
        let metadata = fs::metadata(wasm_path).unwrap();
        let size_mb = metadata.len() as f64 / (1024.0 * 1024.0);

        // Then: File size should be reasonable (< 5MB for now)
        assert!(
            size_mb < 5.0,
            "WASM file size {}MB should be reasonable",
            size_mb
        );
    }
}

#[test]
fn test_index_html_has_correct_structure() {
    // Given: index.html should exist
    let index_path = Path::new("../index.html");

    // When: Reading index.html content
    if index_path.exists() {
        let content = fs::read_to_string(index_path).unwrap();

        // Then: Should contain required elements
        assert!(
            content.contains("<html"),
            "index.html should contain html tag"
        );
        assert!(
            content.contains("<script"),
            "index.html should contain script tag"
        );
        assert!(
            content.contains("helios.js"),
            "index.html should reference helios.js"
        );
    }
}

#[test]
fn test_trunk_serve_starts_correctly() {
    // Given: Trunk should be able to start dev server
    // When: Starting trunk serve (with timeout)
    let mut child = Command::new("trunk")
        .args(&["serve", "--port", "8081", "--no-open"])
        .spawn()
        .expect("Failed to start trunk serve");

    // Give it a moment to start
    std::thread::sleep(std::time::Duration::from_millis(2000));

    // Then: Process should still be running
    match child.try_wait() {
        Ok(Some(status)) => panic!(
            "Trunk serve should still be running, but exited with: {:?}",
            status
        ),
        Ok(None) => {
            // Process is still running, which is good
            child.kill().unwrap();
        }
        Err(e) => panic!("Error checking process status: {}", e),
    }
}

#[test]
fn test_build_pipeline_integration() {
    // Given: Clean environment
    let dist_dir = Path::new("../dist");
    if dist_dir.exists() {
        fs::remove_dir_all(dist_dir).unwrap();
    }

    // When: Running full build pipeline
    let wasm_output = Command::new("wasm-pack")
        .args(&[
            "build",
            "--target",
            "web",
            "--out-dir",
            "../pkg",
            "--out-name",
            "helios",
            "--release",
            "--no-typescript",
            "--no-pack",
        ])
        .current_dir("helios-wasm-core")
        .output()
        .expect("Failed to execute wasm-pack build");

    assert!(
        wasm_output.status.success(),
        "wasm-pack build should succeed"
    );

    let trunk_output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("Failed to execute trunk build");

    // Then: Both builds should succeed and create required files
    assert!(
        trunk_output.status.success(),
        "trunk build should succeed after wasm-pack"
    );
    assert!(
        dist_dir.exists(),
        "dist directory should exist after full pipeline"
    );
}
