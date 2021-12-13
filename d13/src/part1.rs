use std::cmp::max;
use std::collections::HashSet;
use std::slice::Iter;

pub fn solve(input: &(Vec<(usize, usize)>, Vec<(char, usize)>)) -> usize {
    let (points, instructions) = input;
    let (width, height) = measure(points.iter());
    let mut result = HashSet::new();
    for (x, y) in points {
        let (res_x, res_y) = fold((*x, *y), (width, height), &instructions[..1]);
        result.insert((res_x.clone(), res_y.clone()));
    }
    result.len()
}

pub fn fold(
    (x, y): (usize, usize),
    (width, height): (usize, usize),
    instructions: &[(char, usize)],
) -> (usize, usize) {
    match instructions.get(0) {
        Some((c, pos)) => {
            let pos = *pos;
            match c {
                'x' => {
                    let width = if pos * 2 > width { pos } else { width - pos };
                    if x > pos {
                        fold((x - pos, y), (width, height), &instructions[1..])
                    } else {
                        fold((pos - x, y), (width, height), &instructions[1..])
                    }
                }
                'y' => {
                    let height = if pos * 2 > height { pos } else { height - pos };
                    if y > pos {
                        fold((x, y - pos), (width, height), &instructions[1..])
                    } else {
                        fold((x, pos - y), (width, height), &instructions[1..])
                    }
                }
                _ => (x, y),
            }
        }
        None => (x, y),
    }
}

pub fn measure(iter: Iter<(usize, usize)>) -> (usize, usize) {
    iter.fold((0, 0), |(width, height), (x, y)| {
        (max(width, *x), max(height, *y))
    })
}
