# Day 10: Dumbo Octopus

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

Hey, I could reuse my parser from day 9!

My first thought was to use bitmaps again, but that would enabled a 
situation where a matrix position could have multiple values, so it didn't make
sense - so using values 0-10 was sufficient.

I spent most of the time on the two-dimensional Carthesian plane position shifts,
but I kept wondering if there is a better way. Nothing came to my mind though.

### Part 2

Part 2 was basically solved already after part 1 was done.

* parse: 316 ns
* part 1: 108 μs 
* part 2: 337 μs 
 
