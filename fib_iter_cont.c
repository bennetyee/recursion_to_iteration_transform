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

/*
 * In general continuations will take different types of input
 * arguments, but here we are lucky that all accept unsigned long
 * values.
 */
struct tailcall fib_call(struct regset *, unsigned long, callcode_t);
struct tailcall fib_recv_fn1(struct regset *, unsigned long, callcode_t);
struct tailcall fib_recv_fn2(struct regset *, unsigned long, callcode_t);
struct tailcall fib_abort(struct regset *, unsigned long, callcode_t);

static struct tailcall (*const dispatch[])(struct regset *, unsigned long, callcode_t) = {
  (void *) NULL,
  fib_call,
  fib_recv_fn1,
  fib_recv_fn2,
  fib_abort,
};

struct tailcall fib_call(struct regset *rs, unsigned long n, callcode_t cont) {
  struct tailcall c;
  rs->stack[rs->sp].n = n;
  rs->stack[rs->sp].cont = cont;
  ++rs->sp;
  if (n < 2) {
    --rs->sp;
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

struct tailcall fib_recv_fn1(struct regset *rs, unsigned long fn1, callcode_t cont) {
  struct tailcall c;
  (void) cont;
  rs->stack[rs->sp - 1].fn1 = fn1;
  c.fn = CALL;
  c.arg = rs->stack[rs->sp - 1].n - 2;
  c.cont = RECV_FN2;
  return c;
}

struct tailcall fib_recv_fn2(struct regset *rs, unsigned long fn2, callcode_t cont) {
  struct tailcall c;
  (void) cont;
  unsigned long rv = rs->stack[rs->sp - 1].fn1 + fn2;
  c.fn = rs->stack[rs->sp - 1].cont;
  c.arg = rv;
  c.cont = IGNORED;
  --rs->sp;
  return c;
}

struct tailcall fib_abort(struct regset *rs, unsigned long arg, callcode_t cont) {
  (void) rs;
  (void) arg;
  (void) cont;
  abort();
}

unsigned long fib_iter(unsigned long n) {
  struct frame stack[n];
  struct regset rs;
  struct tailcall c;
  struct tailcall (*fn)(struct regset *, unsigned long, callcode_t);

  rs.stack = stack;
  rs.sp = 0;

  c.fn = CALL;
  c.arg = n;
  c.cont = PRINT_AND_EXIT;

  while (c.fn != PRINT_AND_EXIT) {
    fn = dispatch[c.fn];
    c = (*fn)(&rs, c.arg, c.cont);
  }

  return c.arg;
}

int main(int ac, char **av) {
  unsigned long n;

  (void) ac;  /* ARGSUSED */
  while (*++av != NULL) {
    n = strtoul(*av, NULL, 0);
    printf("fib(%lu) = %lu\n", n, fib_iter(n));
  }
  return 0;
}



