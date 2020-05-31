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
	$(MAKE) -C ukiyoe build

lint-ukiyoe:
	$(MAKE) -C ukiyoe lint

test-ukiyoe:
	$(MAKE) -C ukiyoe test

clean-ukiyoe:
	$(MAKE) -C ukiyoe clean

############## EXAMPLES ##############

$(EXAMPLES): build-ukiyoe
	$(MAKE) -C $@

run-example-basic: build
	$(MAKE) -C examples/basic run

run-example-rectangular: build
	$(MAKE) -C examples/rectangular run

run-example-basic_console: build
	$(MAKE) -C examples/basic_console run
