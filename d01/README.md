# Day 1: Sonar Sweep

## Run

```
cat assets/input.txt | cargo run
```

## Bench

```
cat assets/input.txt | cargo bench
```

## Log

It's been a while since writing Rust and it shows.
Struggled with Rust package layout quite a bit, as I didn't remember about
separation of binary / library.

Not the fastest and certainly not the prettiest code, but it's a nice challenge.

Local benchmark results:

* parse: 26 μs
* part 1: 496 ns
* part 2: 2 μs
 