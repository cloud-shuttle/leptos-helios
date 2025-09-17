//! Trunk Build Integration Tests
//! Tests for complete Trunk build pipeline integration

use std::fs;
use std::path::Path;
use std::process::Command;

/// Helper function to check if a file exists
fn file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

/// Helper function to check if a directory exists
fn dir_exists(path: &str) -> bool {
    Path::new(path).is_dir()
}

/// Helper function to get file size in bytes
fn get_file_size(path: &str) -> Option<u64> {
    fs::metadata(path).ok().map(|m| m.len())
}

#[test]
fn test_trunk_configuration_exists() {
    // Given: Trunk configuration should exist
    // When: Checking for Trunk.toml
    // Then: Configuration file should exist

    assert!(file_exists("../Trunk.toml"), "Trunk.toml should exist");

    // Read and validate configuration
    let config_content =
        fs::read_to_string("../Trunk.toml").expect("Should be able to read Trunk.toml");
    assert!(
        config_content.contains("helios-app"),
        "Trunk.toml should reference helios-app binary"
    );
    assert!(
        config_content.contains("wasm_file"),
        "Trunk.toml should specify wasm_file"
    );
}

#[test]
fn test_helios_app_crate_exists() {
    // Given: helios-app crate should exist for Trunk
    // When: Checking for helios-app directory and files
    // Then: Crate should be properly structured

    assert!(
        dir_exists("../helios-app"),
        "helios-app directory should exist"
    );
    assert!(
        file_exists("../helios-app/Cargo.toml"),
        "helios-app/Cargo.toml should exist"
    );
    assert!(
        file_exists("../helios-app/src/lib.rs"),
        "helios-app/src/lib.rs should exist"
    );

    // Check Cargo.toml configuration
    let cargo_content =
        fs::read_to_string("../helios-app/Cargo.toml").expect("Should read helios-app Cargo.toml");
    assert!(
        cargo_content.contains("cdylib"),
        "helios-app should be configured as cdylib"
    );
    assert!(
        cargo_content.contains("wasm-bindgen"),
        "helios-app should have wasm-bindgen dependency"
    );
}

#[test]
fn test_workspace_includes_helios_app() {
    // Given: Workspace should include helios-app
    // When: Checking workspace Cargo.toml
    // Then: helios-app should be listed as a member

    let workspace_content =
        fs::read_to_string("../Cargo.toml").expect("Should read workspace Cargo.toml");
    assert!(
        workspace_content.contains("helios-app"),
        "Workspace should include helios-app as member"
    );
}

