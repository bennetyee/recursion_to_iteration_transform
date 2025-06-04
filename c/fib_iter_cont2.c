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

unsigned long fib_iter(unsigned long n) {
  struct frame stack[n+2];
  size_t sp;
  callcode_t fn;
  unsigned long arg;
  callcode_t cont;

  sp = 0;

  fn = CALL;
  arg = n;
  cont = PRINT_AND_EXIT;

  while (fn != PRINT_AND_EXIT) {
    switch (fn) {
      case CALL:
        stack[sp].n = arg;
        stack[sp].cont = cont;
        ++sp;
        if (stack[sp - 1].n < 2) {
          fn = cont;
          arg = stack[sp - 1].n;
          cont = IGNORED;
          --sp;
        } else {
          fn = CALL;
          arg = stack[sp - 1].n - 1;
          cont = RECV_FN1;
          /* should optimize to a direct jump */
        }
        break;
      case RECV_FN1:
        stack[sp - 1].fn1 = arg;
        fn = CALL;
        arg = stack[sp - 1].n - 2;
        cont = RECV_FN2;
        break;
      case RECV_FN2:
        fn = stack[sp - 1].cont;
        arg = stack[sp - 1].fn1 + arg;
        cont = IGNORED;
        --sp;
        break;
      default:
        abort();
    }
  }

  return arg;
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



