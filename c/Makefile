SHELL=bash
# use bash's built-in time function

TARGETS:=fib_rec fib_iter fib_iter2 fib_iter_cont fib_iter_cont2 fib_rec_inlined fib_rec_inlined2

CC:=gcc
#CC:=clang

ifeq ($(CC),gcc)
 CFLAGS:=-Wall -Wextra -Werror -O3 -faggressive-loop-optimizations -fexpensive-optimizations -funroll-loops
else
 CFLAGS:=-Wall -Wextra -Werror -Ofast
endif

all: $(TARGETS)

clean:
	rm -f *~ $(TARGETS)

test:	all
	set -e; for p in $(TARGETS); do echo -n "$$p: "; time ./$$p $$(seq 0 50) > /tmp/fib.out.$$$$; cmp -s /tmp/fib.out.$$$$ golden.txt || (echo 'OUTPUT BAD' >&2; diff /tmp/fib.out.$$$$ golden.txt; exit 1); done; rm /tmp/fib.out.$$$$

.PHONY:	all clean test
