# Day 4: Giant Squid

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

Initially made several false assumptions about the data and algorithm, so had 
to start from scratch a couple of times.
Submitted wrong result on first try, because I didn't calculate sum of all
unmarked numbers properly. This took some time to identify, as I thought I had 
messed up finding the correct board part.

### Part 2

Had to find the last winning board in part 2, which was quite straightforward 
after solving part 1.

Local benchmark results:

* parse: 47 μs
* part 1: 144 μs
* part 2: 683 μs
 
