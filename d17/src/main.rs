use d17::{parser, part1, part2};
use std::io;

fn main() {
    let parsed = parser::parse(io::stdin().lock()).unwrap();
    println!("Part 1: {}", part1::solve(&parsed));
    println!("Part 2: {}", part2::solve(&parsed));
}
