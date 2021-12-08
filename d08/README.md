# Day 8: Seven Segment Search

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Part 1

Wrote the parser to read segments into integers. Once the parser was working,
it was only a bit of bitwise magic to get the result.

## Part 2

Got lucky here that the order of elements did not become important. :-)

I found it was possible to determine all digits by two passes - first
by reading segments where the value is known by length, and then all the 
remaining.

Local benchmark results:

* parse: 81 μs 
* part 1: 600 ns
* part 2: 18 μs 
 
