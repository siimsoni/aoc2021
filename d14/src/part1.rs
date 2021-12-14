use crate::parser::Transformation;
use std::collections::HashMap;

pub fn solve(input: &(Vec<char>, Vec<Transformation>)) -> usize {
    let (chars, transformations) = input;

    let steps = 10;

    let mut result = Vec::new();
    let spread = 2 << (steps - 1);
    let capacity = (spread * chars.len()) - spread + 1;

    let transformations: HashMap<(char, char), char> = transformations
        .iter()
        .map(|((left, right), result)| ((*left, *right), *result))
        .collect();

    result.resize(capacity, ' ');
    let mut pos = 0;
    for c in chars {
        if let Some(val) = result.get_mut(pos) {
            *val = *c;
        }
        pos += spread;
    }

    result = transform(result, transformations, spread / 2);
    result.sort();

    let mut active = result[0];
    let mut count = 0;
    let mut min_count = usize::MAX;
    let mut max_count = 0;

    for c in result {
        if active != c {
            active = c;
            if count < min_count {
                min_count = count;
            }
            if count > max_count {
                max_count = count;
            }
            count = 1;
        } else {
            count += 1;
        }
    }

    max_count - min_count
}

fn transform(
    mut result: Vec<char>,
    transformations: HashMap<(char, char), char>,
    spread: usize,
) -> Vec<char> {
    let mut pos = 0;
    let mut left: char;
    let mut target: usize;
    let mut right = result[pos];
    while pos < (result.len() - 1) {
        left = right;
        pos += spread;
        target = pos;
        pos += spread;
        right = result[pos];
        if let Some(val) = result.get_mut(target) {
            if let Some(c) = transformations.get(&(left, right)) {
                *val = *c;
            }
        }
    }
    if spread > 1 {
        transform(result, transformations, spread / 2)
    } else {
        result
    }
}
