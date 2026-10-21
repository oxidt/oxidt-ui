tailwind := env("TAILWIND", "tailwindcss")

# Compile the component sources.
check:
    cargo clippy --all-targets -- -D warnings
    cargo test

# Build the visual showcase into tests/fixture/out/ (set TAILWIND to a standalone CLI).
showcase:
    mkdir -p tests/fixture/out
    {{tailwind}} -i tests/fixture/showcase.css -o tests/fixture/out/oxidt.css
    cp tests/fixture/showcase.html tests/fixture/out/showcase.html

# Screenshot an app's pages in light and dark into OUT (see bench/shoot.ts).
# Paths are relative to where you run it, e.g. from an app:
#   just -f ../oxidt-ui/justfile bench-shoot http://127.0.0.1:8080 bench/before pages.txt
[no-cd]
bench-shoot base out pages *args:
    cd {{justfile_directory()}}/bench && bun install --frozen-lockfile --silent && bunx playwright install chromium
    bun {{justfile_directory()}}/bench/shoot.ts {{base}} {{out}} {{pages}} {{args}}

# Compare two bench-shoot runs, worst page first (see bench/diff.ts).
[no-cd]
bench-diff before after *args:
    bun {{justfile_directory()}}/bench/diff.ts {{before}} {{after}} {{args}}

# Browser regression checks for the theme (set TAILWIND to a standalone CLI).
browser-test:
    cd bench && bun install --frozen-lockfile --silent && bunx playwright install chromium
    cd bench && TAILWIND="{{tailwind}}" bun test
