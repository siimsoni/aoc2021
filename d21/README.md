# Day 21: Dirac Dice

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Part 1

Part 1 was very straightforward to solve.

## Part 2

In part 2 the resulting number was in trillions; so a brute force approach 
wouldn't work here. However, it was possible to break down the work into 
cacheable units.

---
* parse: 232 ns
* part 1: 608 ns
* part 2: 115 ms
