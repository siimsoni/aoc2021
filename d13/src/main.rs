use d13::{parser, part1, part2};
use std::io;

fn main() {
    let parsed = parser::parse(io::stdin().lock());
    println!("Part 1: {}", part1::solve(&parsed));

    let (bitset, width) = part2::solve(&parsed);
    println!("Part 2");
    let mut n = 0;
    let len = bitset.size();
    while n < len {
        for pos in n..n + width {
            print!("{}", if bitset[pos] { "*" } else { " " });
        }
        println!();
        n += width;
    }
}
