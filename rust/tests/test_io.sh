#!/bin/bash
# 標準入出力問題のテストスクリプト
# 使い方: bash tests/test_io.sh

set -e
cd "$(dirname "$0")/.."
cargo build --bins 2>/dev/null

PASS=0
FAIL=0

check() {
    local name="$1" input="$2" expected="$3" args="$4"
    local actual
    actual=$(echo -e "$input" | cargo run --quiet --bin "$name" -- $args 2>/dev/null)
    if [ "$actual" = "$expected" ]; then
        echo "  PASS: $name ($5)"
        ((PASS++))
    else
        echo "  FAIL: $name ($5)"
        echo "    expected: '$expected'"
        echo "    actual:   '$actual'"
        ((FAIL++))
    fi
}

echo "=== 標準入出力テスト ==="

echo "[p11_echo]"
check p11_echo "Hello" "Hello" "" "basic"
check p11_echo "Rust is fun" "Rust is fun" "" "with spaces"

echo "[p12_line_count]"
check p12_line_count "aaa\nbbb\nccc" "3" "" "3 lines"
check p12_line_count "one" "1" "" "1 line"

echo "[p13_sum_lines]"
check p13_sum_lines "10\n20\n30" "60" "" "basic"
check p13_sum_lines "5" "5" "" "single"
check p13_sum_lines "-1\n1" "0" "" "negative"

echo "[p14_grep]"
check p14_grep "apple\nbanana\napricot" "apple\napricot" "ap" "pattern ap"
check p14_grep "hello\nworld" "hello\nworld" "l" "pattern l"
check p14_grep "aaa\nbbb" "" "xyz" "no match"

echo "[p15_csv_sum]"
check p15_csv_sum "alice,10\nbob,20\nalice,30" "alice,40\nbob,20" "" "basic"
check p15_csv_sum "x,1" "x,1" "" "single"

echo ""
echo "=== 結果: $PASS passed, $FAIL failed ==="
[ "$FAIL" -eq 0 ]
