#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

unsigned long fib_rec(unsigned long n) {
  if (n < 2) {
    return 1;
  }
  return (
    {
      unsigned long nn = n-1;
      (nn < 2) ? 1 : (
        {
          unsigned long nnn = nn - 1;
          (nnn < 2) ? 1 : fib_rec(nnn-1) + fib_rec(nnn-2);
        }
      ) + (
        {
          unsigned long nnn = nn - 2;
          (nnn < 2) ? 1 : fib_rec(nnn-1) + fib_rec(nnn-2);
        }
      );
    }
  ) + (
    {
      unsigned long nn = n-2;
      (nn < 2) ? 1 : (
        {
          unsigned long nnn = nn - 1;
          (nnn < 2) ? 1 : fib_rec(nnn-1) + fib_rec(nnn-2);
        }
      ) + (
        {
          unsigned long nnn = nn - 2;
          (nnn < 2) ? 1 : fib_rec(nnn-1) + fib_rec(nnn-2);
        }
      );
    }
  );
}

int main(int ac, char **av) {
  unsigned long n;

  (void) ac;  /* ARGSUSED */
  while (*++av != NULL) {
    n = strtoul(*av, NULL, 0);
    printf("fib(%lu) = %lu\n", n, fib_rec(n));
  }
  return 0;
}



