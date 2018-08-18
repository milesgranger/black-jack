

VERSION := 0.0.1

build-docs:
	cargo doc --no-deps --open

test-rust:
	cargo test