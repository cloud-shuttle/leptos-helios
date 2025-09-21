#!/bin/bash
# audit_stubs.sh
echo "=== STUB CODE AUDIT ==="
echo "Functions returning 'not implemented':"
grep -r "not implemented" --include="*.rs" . | wc -l

echo "TODO! macro usage:"
grep -r "todo!()" --include="*.rs" . | wc -l

echo "Unimplemented! macro usage:"
grep -r "unimplemented!()" --include="*.rs" . | wc -l

echo "=== TOP STUB FILES ==="
find . -name "*.rs" -exec grep -l "not implemented\|todo!()\|unimplemented!()" {} \; | head -10
