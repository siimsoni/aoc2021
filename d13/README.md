# Day 13: Transparent Origami

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

Here I was thinking about the order of operation. I realized that in addition to doing
transformation by each level, I could do the transformation per item as well, because
final position of each point was not affected by other points.

It would also be possible to pre-calculate canvas sizes, instead of per each coordinate
transformation, but I did not do that as I had already spent a good amount of time on this.

### Part 2

It was a struggle with off-by-one errors, and when I finally got the result it was rotated
by 180 degrees.

---
* parse: 24 μs
* part 1: 38 μs 
* part 2: 47 μs 
 
