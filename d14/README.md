# Day 14: Extended Polymerization

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Notes

### Part 1

First impression is that this looks very similar to day 6 problem.
However, I figured I could keep all the results in one byte string, 
and it seemed easier to implement, so I just solved it like that.

### Part 2

The solution from part 1 didn't scale to part 2 at all, as it would
just run out of memory.

However, as I observed in beginning of part 1, this was very similar to 
problem of day 6, which meant I could solve each pair independently and
cache the results.

Since this relies heavily on hashmaps, there would be notable performance
benefits in switching away from Rust's default hashing algorithm, which
is designed to be cryptographically secure, and not required in this
non-sensitive context (did not do).

---
* parse: 5 μs
* part 1: 893 μs 
* part 2: 1.3 ms 
 
