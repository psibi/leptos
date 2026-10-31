set shell := ["bash", "-euo", "pipefail", "-c"]

default:
    @just --list

# Regenerate mdBook Markdown from the org sources.
convert:
    ./scripts/build-book.sh

# Convert the org sources and render the HTML book.
build: convert
    mdbook build mdbook

# Convert and serve the book locally with live reload.
serve: convert
    mdbook serve mdbook --open

# Remove generated Markdown and HTML.
clean:
    rm -rf mdbook/src mdbook/book

# Build and verify the generated book artifacts.
check: build
    @test -f mdbook/book/index.html
    @test -f mdbook/src/index.md
    @expected="$(find . -maxdepth 1 -type f -name 'chapter*.org' -size +0c ! -name 'chapter1.org' | wc -l)"; \
     actual="$(find mdbook/src -maxdepth 1 -type f -name 'chapter*.md' | wc -l)"; \
     test "$actual" -eq "$expected"; \
     entries="$(grep -c '^- \[' mdbook/src/SUMMARY.md)"; \
     test "$entries" -eq "$expected"
    @! grep -R -n -E '\]\((file:|\./)?(index|chapter[0-9]+(_[0-9]+)?)\.org' mdbook/src --include='*.md'
    @! grep -R -n -E '^#\+(begin_|end_|RESULTS:)' mdbook/src --include='*.md'
    @echo "All checks passed."
