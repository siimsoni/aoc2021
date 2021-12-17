# Day 17: Trick Shot

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

Feels like there is a logic here, but not quite sure what it is.

Here I used Google to help me out with the mathematics, and I wound up with
finding some Youtube videos about triangular numbers, and eventually formula
for sum of the terms of arithmetic sequence. Deja-vu to last year?

### Part 2

Same, but had to count the number of results, instead of maximum Y-value in path.

Overall, I spent way too much time on an over-complicated parser,
and too little time on the exercise itself. E.g. I'd like to have a better 
understanding of boundaries, when to stop searching for new high Y values.

Local benchmark results:

* parse: 363 ns
* part 1: 8 ms
* part 2: 6 ms
