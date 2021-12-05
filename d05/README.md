# Day 5: Hydrothermal Venture

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Log

### Part 1

Initially solved part 1 with B-tree approach.
This included nesting B-trees in several layers, and did not scale for the problem.


### Part 2

Rewrote to represent lines as a bitmap instead. Benchmark showed that this also made part 1 
a bit faster, so swapped the implementation.

Local benchmark results:

* parse: 34 μs
* part 1: 530 μs
* part 2: 2.1 ms
 
