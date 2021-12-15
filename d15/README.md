# Day 15: Chiton

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Part 1

Ooh, a shortest path algorithm. I'll try to come up with one from scratch, 
otherwise where's the fun, right?

So there's a huge amount of possible paths, and my first goal is to establish 
some rules when a path should be excluded.

To do that, I will create a matrix that contains shortest straight path for every
element. I'll start from the end, iterate up/left, then go diagonally to start point,
rinse and repeat until we are at the beginning.

Once the shortest straight path cost is calculated, that will be the worst case 
scenario. If we want to justify a go-around, then the best possible outcome will have 
to be cheaper than taking the worst-case path.

However, it wasn't necessary to implement go-arounds here - it looked like there
were no go-arounds in this exercise, because the worst-case was correct
answer to today's challenge.

## Part 2

Had to increase the map size by 5 times. The shortcut solution from part 1 worked 
again for the sample, but eventually produced in incorrect result on actual input data.

My first approach to map the values from original array also proved to be quite a 
disaster... So I had to rewrite that. I decided to recalculate lowest neighbors, until
none of the values became any lower.

I thought perhaps this would not solve the issue, but when I started outputting the 
number of changes, I noticed it was getting lower, and after a few minutes 
I got the result.

When benchmarking, I was surprised that the result was actually quite fast  -
when I turned optimizations on, I could confirm that it indeed
ran in two seconds.

Local benchmark results:

* parse: 17 μs 
* part 1: 35 μs (correct result with incorrect algorithm)
* part 2: 1.2s 
 
