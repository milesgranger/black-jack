

VERSION := 0.0.1

build-docs:
	cargo doc --no-deps --open

coverage:
	cargo tarpaulin -v

test:
	cargo test