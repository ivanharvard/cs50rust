inheritance_test: inheritance_test.c rust/inheritance.rs
	@echo "Compiling Rust staticlib rust/inheritance.rs"
	$(RUSTC) --crate-type staticlib --edition 2021 rust/inheritance.rs -o .libinheritance_test.a
	@echo "Compiling C test harness inheritance_test.c"
	$(CC) $(CFLAGS) inheritance_test.c .libinheritance_test.a -o inheritance_test
	@test -f inheritance_test || { echo "Error: inheritance_test was not created"; exit 1; }
	@test -x inheritance_test || { echo "Error: inheritance_test is not executable"; exit 1; }
	@ls -la inheritance_test