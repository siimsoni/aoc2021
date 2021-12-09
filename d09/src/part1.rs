use bitset_fixed::BitSet;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &(Vec<u8>, usize)) -> usize {
    let (matrix, width) = input;
    let len = matrix.len();
    let max = *matrix.iter().max().unwrap();
    let mut excluded = BitSet::new(matrix.len());
    let mut result = 0;
    for n in 0..max {
        let mut current = BitSet::new(matrix.len());
        for (pos, val) in matrix.iter().enumerate() {
            if *val == n {
                excluded = exclude_adjacent(excluded, pos, *width, len);
                current.set(pos, true);
            }
        }
        let mut diff = BitSet::new(matrix.len());
        diff.clone_from(&excluded);
        diff |= &current;
        diff ^= &excluded;
        result += diff.count_ones() as usize * (n as usize + 1);
        excluded |= &current;
    }
    result
}

pub fn exclude_adjacent(mut excluded: BitSet, pos: usize, width: usize, len: usize) -> BitSet {
    let row = pos / width;
    let col = pos - (row * width);
    if let Some(above) = pos.checked_sub(width) {
        excluded.set(above, true);
        if col != 0 {
            excluded.set(above - 1, true);
        }
        if col != (width - 1) {
            excluded.set(above + 1, true);
        }
    }
    if col != 0 {
        excluded.set(pos - 1, true);
    }
    if col != (width - 1) {
        excluded.set(pos + 1, true);
    }
    if row != (len / width) - 1 {
        let below = pos + width;
        excluded.set(below, true);
        if col != 0 {
            excluded.set(below - 1, true);
        }
        if col != (width - 1) {
            excluded.set(below + 1, true);
        }
    }
    excluded
}
