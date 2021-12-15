use bitset_fixed::BitSet;
use std::cmp::{max, min};

pub fn solve(input: &(Vec<u8>, usize)) -> usize {
    let (matrix, width) = input;
    let (matrix, width) = multiply(matrix, *width, 5);

    let mut worst_case = Vec::with_capacity(matrix.len());
    worst_case.resize(matrix.len(), 0);

    let cols = width;
    let rows = matrix.len() / width;
    let steps = max(cols, rows);

    worst_case = lowest_straight(steps - 1, rows - 1, cols - 1, worst_case, &matrix, width);
    let mut queue = BitSet::new(worst_case.len());
    for n in 0..worst_case.len() {
        queue.set(n, true);
    }
    worst_case = settle(worst_case, &matrix, width, queue);
    worst_case[0] - matrix[0] as usize
}


fn lowest_straight(
    step: usize,
    row: usize,
    col: usize,
    mut result: Vec<usize>,
    costs: &Vec<u8>,
    width: usize,
) -> Vec<usize> {
    for x in (0..=step).rev() {
        let mut weight = *costs.get(x * width + col).unwrap() as usize;
        if let Some(lowest_neighbor) = lowest_neighbor(x, col, &result, width) {
            weight += lowest_neighbor;
        }
        *result.get_mut(x * width + col).unwrap() = weight;
    }
    for y in (0..step).rev() {
        let mut weight = *costs.get(row * width + y).unwrap() as usize;
        if let Some(lowest_neighbor) = lowest_neighbor(row, y, &result, width) {
            weight += lowest_neighbor;
        }
        *result.get_mut(row * width + y).unwrap() = weight;
    }
    if step > 0 {
        let new_step = step - 1;
        lowest_straight(new_step, new_step, new_step, result, costs, width)
    } else {
        result
    }
}

fn lowest_neighbor(row: usize, col: usize, result: &Vec<usize>, width: usize) -> Option<usize> {
    let mut right = 0;
    if col != (width - 1) {
        right = *result.get(row * width + col + 1).unwrap();
    }
    let mut bottom = 0;
    if let Some(val) = result.get((row + 1) * width + col) {
        bottom = *val;
    }
    if right == 0 && bottom == 0 {
        return None;
    }
    if bottom == 0 {
        return Some(right);
    }
    if right == 0 {
        return Some(bottom);
    }
    return Some(min(bottom, right));
}

pub fn multiply(input: &Vec<u8>, orig_cols: usize, multiplier: usize) -> (Vec<u8>, usize) {
    let orig_rows = input.len() / orig_cols;
    let size = input.len() * multiplier.pow(2);
    let cols = orig_cols * multiplier;
    // let rows = orig_rows * multiplier;
    let mut result = Vec::with_capacity(size);
    result.resize(size, 0);
    for n in 0..size {
        let col = n % cols;
        let row = n / cols;
        let mapped_row = row % orig_rows;
        let mapped_col = col % orig_cols;
        let distance = (col / orig_cols) + (row / orig_rows);
        let mut weight = *input.get(mapped_row * orig_cols + mapped_col).unwrap();
        weight += distance as u8;
        weight -= 1;
        weight %= 9;
        weight += 1;
        *result.get_mut(n).unwrap() = weight;
    }
    (result, cols)
}


fn settle(totals: Vec<usize>, weights: &Vec<u8>, width: usize, queue: BitSet) -> Vec<usize> {
    let mut result = totals.to_vec();
    let mut new_queue = BitSet::new(queue.size());
    for n in 0..totals.len() {
        if !queue[n] {
            continue;
        }
        let row = n / width;
        let col = n - (row * width);
        let weight = *weights.get(n).unwrap();
        let mut neighbors = [None, None, None, None];
        if col != 0 {
            neighbors[0] = Some(n - 1);
        }
        if col != width - 1 {
            neighbors[1] = Some(n + 1);
        }
        if row != 0 {
            neighbors[2] = Some(n - width);
        }
        if n + width < result.len() {
            neighbors[3] = Some(n + width);
        }

        let low_neighbor = neighbors.iter().fold(usize::MAX, |acc, pos| {
            if let Some(pos) = pos {
                min(acc, *totals.get(*pos).unwrap())
            } else {
                acc
            }
        });

        let total_weight = result.get_mut(n).unwrap();

        let recalculated_weight = weight as usize + low_neighbor;

        if *total_weight > recalculated_weight {
            *total_weight = recalculated_weight;
            for pos in neighbors {
                if let Some(pos) = pos {
                    new_queue.set(pos, true);
                }
            }
        }
    }

    if new_queue.count_ones() > 0 {
       return settle(result, weights, width, new_queue);
    }

    result
}
