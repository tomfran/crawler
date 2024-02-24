fmt: 
	cargo fmt --check

clippy:
	cargo clippy --verbose

build: 
	cargo build --release

run: 
	cargo run --release

test: 
	cargo test --release