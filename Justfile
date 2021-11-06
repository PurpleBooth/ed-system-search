# This help screen
show-help:
	just --list

# Test it was built ok
test:
	RUST_BACKTRACE=1 cargo test

# Test the markdown in the docs directory
specdown: build
	./runners/specdown-runner ./README.md docs/*.md

# Run a smoke test and see if the app runs
smoke-test: build
	cargo run --bin ed-system-search -- -h

# Build release version
build:
	cargo build --release

# Lint it
lint:
	cargo fmt --all -- --check
	cargo clippy --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo
	cargo check
	cargo audit

# Format what can be formatted
fmt:
	cargo fix --allow-dirty --allow-staged
	cargo +nightly clippy --allow-dirty --allow-staged --fix -Z unstable-options --all-features -- -D warnings -Dclippy::all -D clippy::pedantic -D clippy::cargo -D clippy::nursery
	cargo fmt --all
	yamlfmt -w .github/*.yml .github/workflows/*.yml .*.yml

# Clean the build directory
clean:
	cargo clean
