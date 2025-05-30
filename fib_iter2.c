#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

struct frame {
  unsigned long n;
  unsigned long fn1;
  int cont;
};

unsigned long fib_iter(unsigned long n) {
  struct frame stack[n];
  size_t ix;
  unsigned long res = 0;

  stack[0].n = n;
  stack[0].fn1 = 0;
  stack[0].cont = 0;
  ix = 1;
  while (ix > 0) {
    if (stack[ix-1].cont == 0) {
      if (stack[ix-1].n < 2) {
        res = 1;
        --ix;
      } else {
        stack[ix].n = stack[ix-1].n - 1;
        stack[ix].fn1 = 0;
        stack[ix].cont = 0;
        stack[ix-1].cont = 1;
        ++ix;
      }
    } else if (stack[ix-1].cont == 1) {
      stack[ix-1].fn1 = res;
      stack[ix].n = stack[ix-1].n - 2;
      stack[ix].fn1 = 0;
      stack[ix].cont = 0;
      stack[ix-1].cont = 2;
      ++ix;
    } else {
      res += stack[ix-1].fn1;
      --ix;
    }
  }

  return res;
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



