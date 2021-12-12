# Day 12: Passage Pathing

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

Kept track of paths with hash map. Created a data structure to calculate the
hash from, such that "large caves" would always create an unique hash.

This didn't work out elegantly, for several reasons:

1. I created hash from identifier, but my identifier was a byte array. So when
I added an unique part to it, it also had to be a byte array, but there's no
way to convert integer to byte array without extra libraries, which I guess
is due to differences in endianness on various platforms.

2. If I wanted to use sorting (e.g. BTreeMap), I would have needed also
comparison functions, in which case I would have had to store both 
an identifier and an order (Sorting was not really needed to get the result, 
but it helps a ton with debugging).

### Part 2

Here I realized I could have simplified the approach, as I didn't need
to return the paths, and I just needed the count.

After re-reading the problem statement a few times (I missed the part about
small caves appearing twice, but only once), it wasn't a lot more effort 
to get the answer.

---

My brute froce approach is most likely extremely sub-optimal,
as part 2 took several seconds, so I will skip the benchmarks today.
