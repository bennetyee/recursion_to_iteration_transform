#include <stdio.h>
#include <stddef.h>
#include <stdlib.h>

struct frame {
  unsigned long n;
  unsigned long fn1, fn2;
  unsigned long *res;
};

void fib_iter_intern(struct frame *base, size_t ix) {
  while (ix > 0) {
    struct frame *fp = base + ix - 1;
    struct frame *nf = base + ix;
    if (fp->n < 2) {
      *fp->res = 1;
      --ix;
    } else if (fp->fn1 == 0) {
      nf->n = fp->n - 1;
      nf->res = &fp->fn1;
      nf->fn1 = 0;
      nf->fn2 = 0;
      ++ix;
    } else if (fp->fn2 == 0) {
      nf->n = fp->n - 2;
      nf->res = &fp->fn2;
      nf->fn1 = 0;
      nf->fn2 = 0;
      ++ix;
    } else {
      *fp->res = fp->fn1 + fp->fn2;
      --ix;
    }
  }
}

unsigned long fib_iter(unsigned long n) {
  struct frame *stack = calloc(n, sizeof *stack);
  unsigned long fn;

  if (stack == NULL) {
    perror("insufficient memory for stack");
    abort();
  }

  stack[0].n = n;
  stack[0].res = &fn;
  fib_iter_intern(stack, 1);
  free(stack);
  return fn;
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



