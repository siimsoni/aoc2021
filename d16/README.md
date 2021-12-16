# Day 16: Packet Decoder

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Part 1

So, if I had just applied the same approach as I have done with almost every parser up on this challenge,
I would of have saved a lot of time here. 

Unfortunately, I got seriously intimidated by the wall of text, and kept  trying to find a way to pre-determine
sub-packet boundaries, without evaluating the sub-packets. And of course, that did not work.

After getting my mind off from the problem for a bit, I realized my mistake pretty fast, but it was already
late into evening.

## Part 2

Hmm, ideally could be solved with AST I think. Some unit tests would be nice as well, but at this 
point I'll just wrap it up as fast as possible.

Local benchmark results:

* parse: 20 μs 
* part 1: 4 μs
* part 2: 7 μs 
