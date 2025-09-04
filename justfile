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

# Run a specific model: just run large --model ixa -t 10
run bench_name="sir" *args:
   cargo run --bin {{ bench_name }} --release -- {{ args }}

# Run benchmark comparison
compare bench_name="sir" *args: build
  hyperfine --warmup 1 --runs 3 \
   './target/release/{{ bench_name }} --model baseline {{ args }}' \
  './target/release/{{ bench_name }} --model ixa {{ args }}' \
  './target/release/{{ bench_name }} --model ixa-no-queries {{ args }}' \

