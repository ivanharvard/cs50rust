SHELL := /bin/bash

CHECK50_REPO := ivanharvard/cs50rust/main/checks

CC := clang
CFLAGS := -ggdb3 -O0 -std=c11 -Wall -Werror -Wextra -Wpedantic
RUSTC := rustc

MAKEFLAGS += -rR
.SUFFIXES:

.DEFAULT_GOAL := help

.PHONY: help clean clean-all
Makefile: ;
%.c: ;
%.h: ;
%.rs: ;
%.o: ;
%/Makefile: ;

help:
	@echo "Usage:"
	@echo "  make <target>"
	@echo "  make test-<target>"
	@echo "  make check-<target>"
	@echo "  make check-<target> check_path=<slug>"
	@echo ""
	@echo "Examples:"
	@echo "  make pset4/filter-less/filter"
	@echo "  make test-pset5/inheritance/inheritance"
	@echo "  make check-pset4/filter-less/filter"
	@echo "  make check-pset4/filter-less/filter check_path=filter/less"
	@echo ""
	@echo "test-<target> convention:"
	@echo "  target:      pset5/inheritance/inheritance"
	@echo "  test C:      pset5/inheritance/inheritance_test.c"
	@echo "  Rust source: pset5/inheritance/rust/inheritance.rs"
	@echo "  output:      pset5/inheritance/inheritance_test"
	@echo ""
	@echo "check50 slug format:"
	@echo "  $(CHECK50_REPO)/<check_path>"

clean:
	@echo "Cleaning common build artifacts..."
	@find . -type f \( \
		-name '*.o' -o \
		-name '*.out' -o \
		-name '*.exe' -o \
		-name '*.a' -o \
		-name '*.so' \
	\) -delete
	@find . -type f \( \
		-name 'inheritance_test' \
	\) -delete 2>/dev/null || true
	@find . -type d -name 'target' -prune -exec rm -rf {} +
	@find . -type d -name '__pycache__' -prune -exec rm -rf {} +

clean-all: clean
	@echo "Cleaning common CS50-style binaries..."
	@find ./pset* -maxdepth 3 -type f \( \
		-name 'filter' -o \
		-name 'recover' -o \
		-name 'speller' -o \
		-name 'volume' -o \
		-name 'caesar' -o \
		-name 'substitution' -o \
		-name 'plurality' -o \
		-name 'runoff' -o \
		-name 'tideman' -o \
		-name 'sort' -o \
		-name 'scrabble' -o \
		-name 'readability' -o \
		-name 'credit' -o \
		-name 'cash' -o \
		-name 'mario' -o \
		-name 'hello' -o \
		-name 'inheritance' -o \
		-name 'inheritance_test' \
	\) -delete 2>/dev/null || true

.DEFAULT:
	@requested="$@"; \
	\
	if [[ "$$requested" == check-* ]]; then \
		target="$${requested#check-}"; \
		target_dir="$$(dirname "$$target")"; \
		slug_suffix="$(if $(check_path),$(check_path),$${requested#check-})"; \
		echo "Building $$target before running checks..."; \
		$(MAKE) "$$target" || exit $$?; \
		echo "Running check50 from $$target_dir..."; \
		echo "Slug: $(CHECK50_REPO)/$$slug_suffix"; \
		cd "$$target_dir" && check50 $(CHECK50_REPO)/$$slug_suffix --local; \
		exit $$?; \
	fi; \
	\
	if [[ "$$requested" == test-* ]]; then \
		target="$${requested#test-}"; \
		dir="$$(dirname "$$target")"; \
		name="$$(basename "$$target")"; \
		test_c="$$dir/$${name}_test.c"; \
		rust_src="$$dir/rust/$${name}.rs"; \
		out="$$dir/$${name}_test"; \
		lib="$$dir/.lib$${name}_test.a"; \
		\
		echo "Building test target for $$target..."; \
		\
		if [ -f "$$dir/Makefile" ] && [ "$$dir" != "." ]; then \
			echo "Delegating to $$dir/Makefile -> test-$$name"; \
			$(MAKE) -C "$$dir" "test-$$name"; \
		elif [ -f "$$test_c" ] && [ -f "$$rust_src" ]; then \
			echo "Compiling Rust staticlib $$rust_src"; \
			$(RUSTC) --crate-type staticlib --edition 2021 "$$rust_src" -o "$$lib"; \
			echo "Compiling C test harness $$test_c"; \
			$(CC) $(CFLAGS) "$$test_c" "$$lib" -o "$$out"; \
		elif [ -f "$$test_c" ]; then \
			echo "Compiling C test harness $$test_c"; \
			$(CC) $(CFLAGS) "$$test_c" -o "$$out"; \
		else \
			echo "Error: do not know how to build test target '$$requested'"; \
			echo "Expected at least: $$test_c"; \
			exit 1; \
		fi; \
		exit $$?; \
	fi; \
	\
	target="$$requested"; \
	dir="$$(dirname "$$target")"; \
	name="$$(basename "$$target")"; \
	\
	echo "Building $$target..."; \
	\
	if [ -f "$$dir/Makefile" ] && [ "$$dir" != "." ]; then \
		echo "Delegating to $$dir/Makefile -> $$name"; \
		$(MAKE) -C "$$dir" "$$name"; \
	elif [ -f "$$dir/$$name.c" ] && [ -f "$$dir/rust/$$name.rs" ]; then \
		echo "Compiling Rust staticlib $$dir/rust/$$name.rs"; \
		$(RUSTC) --crate-type staticlib --edition 2021 "$$dir/rust/$$name.rs" -o "$$dir/.lib$$name.a"; \
		echo "Compiling C wrapper $$dir/$$name.c"; \
		$(CC) $(CFLAGS) "$$dir/$$name.c" "$$dir/.lib$$name.a" -o "$$target"; \
	elif [ -f "$$dir/$$name.c" ]; then \
		echo "Compiling C source $$dir/$$name.c"; \
		$(CC) $(CFLAGS) "$$dir/$$name.c" -o "$$target"; \
	elif [ -f "$$dir/$$name.rs" ]; then \
		echo "Compiling Rust source $$dir/$$name.rs"; \
		$(RUSTC) "$$dir/$$name.rs" -o "$$target"; \
	elif [ -f "$$target.c" ]; then \
		echo "Compiling C source $$target.c"; \
		$(CC) $(CFLAGS) "$$target.c" -o "$$target"; \
	elif [ -f "$$target.rs" ]; then \
		echo "Compiling Rust source $$target.rs"; \
		$(RUSTC) "$$target.rs" -o "$$target"; \
	else \
		echo "Error: do not know how to build '$$target'"; \
		exit 1; \
	fi