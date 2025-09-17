//! Trunk Configuration Fix Tests
//! TDD approach to fix Trunk HTML file path issues

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

#[test]
fn test_trunk_finds_html_file_in_correct_location() {
    // Given: Trunk should find the HTML file in the correct location
    // When: Running trunk build from helios-app directory
    // Then: Trunk should not complain about missing HTML file

    // Ensure we have the HTML file in helios-app directory
    assert!(
        file_exists("../helios-app/index.html"),
        "helios-app/index.html should exist"
    );

    // Run trunk build from helios-app directory
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir("../helios-app")
        .output()
        .expect("trunk command should be available");

    // Trunk should not fail with "error getting the canonical path to the build target HTML file"
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("error getting the canonical path to the build target HTML file"),
        "Trunk should find HTML file, but got error: {}",
        stderr
    );

    // Build should succeed
    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        stderr
    );
}

#[test]
fn test_trunk_creates_dist_directory_in_helios_app() {
    // Given: Trunk build should create dist directory in helios-app
    // When: Running trunk build
    // Then: dist directory should be created in the correct location

    // Clean up any existing dist directory
    if dir_exists("../helios-app/dist") {
        fs::remove_dir_all("../helios-app/dist").expect("Should be able to remove dist directory");
    }

    // Run trunk build from helios-app directory
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir("../helios-app")
        .output()
        .expect("trunk command should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check that dist directory was created in helios-app
    assert!(
        dir_exists("../helios-app/dist"),
        "dist directory should be created in helios-app"
    );
}

#[test]
fn test_trunk_creates_required_files_in_dist() {
    // Given: Trunk build should create required files
    // When: Running trunk build
    // Then: All required files should be present in dist

    // Run trunk build from helios-app directory
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir("../helios-app")
        .output()
        .expect("trunk command should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check for required files in helios-app/dist
    assert!(
        file_exists("../helios-app/dist/index.html"),
        "dist/index.html should exist"
    );

    // Trunk creates files with hash-based names, so we check for the pattern
    let dist_dir = std::fs::read_dir("../helios-app/dist").expect("Should read dist directory");
    let mut has_js_file = false;
    let mut has_wasm_file = false;

    for entry in dist_dir {
        let entry = entry.expect("Should read directory entry");
        let file_name = entry
            .file_name()
            .into_string()
            .expect("Should convert to string");
        if file_name.ends_with(".js") && file_name.contains("helios-app") {
            has_js_file = true;
        }
        if file_name.ends_with(".wasm") && file_name.contains("helios-app") {
            has_wasm_file = true;
        }
    }

    assert!(has_js_file, "dist should contain a helios-app JS file");
    assert!(has_wasm_file, "dist should contain a helios-app WASM file");
}

#[test]
fn test_trunk_html_file_has_correct_content() {
    // Given: Trunk should process the HTML file correctly
    // When: Running trunk build
    // Then: The processed HTML should have correct content

    // Run trunk build from helios-app directory
    let output = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir("../helios-app")
        .output()
        .expect("trunk command should be available");

    assert!(
        output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Check HTML content
    let html_content =
        fs::read_to_string("../helios-app/dist/index.html").expect("Should read dist/index.html");
    assert!(
        html_content.contains("helios-app"),
        "HTML should reference helios-app files"
    );
    assert!(
        html_content.contains(".js"),
        "HTML should reference JS file"
    );
    assert!(
        html_content.contains(".wasm"),
        "HTML should reference WASM file"
    );
    assert!(
        html_content.contains("init"),
        "HTML should call WASM init function"
    );
}

#[test]
fn test_trunk_serve_works_from_helios_app() {
    // Given: Trunk serve should work from helios-app directory
    // When: Starting trunk serve
    // Then: Server should start without errors

    // Start trunk serve in background from helios-app directory
    let mut child = Command::new("trunk")
        .args(&["serve", "--port", "8082"])
        .current_dir("../helios-app")
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
fn test_trunk_configuration_is_correct() {
    // Given: Trunk configuration should be correct
    // When: Checking Trunk.toml in helios-app
    // Then: Configuration should be valid

    // Check that Trunk.toml exists in helios-app
    assert!(
        file_exists("../helios-app/Trunk.toml"),
        "helios-app/Trunk.toml should exist"
    );

    // Read and validate configuration
    let config_content =
        fs::read_to_string("../helios-app/Trunk.toml").expect("Should read helios-app/Trunk.toml");
    assert!(
        config_content.contains("wasm_opt = \"false\""),
        "Should disable wasm_opt"
    );
}

#[test]
fn test_end_to_end_trunk_build_pipeline() {
    // Given: Complete Trunk build pipeline should work
    // When: Running full build process
    // Then: All steps should succeed

    // Step 1: Clean build
    if dir_exists("../helios-app/dist") {
        fs::remove_dir_all("../helios-app/dist").expect("Should be able to remove dist directory");
    }

    // Step 2: Trunk build
    let trunk_output = Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir("../helios-app")
        .output()
        .expect("trunk should be available");

    assert!(
        trunk_output.status.success(),
        "trunk build should succeed: {}",
        String::from_utf8_lossy(&trunk_output.stderr)
    );

    // Step 3: Verify final output
    assert!(
        dir_exists("../helios-app/dist"),
        "dist directory should exist"
    );
    assert!(
        file_exists("../helios-app/dist/index.html"),
        "dist/index.html should exist"
    );

    // Check for Trunk-generated files with hash-based names
    let dist_dir = std::fs::read_dir("../helios-app/dist").expect("Should read dist directory");
    let mut wasm_size = 0;

    for entry in dist_dir {
        let entry = entry.expect("Should read directory entry");
        let file_name = entry
            .file_name()
            .into_string()
            .expect("Should convert to string");
        if file_name.ends_with(".wasm") && file_name.contains("helios-app") {
            wasm_size = entry.metadata().expect("Should read metadata").len();
        }
    }

    // Step 4: Verify file sizes are reasonable
    assert!(wasm_size > 0, "WASM file should not be empty");
    assert!(
        wasm_size < 10 * 1024 * 1024,
        "WASM file should be less than 10MB, got {} bytes",
        wasm_size
    );
}
