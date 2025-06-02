#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

typedef enum {
  PRINT_AND_EXIT,
  CALL,
  RECV_FN1,
  RECV_FN2,
  IGNORED,
} callcode_t;

struct frame {
  unsigned long n;
  unsigned long fn1;
  callcode_t cont;
};

struct regset {
  struct frame *stack;
  size_t sp;
};

struct tailcall {
  callcode_t fn;
  unsigned long arg;
  callcode_t cont;
};

#if GLOBALS
# define RS
# define RSF /* regset formal param */
# define LREF
# define DREF
# define AREF
#else
# define RS struct regset *,
# define RSF struct regset *rs,
# define LREF rs->
# define DREF rs.
# define AREF &rs,
#endif

/*
 * In general continuations will take different types of input
 * arguments, but here we are lucky that all accept unsigned long
 * values.
 */
struct tailcall fib_call(RS unsigned long, callcode_t);
struct tailcall fib_recv_fn1(RS unsigned long, callcode_t);
struct tailcall fib_recv_fn2(RS unsigned long, callcode_t);
struct tailcall fib_abort(RS unsigned long, callcode_t);

static struct tailcall (*const dispatch[])(RS unsigned long, callcode_t) = {
  (void *) NULL,
  fib_call,
  fib_recv_fn1,
  fib_recv_fn2,
  fib_abort,
};

#if GLOBALS
struct frame stack[1000];
size_t sp;
#endif

struct tailcall fib_call(RSF unsigned long n, callcode_t cont) {
  struct tailcall c;
  LREF stack[LREF sp].n = n;
  LREF stack[LREF sp].cont = cont;
  ++LREF sp;
  if (n < 2) {
    --LREF sp;
    c.fn = cont;
    c.arg = 1;
    c.cont = IGNORED;
  } else {
    c.fn = CALL;
    c.arg = n - 1;
    c.cont = RECV_FN1;
  }
  return c;
}

struct tailcall fib_recv_fn1(RSF unsigned long fn1, callcode_t cont) {
  struct tailcall c;
  (void) cont;
  LREF stack[LREF sp - 1].fn1 = fn1;
  c.fn = CALL;
  c.arg = LREF stack[LREF sp - 1].n - 2;
  c.cont = RECV_FN2;
  return c;
}

struct tailcall fib_recv_fn2(RSF unsigned long fn2, callcode_t cont) {
  struct tailcall c;
  (void) cont;
  unsigned long rv = LREF stack[LREF sp - 1].fn1 + fn2;
  c.fn = LREF stack[LREF sp - 1].cont;
  c.arg = rv;
  c.cont = IGNORED;
  --LREF sp;
  return c;
}

struct tailcall fib_abort(RSF unsigned long arg, callcode_t cont) {
#if !GLOBALS
  (void) rs;
#endif
  (void) arg; (void) cont;
  abort();
}

unsigned long fib_iter(unsigned long n) {
#if !GLOBALS
  struct frame stack[n];
  struct regset rs;
#endif
  struct tailcall c;
  struct tailcall (*fn)(RS unsigned long, callcode_t);

#if !GLOBALS
  rs.stack = stack;
#endif
  DREF sp = 0;

  c.fn = CALL;
  c.arg = n;
  c.cont = PRINT_AND_EXIT;

  while (c.fn != PRINT_AND_EXIT) {
    fn = dispatch[c.fn];
    c = (*fn)(AREF c.arg, c.cont);
  }

  return c.arg;
}

int main(int ac, char **av) {
  unsigned long n;

  (void) ac;  /* ARGSUSED */
  while (*++av != NULL) {
    n = strtoul(*av, NULL, 0);
#if GLOBALS
    if (n > sizeof stack/sizeof stack[0]) {
      fprintf(stderr, "fib(%lu) would overflow stack\n", n);
      continue;
    }
#endif
    printf("fib(%lu) = %lu\n", n, fib_iter(n));
  }
  return 0;
}



