SHELL := /bin/bash

CHECK50_REPO := ivanharvard/cs50rust/main/checks

CC := clang
CFLAGS := -ggdb3 -O0 -std=c11 -Wall -Werror -Wextra -Wpedantic
RUSTC := rustc

MAKEFLAGS += -rR
.SUFFIXES:

.DEFAULT_GOAL := help

.PHONY: help clean clean-all test-%
Makefile: ;
%.c: ;
%.h: ;
%.rs: ;
%.o: ;
%/Makefile: ;

help:
	@echo "Usage:"
	@echo "  make <target>"
	@echo "  make test-<name>"
	@echo "  make check-<target>"

clean:
	@find . -type f \( \
		-name '*.o' -o \
		-name '*.out' -o \
		-name '*.exe' -o \
		-name '*.a' -o \
		-name '*.so' -o \
		-name '*_test' \
	\) -delete 2>/dev/null || true
	@find . -type d -name 'target' -prune -exec rm -rf {} +
	@find . -type d -name '__pycache__' -prune -exec rm -rf {} +

clean-all: clean

# make test-inheritance -> build inheritance_test
test-%: %_test
	@test -x "$<" || { echo "Error: expected executable '$<' was not created"; exit 1; }

# Build inheritance_test from inheritance_test.c and rust/inheritance.rs
%_test: %_test.c rust/%.rs
	@echo "Compiling Rust staticlib rust/$*.rs"
	$(RUSTC) --crate-type staticlib --edition 2021 "rust/$*.rs" -o ".lib$*_test.a"
	@echo "Compiling C test harness $<"
	$(CC) $(CFLAGS) "$<" ".lib$*_test.a" -o "$@"
	@echo "Verifying output $@"
	@test -f "$@" || { echo "Error: '$@' was not created"; ls -la; exit 1; }
	@test -x "$@" || { echo "Error: '$@' exists but is not executable"; ls -la "$@"; exit 1; }
	@ls -la "$@"

.DEFAULT:
	@requested="$@"; \
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
	target="$$requested"; \
	dir="$$(dirname "$$target")"; \
	name="$$(basename "$$target")"; \
	echo "Building $$target..."; \
	if [ -f "$$dir/Makefile" ] && [ "$$dir" != "." ]; then \
		$(MAKE) -C "$$dir" "$$name"; \
	elif [ -f "$$dir/$$name.c" ] && [ -f "$$dir/rust/$$name.rs" ]; then \
		$(RUSTC) --crate-type staticlib --edition 2021 "$$dir/rust/$$name.rs" -o "$$dir/.lib$$name.a"; \
		$(CC) $(CFLAGS) "$$dir/$$name.c" "$$dir/.lib$$name.a" -o "$$target"; \
		@test -f "$$target" || { echo "Error: '$$target' was not created"; ls -la "$$dir"; exit 1; }; \
	elif [ -f "$$dir/$$name.c" ]; then \
		$(CC) $(CFLAGS) "$$dir/$$name.c" -o "$$target"; \
		@test -f "$$target" || { echo "Error: '$$target' was not created"; ls -la "$$dir"; exit 1; }; \
	elif [ -f "$$dir/$$name.rs" ]; then \
		$(RUSTC) "$$dir/$$name.rs" -o "$$target"; \
		@test -f "$$target" || { echo "Error: '$$target' was not created"; ls -la "$$dir"; exit 1; }; \
	elif [ -f "$$target.c" ]; then \
		$(CC) $(CFLAGS) "$$target.c" -o "$$target"; \
		@test -f "$$target" || { echo "Error: '$$target' was not created"; ls -la; exit 1; }; \
	elif [ -f "$$target.rs" ]; then \
		$(RUSTC) "$$target.rs" -o "$$target"; \
		@test -f "$$target" || { echo "Error: '$$target' was not created"; ls -la; exit 1; }; \
	else \
		echo "Error: do not know how to build '$$target'"; \
		exit 1; \
	fi