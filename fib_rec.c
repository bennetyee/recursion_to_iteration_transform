#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

unsigned long fib_rec(unsigned long n) {
  if (n < 2) {
    return 1;
  }
  return fib_rec(n-1) + fib_rec(n-2);
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



