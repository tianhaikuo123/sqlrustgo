#!/bin/bash
# SQLRustGo Pre-Release Gate Check Script
# Usage: bash scripts/pre-release.sh
set -e

echo "============================================"
echo "  SQLRustGo Pre-Release Gate Check"
echo "============================================"
echo ""

PASS=0
FAIL=0

check() {
    local name="$1"
    shift
    echo "🔍 [$name] Running..."
    if "$@"; then
        echo "   ✅ PASS: $name"
        PASS=$((PASS + 1))
    else
        echo "   ❌ FAIL: $name"
        FAIL=$((FAIL + 1))
    fi
    echo ""
}

# 1. Format Check
check "cargo fmt" cargo fmt --all -- --check

# 2. Clippy Check
check "cargo clippy" cargo clippy --all-targets --all-features -- -D warnings

# 3. Build Check
check "cargo build" cargo build --all-features

# 4. Test Check
check "cargo test" cargo test --all-features

# 5. Security Audit
check "cargo audit" cargo audit

# 6. License Check (if cargo-deny installed)
if command -v cargo-deny &>/dev/null; then
    check "cargo deny" cargo deny check
else
    echo "⚠️  [cargo deny] SKIPPED — cargo-deny not installed"
    echo "   Install: cargo install cargo-deny --locked"
    echo ""
fi

echo "============================================"
echo "  RESULTS: $PASS passed, $FAIL failed"
echo "============================================"

if [ "$FAIL" -gt 0 ]; then
    echo ""
    echo "❌ PRE-RELEASE CHECKS FAILED!"
    echo "   Fix the issues above before releasing."
    exit 1
else
    echo ""
    echo "✅ ALL CHECKS PASSED — Ready for release!"
    exit 0
fi
