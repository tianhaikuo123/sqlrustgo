#!/bin/bash
# SQLRustGo Pre-Release Gate Check Script
# Usage: bash scripts/pre-release.sh
# This script runs all quality checks required before a release.

set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

PASS=0
FAIL=0

check() {
    local name="$1"
    shift
    echo -n "  [$name] ... "
    if "$@" > /dev/null 2>&1; then
        echo -e "${GREEN}PASS${NC}"
        PASS=$((PASS + 1))
    else
        echo -e "${RED}FAIL${NC}"
        FAIL=$((FAIL + 1))
    fi
}

echo ""
echo "=========================================="
echo "  SQLRustGo Pre-Release Gate Check"
echo "=========================================="
echo ""

# ---- Gate 1: Format Check ----
echo -e "${YELLOW}[1/6] Code Format Check (cargo fmt)${NC}"
check "fmt" cargo fmt -- --check

# ---- Gate 2: Clippy Lint ----
echo -e "${YELLOW}[2/6] Clippy Lint (cargo clippy)${NC}"
check "clippy" cargo clippy --all-targets -- -D warnings

# ---- Gate 3: Build ----
echo -e "${YELLOW}[3/6] Build (cargo build)${NC}"
check "build" cargo build --all

# ---- Gate 4: Test ----
echo -e "${YELLOW}[4/6] Test (cargo test)${NC}"
check "test" cargo test --all

# ---- Gate 5: Security Audit ----
echo -e "${YELLOW}[5/6] Security Audit (cargo audit)${NC}"
check "audit" cargo audit

# ---- Gate 6: License Check ----
echo -e "${YELLOW}[6/6] License Check (cargo deny)${NC}"
check "deny" cargo deny check

# ---- Summary ----
echo ""
echo "=========================================="
TOTAL=$((PASS + FAIL))
if [ $FAIL -eq 0 ]; then
    echo -e "  ${GREEN}ALL CHECKS PASSED ($PASS/$TOTAL)${NC}"
    echo "  Ready for release!"
    echo "=========================================="
    exit 0
else
    echo -e "  ${RED}$FAIL CHECKS FAILED ($PASS/$TOTAL passed)${NC}"
    echo "  Fix the failures before release."
    echo "=========================================="
    exit 1
fi
