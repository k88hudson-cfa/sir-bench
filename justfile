

help:
    cargo run --bin sir_100k -- --help

# Install hyperfine if not already installed
setup:
    @if ! command -v hyperfine >/dev/null; then \
        echo "Installing hyperfine..."; \
        cargo install hyperfine; \
    else \
        echo "You're good to go, hyperfine is installed."; \
    fi

# Build the a release version for a more representative runtime
build:
  cargo build --release

# Run a specific model: just run ixa, just run baseline
run *args: build
  ./target/release/sir_100k {{ args }}

# Run benchmark comparison
compare *args: build
  hyperfine --warmup 1 --runs 3 \
  './target/release/sir_100k --model baseline {{ args }}' \
  './target/release/sir_100k --model ixa {{ args }}'
