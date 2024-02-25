fmt: 
	cargo fmt --check

clippy:
	cargo clippy --verbose

build: 
	cargo build

run: 
	cargo run --release

test: 
	cargo test -- --test-threads=3

bench: 
	cargo bench