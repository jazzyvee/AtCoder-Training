#!/usr/bin/env bash
# Generate mdBook pages (book/src/*.md, book/src/SUMMARY.md) from atcoder/<contest>/
# Fully offline: no network access, safe to run in CI or locally at any time.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
ATCODER_DIR="$REPO_ROOT/atcoder"
SRC_DIR="$SCRIPT_DIR/src"

# cargo-compete's untouched template (compete.toml [template] src), whitespace-normalized.
UNSOLVED_MARKER="fnmain(){todo!();}"

mkdir -p "$SRC_DIR"
find "$SRC_DIR" -maxdepth 1 -name '*.md' -delete

contests=()

for cargo_toml in "$ATCODER_DIR"/*/Cargo.toml; do
    [ -e "$cargo_toml" ] || continue
    contest_dir="$(dirname "$cargo_toml")"
    contest="$(basename "$contest_dir")"

    mapfile -t bin_lines < <(grep -E '^[a-zA-Z0-9_-]+ = \{ alias = "[^"]+", problem = "[^"]+" \}' "$cargo_toml" || true)

    problem_entries=()
    for line in "${bin_lines[@]}"; do
        alias="$(sed -E 's/.*alias = "([^"]+)".*/\1/' <<<"$line")"
        url="$(sed -E 's/.*problem = "([^"]+)".*/\1/' <<<"$line")"
        src_file="$contest_dir/src/bin/$alias.rs"
        [ -f "$src_file" ] || continue

        normalized="$(tr -d '[:space:]' < "$src_file")"
        if [ "$normalized" = "$UNSOLVED_MARKER" ]; then
            solved="0"
        else
            solved="1"
        fi

        problem_entries+=("$alias|$url|$solved")
    done

    memo_file="$contest_dir/memo.md"
    if [ "${#problem_entries[@]}" -eq 0 ] && [ ! -f "$memo_file" ]; then
        continue
    fi

    contests+=("$contest")

    page="$SRC_DIR/$contest.md"
    {
        echo "# $contest"
        echo

        if [ -f "$memo_file" ]; then
            echo "## メモ"
            echo
            echo "{{#include ../../atcoder/$contest/memo.md}}"
            echo
        fi

        for entry in "${problem_entries[@]}"; do
            alias="$(cut -d'|' -f1 <<<"$entry")"
            url="$(cut -d'|' -f2 <<<"$entry")"
            solved="$(cut -d'|' -f3 <<<"$entry")"
            problem_id="${url##*/}"

            if [ "$solved" = "1" ]; then
                echo "## <span data-problem-id=\"$problem_id\">$alias</span>"
                echo
                echo "[問題ページ]($url)"
                echo
                echo '```rust'
                echo "{{#include ../../atcoder/$contest/src/bin/$alias.rs}}"
                echo '```'
                echo
            else
                echo "## <span data-problem-id=\"$problem_id\">$alias</span>（未回答）"
                echo
                echo "[問題ページ]($url)"
                echo
            fi
        done
    } > "$page"
done

sorted=()
if [ "${#contests[@]}" -gt 0 ]; then
    IFS=$'\n' sorted=($(sort <<<"${contests[*]}"))
    unset IFS
fi

{
    echo "# Summary"
    echo
    for c in "${sorted[@]}"; do
        echo "- [$c](./$c.md)"
    done
} > "$SRC_DIR/SUMMARY.md"
