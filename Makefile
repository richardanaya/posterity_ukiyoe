EXAMPLES := $(wildcard examples/*)
RENDERERS := "ukiyoe_curses"

.default: build

.PHONY: build lint test clean build-ukiyoe lint-ukiyoe test-ukiyoe clean-ukiyoe $(EXAMPLES) $(RENDERERS)

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

run-example-curses: examples/curses
	(cd examples/curses; cargo run;)

run-example-glfw_opengl3: examples/glfw_opengl3
	(cd examples/glfw_opengl3; cargo run;)
