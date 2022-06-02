# Run like "make DEBUG=1 ..." to turn on debugging symbols
DEBUG ?= 0
ifeq ($(DEBUG), 1)
    CFLAGS=-g3 -ggdb -DDEBUG -O0
else
	# -DNDEBUG disables assert. Not cool.
    # CFLAGS=-DNDEBUG
	CFLAGS=
endif

CC=gcc
CFLAGS+=-Wall
CFLAGS+=-Wextra
CFLAGS+=-pedantic
CFLAGS+=-Wno-overlength-strings # We have long strings sometimes

.PHONE: run
run: main
	./main

main: main.c
	$(CC) $(CFLAGS) $(LDFLAGS) -o $@ $^

.PHONE: test
test: main
	./main test
