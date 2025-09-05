# Helios Makefile
# Development workflow automation

.PHONY: help build dev test clean install-tools wasm trunk lint format check docs serve

# Default target
.DEFAULT_GOAL := help

# Configuration
PROJECT_NAME := helios
WASM_TARGET := wasm32-unknown-unknown
RELEASE_DIR := dist
PKG_DIR := pkg
CARGO_FLAGS := --workspace
WASM_FLAGS := --target web --out-dir $(PKG_DIR) --out-name helios --release --no-typescript --no-pack

# Colors
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[1;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Help target
help: ## Show this help message
	@echo "$(BLUE)Helios Visualization Library - Development Commands$(NC)"
	@echo ""
	@echo "$(GREEN)Build Commands:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST) | grep -E "(build|dev|wasm|trunk|clean)"
	@echo ""
	@echo "$(GREEN)Development Commands:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST) | grep -E "(serve|test|lint|format|check)"
	@echo ""
	@echo "$(GREEN)Setup Commands:$(NC)"
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  $(YELLOW)%-15s$(NC) %s\n", $$1, $$2}' $(MAKEFILE_LIST) | grep -E "(install|docs)"

# Build targets
build: clean install-tools ## Full build process (clean, install tools, build WASM, optimize, build with Trunk)
	@echo "$(BLUE)[INFO]$(NC) Starting full build process..."
	@chmod +x build.sh
	@./build.sh build

dev: ## Start development server with hot reload
	@echo "$(BLUE)[INFO]$(NC) Starting development server..."
	@chmod +x build.sh
	@./build.sh dev

wasm: ## Build and optimize WASM only
	@echo "$(BLUE)[INFO]$(NC) Building WASM..."
	@chmod +x build.sh
	@./build.sh wasm

trunk: ## Build with Trunk only
	@echo "$(BLUE)[INFO]$(NC) Building with Trunk..."
	@chmod +x build.sh
	@./build.sh trunk

clean: ## Clean build artifacts
	@echo "$(BLUE)[INFO]$(NC) Cleaning build artifacts..."
	@rm -rf $(RELEASE_DIR)
	@rm -rf $(PKG_DIR)
	@cargo clean
	@echo "$(GREEN)[SUCCESS]$(NC) Clean completed"

# Development targets
serve: dev ## Alias for dev command

test: ## Run all tests (Rust + WASM)
	@echo "$(BLUE)[INFO]$(NC) Running tests..."
	@cargo test $(CARGO_FLAGS)
	@echo "$(GREEN)[SUCCESS]$(NC) All tests passed"

test-unit: ## Run unit tests only
	@echo "$(BLUE)[INFO]$(NC) Running unit tests..."
	@cargo test $(CARGO_FLAGS) --lib

test-integration: ## Run integration tests only
	@echo "$(BLUE)[INFO]$(NC) Running integration tests..."
	@cargo test $(CARGO_FLAGS) --test '*'

test-wasm: ## Run WASM-specific tests
	@echo "$(BLUE)[INFO]$(NC) Running WASM tests..."
	@cargo test $(CARGO_FLAGS) --target $(WASM_TARGET)

check: ## Run cargo check on all packages
	@echo "$(BLUE)[INFO]$(NC) Running cargo check..."
	@cargo check $(CARGO_FLAGS)
	@echo "$(GREEN)[SUCCESS]$(NC) Check completed"

lint: ## Run clippy linter
	@echo "$(BLUE)[INFO]$(NC) Running clippy..."
	@cargo clippy $(CARGO_FLAGS) -- -D warnings
	@echo "$(GREEN)[SUCCESS]$(NC) Linting completed"

format: ## Format code with rustfmt
	@echo "$(BLUE)[INFO]$(NC) Formatting code..."
	@cargo fmt $(CARGO_FLAGS)
	@echo "$(GREEN)[SUCCESS]$(NC) Formatting completed"

format-check: ## Check code formatting
	@echo "$(BLUE)[INFO]$(NC) Checking code formatting..."
	@cargo fmt $(CARGO_FLAGS) -- --check
	@echo "$(GREEN)[SUCCESS]$(NC) Format check completed"

# Setup targets
install-tools: ## Install required build tools
	@echo "$(BLUE)[INFO]$(NC) Installing build tools..."
	@chmod +x build.sh
	@./build.sh install-tools

install-rust: ## Install Rust toolchain
	@echo "$(BLUE)[INFO]$(NC) Installing Rust toolchain..."
	@curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
	@source ~/.cargo/env
	@rustup target add $(WASM_TARGET)
	@echo "$(GREEN)[SUCCESS]$(NC) Rust toolchain installed"

install-wasm-pack: ## Install wasm-pack
	@echo "$(BLUE)[INFO]$(NC) Installing wasm-pack..."
	@cargo install wasm-pack
	@echo "$(GREEN)[SUCCESS]$(NC) wasm-pack installed"

install-trunk: ## Install trunk
	@echo "$(BLUE)[INFO]$(NC) Installing trunk..."
	@cargo install trunk
	@echo "$(GREEN)[SUCCESS]$(NC) trunk installed"

install-binaryen: ## Install binaryen (wasm-opt)
	@echo "$(BLUE)[INFO]$(NC) Installing binaryen..."
ifeq ($(OS),Windows_NT)
	@echo "$(RED)[ERROR]$(NC) Please install binaryen manually on Windows"
	@echo "Download from: https://github.com/WebAssembly/binaryen/releases"
else
ifeq ($(shell uname),Darwin)
	@brew install binaryen
else
	@sudo apt-get update && sudo apt-get install -y binaryen
endif
endif
	@echo "$(GREEN)[SUCCESS]$(NC) binaryen installed"

# Documentation targets
docs: ## Generate documentation
	@echo "$(BLUE)[INFO]$(NC) Generating documentation..."
	@cargo doc $(CARGO_FLAGS) --no-deps --open
	@echo "$(GREEN)[SUCCESS]$(NC) Documentation generated"

docs-serve: ## Serve documentation locally
	@echo "$(BLUE)[INFO]$(NC) Serving documentation..."
	@cargo doc $(CARGO_FLAGS) --no-deps
	@cd target/doc && python3 -m http.server 8000

# Performance targets
bench: ## Run benchmarks
	@echo "$(BLUE)[INFO]$(NC) Running benchmarks..."
	@cargo bench $(CARGO_FLAGS)
	@echo "$(GREEN)[SUCCESS]$(NC) Benchmarks completed"

profile: ## Profile the application
	@echo "$(BLUE)[INFO]$(NC) Profiling application..."
	@cargo build --release $(CARGO_FLAGS)
	@echo "$(GREEN)[SUCCESS]$(NC) Profile build completed"

# Release targets
release: ## Build release version
	@echo "$(BLUE)[INFO]$(NC) Building release version..."
	@cargo build --release $(CARGO_FLAGS)
	@wasm-pack build --target web --out-dir $(PKG_DIR) --out-name helios --release --no-typescript --no-pack
	@trunk build --release
	@echo "$(GREEN)[SUCCESS]$(NC) Release build completed"

release-check: ## Check release build
	@echo "$(BLUE)[INFO]$(NC) Checking release build..."
	@cargo check --release $(CARGO_FLAGS)
	@echo "$(GREEN)[SUCCESS]$(NC) Release check completed"

# CI/CD targets
ci: ## Run CI pipeline (check, test, lint, format-check)
	@echo "$(BLUE)[INFO]$(NC) Running CI pipeline..."
	@$(MAKE) check
	@$(MAKE) test
	@$(MAKE) lint
	@$(MAKE) format-check
	@echo "$(GREEN)[SUCCESS]$(NC) CI pipeline completed"

ci-test: ## Run tests for CI
	@echo "$(BLUE)[INFO]$(NC) Running CI tests..."
	@cargo test $(CARGO_FLAGS) --verbose

# Development workflow targets
watch: ## Watch for changes and rebuild
	@echo "$(BLUE)[INFO]$(NC) Watching for changes..."
	@cargo watch -x "check" -x "test"

watch-test: ## Watch for changes and run tests
	@echo "$(BLUE)[INFO]$(NC) Watching for changes and running tests..."
	@cargo watch -x "test"

# Utility targets
size: ## Show build size information
	@echo "$(BLUE)[INFO]$(NC) Build size information:"
	@du -sh $(RELEASE_DIR) 2>/dev/null || echo "No release build found"
	@du -sh $(PKG_DIR) 2>/dev/null || echo "No WASM build found"
	@cargo tree --duplicates

deps: ## Show dependency information
	@echo "$(BLUE)[INFO]$(NC) Dependency information:"
	@cargo tree
	@cargo tree --duplicates

update: ## Update dependencies
	@echo "$(BLUE)[INFO]$(NC) Updating dependencies..."
	@cargo update $(CARGO_FLAGS)
	@echo "$(GREEN)[SUCCESS]$(NC) Dependencies updated"

# Environment targets
env: ## Show environment information
	@echo "$(BLUE)[INFO]$(NC) Environment information:"
	@echo "Rust version: $$(rustc --version)"
	@echo "Cargo version: $$(cargo --version)"
	@echo "wasm-pack version: $$(wasm-pack --version 2>/dev/null || echo 'Not installed')"
	@echo "trunk version: $$(trunk --version 2>/dev/null || echo 'Not installed')"
	@echo "wasm-opt version: $$(wasm-opt --version 2>/dev/null || echo 'Not installed')"
	@echo "Target: $(WASM_TARGET)"
	@echo "Release dir: $(RELEASE_DIR)"
	@echo "Package dir: $(PKG_DIR)"
