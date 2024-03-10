fmt: 
	cargo fmt --check

clippy:
	cargo clippy --verbose

build: 
	cargo build

test: 
	cargo test -- --test-threads=3

run:
	RUST_LOG=none,crawler=debug cargo run --release
	
run-info:
	RUST_LOG=none,crawler=info cargo run --release