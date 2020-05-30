EXAMPLES := $(wildcard examples/*)

.default: build

.PHONY: build lint test clean build-ukiyoe lint-ukiyoe test-ukiyoe clean-ukiyoe $(EXAMPLES) run-example-basic run-example-rectangular

############## PROJECT TOP LEVEL ##############

build: build-ukiyoe $(EXAMPLES)

lint: lint-ukiyoe
	for dir in $(EXAMPLES); do \
		$(MAKE) -C $$dir lint; \
	done

test: test-ukiyoe
	for dir in $(EXAMPLES); do \
		$(MAKE) -C $$dir test; \
	done

clean: clean-ukiyoe
	for dir in $(EXAMPLES); do \
		$(MAKE) -C $$dir clean; \
	done

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
	$(MAKE) -C $@

run-example-basic: build
	$(MAKE) -C examples/basic run

run-example-rectangular: build
	$(MAKE) -C examples/rectangular run
