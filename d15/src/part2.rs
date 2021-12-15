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
    worst_case = settle(worst_case, &matrix, width);

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


fn settle(totals: Vec<usize>, weights: &Vec<u8>, width: usize) -> Vec<usize> {
    let mut changes = 0;
    let mut result = totals.to_vec();
    let len = totals.len();
    for n in 0..totals.len() {
        let row = n / width;
        let col = n - (row * width);
        let weight = *weights.get(n).unwrap();
        let mut low_neighbor = usize::MAX;
        if col != 0 {
            low_neighbor = min(low_neighbor, *totals.get(n - 1).unwrap());
        }
        if col != width - 1 {
            low_neighbor = min(low_neighbor, *totals.get(n + 1).unwrap());
        }
        if row != 0 {
            low_neighbor = min(low_neighbor, *totals.get(n - width).unwrap());
        }
        if let Some(below) = totals.get(n+width) {
            low_neighbor = min(low_neighbor, *below);
        }
        let total_weight = result.get_mut(n).unwrap();

        let recalculated_weight = weight as usize + low_neighbor;

        if *total_weight > recalculated_weight {
            *total_weight = recalculated_weight;
            changes += 1;
        }
    }

    if changes > 0 {
       return settle(result, weights, width);
    }

    result
}
