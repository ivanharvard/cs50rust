# ---- config ----
RUSTC      := rustc
CC         := gcc
PYTHON     := python3

# Extra libs sometimes needed when linking Rust staticlibs that use std
RUST_LINK_LIBS := -lpthread -ldl -lm -lcs50

# Default target: show help
.PHONY: help
help:
	@echo "Usage:"
	@echo "  make path/to/file            # builds executable from path/to/file_stub.c and path/to/rust/file.rs"
	@echo "  make path/to/file check=1    # runs check50 for that path"
	@echo ""
	@echo "Examples:"
	@echo "  make hello                      # builds ./hello from hello_stub.c and rust/hello.rs"
	@echo "  make pset1/world/hello          # builds pset1/world/hello"
	@echo "  make pset1/world/hello check=1  # runs check50 ivanharvard/cs50r/checks/world/ --local"

# Force target for check runs
.PHONY: FORCE
FORCE:

# Explicit rule for Makefile to prevent pattern rule from matching it
Makefile: ;

# Enable secondary expansion for pattern rule prerequisites
.SECONDEXPANSION:

# Explicit rule for *_test targets (used by check50 for unit tests)
# Compiles rust_src (default: same name) and links with C test file
%_test: $$(notdir $$@).c
	$(eval DIR := $(dir $@))
	$(eval BASENAME := $(patsubst %_test,%,$(notdir $@)))
	$(eval RUST_SRC_NAME := $(if $(rust_src),$(rust_src),$(BASENAME)))
	$(eval RUST_FILE := $(if $(DIR),$(DIR),.)rust/$(RUST_SRC_NAME).rs)
	$(eval TEST_FILE := $@.c)
	$(eval RUST_LIB := $(if $(DIR),$(DIR),.).librust_$(BASENAME).a)

	cd $(if $(DIR),$(DIR),.) && $(RUSTC) --crate-type staticlib --edition 2021 rust/$(RUST_SRC_NAME).rs -o .librust_$(BASENAME).a
	$(CC) $(TEST_FILE) $(RUST_LIB) $(RUST_LINK_LIBS) -o $@
	@rm -f $(RUST_LIB)
	@echo "Built: $@"

# Pattern rule to build executable from path OR run check50
# Example: make pset1/world/hello or just make hello
# Looks for: dir/file_stub.c and dir/rust/file.rs
# Produces: dir/file executable (or runs check50 if check=1)
# Optional: check_path=path/to/check to override default parent dir heuristic
%: $$(if $$(check),FORCE,$$(dir $$@)$$(notdir $$@).c $$(dir $$@)rust/$$(notdir $$@).rs)
ifeq ($(filter Makefile,$@),)
ifdef check
	$(eval FULL_PATH := $(patsubst %/,%,$@))
	$(eval CHECK_DIR := $(dir $(FULL_PATH)))
ifdef check_path
	$(eval CHECK_NAME := $(check_path))
	@echo "Running check50 in $(CHECK_DIR)..."
	@echo "Using custom check path: checks/$(CHECK_NAME)"
else
	$(eval CHECK_NAME := $(notdir $(patsubst %/,%,$(CHECK_DIR))))
	@echo "Running check50 in $(CHECK_DIR)..."
endif
	@cd $(CHECK_DIR) && check50 ivanharvard/cs50r/main/checks/$(CHECK_NAME) --local
else
	$(eval DIR := $(dir $@))
	$(eval BASENAME := $(notdir $@))
	$(eval RUST_SRC_NAME := $(if $(rust_src),$(rust_src),$(BASENAME)))
	$(eval RUST_FILE := $(if $(DIR),$(DIR),.)rust/$(RUST_SRC_NAME).rs)
	$(eval STUB_FILE := $(if $(DIR),$(DIR),.)$(BASENAME).c)
	$(eval RUST_LIB := $(if $(DIR),$(DIR),.).librust_$(BASENAME).a)

	@if grep -Eq "^\s*//.*#\[no_mangle\]" $(RUST_FILE); then \
    echo "ERROR: Found #[no_mangle] only inside comments in $(RUST_FILE)."; \
    exit 1; \
	fi

	@if ! grep -Eq "^\s*#\[no_mangle\].*" $(RUST_FILE); then \
		echo "ERROR: $(RUST_FILE) must use #[no_mangle]."; \
		exit 1; \
	fi

	cd $(if $(DIR),$(DIR),.) && $(RUSTC) --crate-type staticlib --edition 2021 rust/$(RUST_SRC_NAME).rs -o .librust_$(BASENAME).a
	$(CC) $(STUB_FILE) $(RUST_LIB) $(RUST_LINK_LIBS) -o $@
	@rm -f $(RUST_LIB)
	@echo "Built: $@"
	@echo "Run with: ./$@"
endif
endif

.PHONY: clean
clean:
	rm -f hello *.o .librust_*.a
