CC := clang
CFLAGS := -ggdb3 -gdwarf-4 -O0 -Qunused-arguments -std=c11 \
          -Wall -Werror -Wextra -Wno-gnu-folding-constant \
          -Wno-sign-compare -Wno-unused-parameter -Wno-unused-variable -Wshadow
RUSTC := rustc

all: recover

recover: recover.c rust/recover.rs
	$(RUSTC) --crate-type staticlib --edition 2021 rust/recover.rs -o .librecover.a
	$(CC) $(CFLAGS) recover.c .librecover.a -o recover -lpthread -ldl -lm

clean:
	rm -f recover .librecover.a *.jpg

.PHONY: all clean