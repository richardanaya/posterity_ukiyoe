EXAMPLES := $(wildcard examples/*)

.default: build

.PHONY: build lint test clean build-ukiyoe lint-ukiyoe test-ukiyoe clean-ukiyoe $(EXAMPLES) run-example-basic run-example-rectangular

############## PROJECT TOP LEVEL ##############

build: build-ukiyoe $(EXAMPLES)
	$(MAKE) -C $(EXAMPLES) build

lint: lint-ukiyoe
	$(MAKE) -C $(EXAMPLES) lint

test: test-ukiyoe
	$(MAKE) -C $(EXAMPLES) test

clean: clean-ukiyoe
	$(MAKE) -C $(EXAMPLES) clean

############## UKIYOE TOP LEVEL ##############

build-ukiyoe:
	cargo build

lint-ukiyoe:
	cargo fmt

test-ukiyoe:
	cargo test

clean-ukiyoe:
	cargo clean

############## EXAMPLES ##############

$(EXAMPLES): build-ukiyoe
	$(MAKE) -C $@ build

run-example-basic: build
	$(MAKE) -C examples/basic run

run-example-rectangular: build
	$(MAKE) -C examples/rectangular run
