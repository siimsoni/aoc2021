use crate::parser::Transformation;
use std::collections::HashMap;

pub fn solve(input: &(Vec<char>, Vec<Transformation>)) -> usize {
    let (chars, transformations) = input;

    let transformations: HashMap<(char, char), char> = transformations
        .iter()
        .map(|((left, right), result)| ((*left, *right), *result))
        .collect();

    let mut cache = HashMap::new();
    let mut left;
    let mut right;

    let mut result = HashMap::new();

    let steps = 40;

    for n in 0..chars.len() - 1 {
        left = chars[n];
        right = chars[n + 1];
        cache = add(left, right, steps, &transformations, cache);
        result = merge(result, cache.get(&(left, right, steps)).unwrap());
    }

    *result.get_mut(&chars[0]).unwrap() -= 1;
    *result.get_mut(&chars[chars.len() - 1]).unwrap() -= 1;
    result = result.iter().map(|(c, f)| (*c, f/2)).collect();
    *result.get_mut(&chars[0]).unwrap() += 1;
    *result.get_mut(&chars[chars.len() - 1]).unwrap() += 1;

    result.values().max().unwrap() - result.values().min().unwrap()
}

fn add(
    left: char,
    right: char,
    step: usize,
    transformations: &HashMap<(char, char), char>,
    mut cache: HashMap<(char, char, usize), HashMap<char, usize>>,
) -> HashMap<(char, char, usize), HashMap<char, usize>> {
    if cache.contains_key(&(left, right, step)) {
    } else if step == 0 {
        let mut result = HashMap::new();
        if left == right {
            result.insert(left, 2);
        } else {
            result.insert(left, 1);
            result.insert(right, 1);
        }
        // println!("{} {} {} {:?}", left, right, step, result);
        cache.insert((left, right, step), result);
    } else {
        let middle = transformations.get(&(left, right)).unwrap();
        cache = add(left, *middle, step - 1, transformations, cache);
        cache = add(*middle, right, step - 1, transformations, cache);
        let mut result = cache.get(&(left, *middle, step - 1)).unwrap().clone();
        result = merge(result, cache.get(&(*middle, right, step - 1)).unwrap());

        // println!("---{} {} {} {:?}", left, right, step, result);
        cache.insert((left, right, step), result);
    }
    cache
}

fn merge(mut left_pairs: HashMap<char, usize>, right: &HashMap<char, usize>) -> HashMap<char, usize> {
    for (c, f) in right.iter() {
        *left_pairs.entry(*c).or_insert(0) += f;
    }
    left_pairs
}