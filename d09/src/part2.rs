use bitset_fixed::BitSet;
use std::collections::HashMap;
use std::collections::HashSet;

pub fn solve(input: &(Vec<u8>, usize)) -> usize {
    let (matrix, width) = input;
    let len = matrix.len();
    let max = *matrix.iter().max().unwrap();
    let mut excluded = BitSet::new(matrix.len());
    for (pos, val) in matrix.iter().enumerate() {
        if *val == max {
            excluded.set(pos, true);
        }
    }

    let mut basin_sizes = Vec::new();

    for n in 0..len {
        if !excluded[n] {
            let basin = map_basin(n, BitSet::new(matrix.len()), &matrix, &excluded, *width, len);
            basin_sizes.push(basin.count_ones());
            excluded |= &basin;
        }
    }

    basin_sizes.sort();

    (basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap() * basin_sizes.pop().unwrap()) as usize
}

pub fn map_basin(pos: usize, mut basin: BitSet, matrix: &Vec<u8>, excluded: &BitSet, width: usize, len: usize) -> BitSet {
    if basin[pos] || excluded[pos] {
        return basin;
    }
    basin.set(pos, true);
    let row = pos / width;
    let col = pos - (row * width);
    if row != 0 {
        basin = map_basin(pos - width, basin, matrix, excluded, width, len);
    }
    if col != 0 {
        basin = map_basin(pos - 1, basin, matrix, excluded, width, len);
    }
    if row != (len / width) - 1 {
        basin = map_basin(pos + width, basin, matrix, excluded, width, len);
    }
    if col != (width - 1) {
        basin = map_basin(pos + 1, basin, matrix, excluded, width, len);
    }
    basin
}
