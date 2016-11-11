BINDIR := bin

all:
	mkdir -p $(BINDIR)
	gcc src/md5.c -lm -Wall -o bin/md5
