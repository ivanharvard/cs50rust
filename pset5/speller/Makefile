CC = clang
CFLAGS = -ggdb3 -gdwarf-4 -O3 -Qunused-arguments -std=c11 -Wall -Werror -Wextra -Wno-gnu-folding-constant -Wno-sign-compare -Wno-unused-parameter -Wno-unused-variable -Wshadow

RUSTC = rustc
RUSTFLAGS = -O
RUST_SRC = rust/speller.rs
RUST_MOD = rust/cs50.rs
RUST_LIB = rust/libspeller_rs.a

speller: speller.o dictionary.o $(RUST_LIB)
	$(CC) $(CFLAGS) -o speller speller.o dictionary.o $(RUST_LIB) -lm -ldl -lpthread

speller.o: speller.c dictionary.h
	$(CC) $(CFLAGS) -c -o speller.o speller.c

dictionary.o: dictionary.c dictionary.h
	$(CC) $(CFLAGS) -c -o dictionary.o dictionary.c

$(RUST_LIB): $(RUST_SRC) $(RUST_MOD)
	$(RUSTC) --crate-type staticlib $(RUST_SRC) $(RUSTFLAGS) -o $(RUST_LIB)

clean:
	rm -f speller speller.o dictionary.o $(RUST_LIB)