# Day 9: Smoke Basin

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Part 1

Had to find lowest points. 

Created an algorithm with bitmap that started from low values, and then went up.

At every step, marked all the neighboring coordinates for current values. Then the low points
were current, but unmarked values. Before incrementing the value, marked remainder of the current
values.

## Part 2

Here had to find "basins", or connected regions. Regions are split by highest value points, so
started by marking all highest value points.

Created basins using a recursive function, noted the size, and marked all values in the
basin (these could be then skipped).

Local benchmark results:

* parse: 16 μs 
* part 1: 171 μs 
* part 2: 361 μs 
 
