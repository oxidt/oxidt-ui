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
