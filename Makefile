.PHONY: build lint test examples run-examples

build:
	cargo build

lint:
	cargo fmt

test:
	cargo test

examples: build
	(cd examples/basic; cargo build)

run-examples: examples
	(cd examples/basic; cargo run)
