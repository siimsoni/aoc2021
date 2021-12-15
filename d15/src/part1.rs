use std::cmp::{max, min};

pub fn solve(input: &(Vec<u8>, usize)) -> usize {
    let (matrix, width) = input;
    let mut worst_case = Vec::with_capacity(matrix.len());
    worst_case.resize(matrix.len(), 0);

    let cols = *width;
    let rows = matrix.len() / width;
    let steps = max(cols, rows);

    worst_case = lowest_straight(steps - 1, rows - 1, cols - 1, worst_case, matrix, *width);
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
            *result.get_mut(row * width + col).unwrap() = weight;
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
