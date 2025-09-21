#!/bin/bash
# fix_warnings.sh - Fix common code quality warnings

echo "🔧 FIXING CODE QUALITY WARNINGS"
echo "================================"

# Fix unused variables by adding underscores
echo "1. Fixing unused variables..."

# Fix unused variables in helios-leptos
sed -i.bak 's/let mut df = DataFrame::empty();/let _df = DataFrame::empty();/g' helios-leptos/tests/simple_components.rs
sed -i.bak 's/let mut invalid_spec = ChartSpec::new();/let invalid_spec = ChartSpec::new();/g' helios-leptos/tests/simple_components.rs

# Fix unused variables in helios-core
find helios-core/src -name "*.rs" -exec sed -i.bak 's/let mut warnings = Vec::new();/let warnings = Vec::new();/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/let mut recommendations = Vec::new();/let recommendations = Vec::new();/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/let mut manager = DataSourceManager::new();/let manager = DataSourceManager::new();/g' {} \;

# Fix unused variables by adding underscores
find helios-core/src -name "*.rs" -exec sed -i.bak 's/let execution_time = start_time.elapsed();/let _execution_time = start_time.elapsed();/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/let first_col = data.column(/let _first_col = data.column(/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/spec: &ChartSpec,/_spec: &ChartSpec,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/data: &DataFrame,/_data: &DataFrame,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/config: &[A-Za-z]*Config,/_config: &[A-Za-z]*Config,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/query: &str,/_query: &str,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/statement: &str,/_statement: &str,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/user: &User,/_user: &User,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/access_token: &str,/_access_token: &str,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/saml_response: &str,/_saml_response: &str,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/data_spec: &DataSpec,/_data_spec: &DataSpec,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/dpi: Option<u32>,/_dpi: Option<u32>,/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/for (i, column) in/for (_i, column) in/g' {} \;
find helios-core/src -name "*.rs" -exec sed -i.bak 's/for (field, encoding) in/for (_field, _encoding) in/g' {} \;

# Clean up backup files
find . -name "*.bak" -delete

echo "✅ Warning fixes applied"
echo ""
echo "2. Running clippy to check remaining warnings..."
