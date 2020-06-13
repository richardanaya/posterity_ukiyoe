EXAMPLES := $(wildcard examples/*)
RENDERERS := "ukiyoe_curses"

.default: build

.PHONY: build lint test clean build-ukiyoe lint-ukiyoe test-ukiyoe clean-ukiyoe $(EXAMPLES) $(RENDERERS) run-example-basic run-example-rectangular

############## PROJECT TOP LEVEL ##############

build: build-ukiyoe $(RENDERERS) $(EXAMPLES)

lint: lint-ukiyoe
	for dir in $(EXAMPLES); do \
		(cd $$dir; cargo lint;) \
	done

test: test-ukiyoe
	for dir in $(EXAMPLES); do \
		(cd $$dir; cargo test;) \
	done

clean: clean-ukiyoe
	for dir in $(EXAMPLES); do \
		(cd $$dir; cargo clean;) \
	done

############## UKIYOE TOP LEVEL ##############

build-ukiyoe:
	(cd ukiyoe; cargo build;)

lint-ukiyoe:
	(cd ukiyoe; cargo lint;)

test-ukiyoe:
	(cd ukiyoe; cargo test;)

clean-ukiyoe:
	(cd ukiyoe; cargo clean;)

############## RENDERERS TOP LEVEL ##############

$(RENDERERS): build-ukiyoe
	(cd $@; cargo build)

############## EXAMPLES ##############

$(EXAMPLES): build-ukiyoe
	(cd $@; cargo build)

run-example-curses: build
	(cd examples/curses; cargo run;)

run-example-rectangular: build
	$(MAKE) -C examples/rectangular run

run-example-basic_console: build
	$(MAKE) -C examples/basic_console run
