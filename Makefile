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

.PHONY:	all clean
