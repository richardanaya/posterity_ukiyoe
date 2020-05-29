.PHONY: build lint test examples

build:
	cargo build
lint:
	cargo fmt
test:
	cargo test
examples:
	(cd examples/basic; cargo build)