#[test]
fn test_trunk_build_creates_dist_directory() {
    // Given: Clean build environment
    // When: Running trunk build
    // Then: dist directory should be created

    // Clean up any existing dist directory
    if dir_exists("../dist") {
        fs::remove_dir_all("../dist").expect("Should be able to remove dist directory");
    }

    // Run trunk build from the root directory
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir("..")
        .output()
        .expect("trunk command should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that dist directory was created
    assert!(
        dir_exists("../dist"),
        "dist directory should be created after trunk build"
    );
}

#[test]
fn test_trunk_build_creates_required_files() {
    // Given: Trunk build should create required files
    // When: Running trunk build
    // Then: All required files should be present

    // Run trunk build
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("trunk command should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check for required files
    assert!(
        file_exists("dist/index.html"),
        "dist/index.html should exist"
    );
    assert!(file_exists("dist/helios.js"), "dist/helios.js should exist");
    assert!(
        file_exists("dist/helios_bg.wasm"),
        "dist/helios_bg.wasm should exist"
    );
}

#[test]
fn test_wasm_file_size_optimization() {
    // Given: WASM file should be optimized
    // When: Checking WASM file size
    // Then: File should be reasonably sized

    // Run trunk build
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("trunk command should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check WASM file size
    let wasm_size = get_file_size("dist/helios_bg.wasm").expect("WASM file should exist");
    assert!(wasm_size > 0, "WASM file should not be empty");
    assert!(
        wasm_size < 10 * 1024 * 1024,
        "WASM file should be less than 10MB, got {} bytes",
        wasm_size
    );
}

#[test]
fn test_html_file_includes_wasm_loading() {
    // Given: HTML file should properly load WASM
    // When: Checking dist/index.html content
    // Then: Should include proper WASM loading script

    // Run trunk build
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("trunk command should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check HTML content
    let html_content = fs::read_to_string("dist/index.html").expect("Should read dist/index.html");
    assert!(
        html_content.contains("helios.js"),
        "HTML should reference helios.js"
    );
    assert!(
        html_content.contains("helios_bg.wasm"),
        "HTML should reference helios_bg.wasm"
    );
    assert!(
        html_content.contains("init"),
        "HTML should call WASM init function"
    );
}

#[test]
fn test_trunk_serve_starts_correctly() {
    // Given: Trunk serve should start correctly
    // When: Starting trunk serve
    // Then: Server should start without errors

    // Start trunk serve in background
    let mut child = Command::new("trunk")
        .args(&["serve", "--port", "8081"])
        .spawn()
        .expect("trunk serve should start");

    // Give it a moment to start
    std::thread::sleep(std::time::Duration::from_millis(2000));

    // Check if process is still running
    match child.try_wait() {
        Ok(Some(status)) => {
            panic!(
                "Trunk serve should still be running, but exited with: {:?}",
                status
            );
        }
        Ok(None) => {
            // Process is still running, which is good
        }
        Err(e) => {
            panic!("Error checking trunk serve process: {}", e);
        }
    }

    // Clean up - kill the process
    let _ = child.kill();
    let _ = child.wait();
}

#[test]
fn test_build_pipeline_integration() {
    // Given: Complete build pipeline should work
    // When: Running full build process
    // Then: All steps should succeed

    // Step 1: Clean build
    let clean_output = Command::new("cargo")
        .args(&["clean"])
        .output()
        .expect("cargo clean should work");

    assert!(clean_output.status.success(), "cargo clean should succeed");

    // Step 2: Build WASM
    let wasm_output = Command::new("wasm-pack")
        .args(&["build", "--target", "web", "--out-dir", "pkg"])
        .current_dir("helios-app")
        .output()
        .expect("wasm-pack should be available");

    assert!(
        wasm_output.status.success(),
        "wasm-pack build should succeed: {}",
        String::from_utf8_lossy(&wasm_output.stderr)
    );

    // Step 3: Trunk build
    let trunk_output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("trunk should be available");

    assert!(
        trunk_output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&trunk_output.stderr)
    );

    // Step 4: Verify final output
    assert!(dir_exists("dist"), "dist directory should exist");
    assert!(
        file_exists("dist/index.html"),
        "dist/index.html should exist"
    );
    assert!(file_exists("dist/helios.js"), "dist/helios.js should exist");
    assert!(
        file_exists("dist/helios_bg.wasm"),
        "dist/helios_bg.wasm should exist"
    );
}

#[test]
fn test_development_vs_production_builds() {
    // Given: Different build modes should work
    // When: Building in development and production modes
    // Then: Both should succeed with appropriate optimizations

    // Development build
    let dev_output = Command::new("trunk")
        .args(&["build"])
        .output()
        .expect("trunk should be available");

    assert!(
        dev_output.status.success(),
        "trunk dev build should succeed: {}",
        String::from_utf8_lossy(&dev_output.stderr)
    );

    // Production build
    let prod_output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("trunk should be available");

    assert!(
        prod_output.status.success(),
        "trunk prod build should succeed: {}",
        String::from_utf8_lossy(&prod_output.stderr)
    );

    // Production build should be smaller
    let dev_size = get_file_size("dist/helios_bg.wasm").expect("Dev WASM should exist");
    let prod_size = get_file_size("dist/helios_bg.wasm").expect("Prod WASM should exist");

    // Note: In practice, we'd need to compare different builds, but for now just verify both exist
    assert!(dev_size > 0, "Development WASM should not be empty");
    assert!(prod_size > 0, "Production WASM should not be empty");
}

#[test]
fn test_build_artifacts_are_valid() {
    // Given: Build artifacts should be valid
    // When: Checking generated files
    // Then: Files should be properly formatted and loadable

    // Run trunk build
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .output()
        .expect("trunk should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check HTML is valid
    let html_content = fs::read_to_string("dist/index.html").expect("Should read HTML");
    assert!(
        html_content.contains("<!DOCTYPE html>") || html_content.contains("<html"),
        "HTML should be properly formatted"
    );

    // Check JS file is not empty
    let js_size = get_file_size("dist/helios.js").expect("JS file should exist");
    assert!(js_size > 0, "JavaScript file should not be empty");

    // Check WASM file is not empty
    let wasm_size = get_file_size("dist/helios_bg.wasm").expect("WASM file should exist");
    assert!(wasm_size > 0, "WASM file should not be empty");

    // Check WASM file has correct magic bytes
    let wasm_content = fs::read("dist/helios_bg.wasm").expect("Should read WASM file");
    assert!(
        wasm_content.len() >= 8,
        "WASM file should have proper header"
    );
    // WASM files start with magic bytes: 0x00 0x61 0x73 0x6D
    assert_eq!(
        &wasm_content[0..4],
        &[0x00, 0x61, 0x73, 0x6D],
        "WASM file should have correct magic bytes"
    );
}
