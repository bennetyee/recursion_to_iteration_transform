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
iterative, explicit/manual call stack version.

The transformation was straightforward, and I expected that w/o
optimizations that the iterative version wouldn't be very high
performance.  What was surprising to me was the performance difference
between compilers -- try both the recursive and iterative versions
compiled with `gcc` and with `clang` to see for yourself!
