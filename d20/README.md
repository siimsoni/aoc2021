# Day 20: Trench Map

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Part 1

Compared to previous days, this looks pretty straightforward.
We'll have an input image that increases in size on very iteration
of applying "enhancement".

I will represent the image as a function of bitset and width. It looks like
input image would be a square in this case, so the width could technically
be omitted and assumed to be square root of length.

Similar to some earlier challenges, this will require getting neighbors 
of an integer.

What was different, and what completely threw me off was the fact that the "outside"
pixels can change color - although the text did several times point out that the canvas
is infinite and all pixels must be updated. I was in the mindset that this is a super
easy problem, and became confident that I have a correct solution, so in that
way it was a bit humbling. Thankfully a colleague of mine helped me from pulling 
my hair out. :-)

## Part 2

More of the same...

---

Local benchmark results:

* parse: 77 μs
* part 1: 273 μs 
* part 2: 15 ms  
