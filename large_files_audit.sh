#!/bin/bash
# large_files_audit.sh
echo "=== FILES >300 LINES ==="
find . -name "*.rs" -exec wc -l {} + | awk '$1 > 300 {print $0}' | sort -nr > large_files.txt
echo "Found $(cat large_files.txt | wc -l) files >300 lines"

echo "=== REFACTORING CANDIDATES ==="
cat large_files.txt | head -20
