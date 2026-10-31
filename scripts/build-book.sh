#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SRC="$ROOT/mdbook/src"

require() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command '$1' not found" >&2
    exit 1
  fi
}

require pandoc

rm -rf "$SRC"
mkdir -p "$SRC"

# The org sources remain the source of truth; this map only controls the
# curated sidebar labels. Unknown chapters fall back to their first heading.
declare -A CHAPTER_TITLES=(
  [chapter2]="Getting Started"
  [chapter2_1]="Development Experience Improvements"
  [chapter2_2]="The Leptos Community and Crates"
  [chapter3]="Part 1: Building User Interfaces"
  [chapter4]="Reactivity"
  [chapter6]="Async"
  [chapter7]="Projecting Children"
  [chapter8]="Global State Management"
  [chapter9]="Router"
  [chapter10]="Styling"
  [chapter11]="Metadata"
)

convert() {
  pandoc "$1" \
    --from=org \
    --to=gfm \
    --wrap=none \
    --markdown-headings=atx \
    --output="$2"
}

# chapter1 is the introduction and becomes the book's landing page.
convert "$ROOT/chapter1.org" "$SRC/index.md"

# Empty chapter files are skipped until they have content. chapter1 is handled
# above, and version sorting keeps chapter10 after chapter9.
mapfile -t chapter_files < <(
  find "$ROOT" -maxdepth 1 -type f -name 'chapter*.org' -size +0c \
    ! -name 'chapter1.org' -print | sort -V
)

{
  printf '# Summary\n\n'
  printf '[Introduction](index.md)\n\n'
  printf '# Chapters\n\n'
} > "$SRC/SUMMARY.md"

for org in "${chapter_files[@]}"; do
  base="$(basename "$org" .org)"

  convert "$org" "$SRC/$base.md"

  if [[ -n "${CHAPTER_TITLES[$base]+x}" ]]; then
    title="${CHAPTER_TITLES[$base]}"
  else
    title="$(sed -n 's/^\* //p' "$org" | head -n 1)"
    if [[ -z "$title" ]]; then
      suffix="${base#chapter}"
      title="Chapter ${suffix//_/.}"
    fi
  fi

  printf -- '- [%s](%s.md)\n' "$title" "$base" >> "$SRC/SUMMARY.md"
done

# Rewrite local org links to their generated Markdown pages. External URLs are
# left untouched, including external domains that happen to end in ".org".
while IFS= read -r -d '' md; do
  sed -E -i \
    -e 's~\]\(file:chapter1\.org(#[^)]*)?\)~](index.md\1)~g' \
    -e 's~\]\(\./chapter1\.org(#[^)]*)?\)~](index.md\1)~g' \
    -e 's~\]\(chapter1\.org(#[^)]*)?\)~](index.md\1)~g' \
    -e 's~\]\(file:(chapter[0-9]+(_[0-9]+)?)\.org(#[^)]*)?\)~](\1.md\3)~g' \
    -e 's~\]\(\./(chapter[0-9]+(_[0-9]+)?)\.org(#[^)]*)?\)~](\1.md\3)~g' \
    -e 's~\]\((chapter[0-9]+(_[0-9]+)?)\.org(#[^)]*)?\)~](\1.md\3)~g' \
    -e 's#\]\(file:#](#g' \
    "$md"
done < <(find "$SRC" -maxdepth 1 -type f -name '*.md' -print0)

echo "Generated mdBook sources in $SRC"
