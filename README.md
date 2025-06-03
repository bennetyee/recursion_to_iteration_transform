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
iterative, explicit/manual call stack version.  This investigation is
<em>not</em> about efficient algorithms for computing the Fibonacci
sequence -- we don't implement Binet's formula nor the matrix
exponentiation approaches, but rather only code transformation
techniques.

The implementation in `fib_iter.c` is the first attempt to transform
recursion to manual stack management.  The difference between
`fib_iter.c` and `fib_iter2.c` is to remove pointer usage in case the
compiler has trouble w/ alias analysis.  The implementations in
`fib_iter_cont.c` and `fib_iter_cont2.c` are a manual transformation
to continuation-passing style (CPS), first with functions that
returned their intended tail calls to a executor that would actually
make those tail calls in a loop, and second with the functions
in-lined to eliminate those function call overhead, resulting in code
that looks similar to `fib_iter.c`.

The transformations were straightforward, and I expected that w/o
optimizations that the iterative version wouldn't be very high
performance.  What was surprising to me was the performance difference
between compilers -- try both the various versions compiled with `gcc`
and with `clang` to see for yourself (`make CC=clang`)!  Play with
compiler optimization flags, e.g., `-O3`, `-Os`,
`-faggressive-loop-optimizations`, `-fexpensive-optimizations`,
`-funroll-loops`, etc for `gcc`, and `-O3`, `-Os`, or `-Ofast` for
`clang`.  Run the binaries with something like `time ./fib_rec $(seq 1
50)` -- modern CPUs adjust their clock frequencies and switch cores
based on thermal sensor data, so timing via just wall-clock execution
time can be a little tricky.

The files `fib_rec_inlined.c` and `fib_rec_inlined2.c` contains the
recursive implementation with one and two levels of recursive calls
manually inlined.  This explores whether `gcc`'s rather dramatic speed
for it's compilation of `fib_rec.c` over `clang` is due to more
aggressive inlining.  The generated executables are much faster than
any of the other approaches -- for both compilers -- and the
differences between compilers appear to be diminishing.

## Performance

Single execution, not averaged, so only good for "ballpark" comparisons.

`gcc` 13.3:
```
fib_rec: 
real	0m41.019s
user	0m40.994s
sys	0m0.005s
fib_iter: 
real	2m23.484s
user	2m23.466s
sys	0m0.002s
fib_iter2: 
real	3m24.828s
user	3m24.595s
sys	0m0.096s
fib_iter_cont: 
real	8m9.161s
user	8m8.595s
sys	0m0.306s
fib_iter_cont2: 
real	3m2.271s
user	3m2.125s
sys	0m0.058s
fib_rec_inlined: 
real	0m10.232s
user	0m10.207s
sys	0m0.016s
fib_rec_inlined2: 
real	0m0.059s
user	0m0.056s
sys	0m0.003s
```

`clang` 18.1.3
```
fib_rec: 
real	1m9.535s
user	1m9.494s
sys	0m0.013s
fib_iter: 
real	2m22.505s
user	2m22.456s
sys	0m0.025s
fib_iter2: 
real	3m53.955s
user	3m53.847s
sys	0m0.045s
fib_iter_cont: 
real	7m16.191s
user	7m16.021s
sys	0m0.057s
fib_iter_cont2: 
real	3m54.488s
user	3m54.437s
sys	0m0.014s
fib_rec_inlined: 
real	0m0.935s
user	0m0.932s
sys	0m0.002s
fib_rec_inlined2: 
real	0m0.068s
user	0m0.065s
sys	0m0.002s
```
