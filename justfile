clean: 
	cargo clean

fmt: 
	cargo fmt --check

clippy:
	cargo clippy --verbose

build: 
	cargo build

test: 
	cargo test -- --test-threads=3

run:
	cargo run --release 
	