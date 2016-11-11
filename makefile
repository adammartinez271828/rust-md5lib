BINDIR := bin

all:
	cargo build
	mkdir -p $(BINDIR)
	gcc src/md5.c -lm -Wall -L ./target/debug -lmd5lib -o bin/md5
