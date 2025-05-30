# Transforming Recursive Computation to Iterative

Intrigued and motivated by

```
"I have always felt that the transformation from recursion to
iteration is one of the most fundamental concepts of computer science,
and that students should learn it at about the time they are learning
data structures. This topic is the subject of Chapter 8 in my
multivolume work; but it’s only by accident that the recursion chapter
wasn’t Chapter 3"
```

I thought I'd do a mechanical transformation of the naïve recursive
implementation of the Fibonacci computation (of course), to an
iterative, explicit/manual call stack version.  The difference between
`fib_iter.c` and `fib_iter2.c` is to remove pointer usage in case the
compiler has trouble w/ alias analysis.

The transformation was straightforward, and I expected that w/o
optimizations that the iterative version wouldn't be very high
performance.  What was surprising to me was the performance difference
between compilers -- try both the various versions compiled with `gcc`
and with `clang` to see for yourself!  Play with compiler optimization
flags, e.g., `-O3`, `-Os`, `-faggressive-loop-optimizations`,
`-fexpensive-optimizations`, `-funroll-loops`, etc for `gcc`, and
`-O3`, `-Os`, or `-Ofast` for `clang`.  Run the binaries with
something like `time ./fib_rec $(seq 1 50)` -- modern CPUs adjust
their clock frequencies and switch cores based on thermal sensor data,
so timing via just wall-clock execution time can be a little tricky.
