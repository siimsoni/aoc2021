use std::collections::{HashMap, HashSet};
use std::slice::Iter;

pub fn solve(parsed: &(Vec<usize>, Vec<Vec<(usize, usize, usize)>>)) -> Option<usize> {
    let (drawed_numbers, boards) = parsed;
    let mut combinations = Vec::new();
    let mut tables = HashSet::new();

    for board in boards {
        let mut rows: HashMap<usize, u128> = HashMap::new();
        let mut cols: HashMap<usize, u128> = HashMap::new();
        let mut table: u128 = 0;
        for (val, row, col) in board {
            *(rows.entry(*row).or_insert(0)) += 1 << *val as u128;
            *(cols.entry(*col).or_insert(0)) += 1 << *val as u128;
            table += 1 << val;
        }
        for row in rows.values() {
            combinations.push((*row, table));
        }
        for col in cols.values() {
            combinations.push((*col, table));
        }
        tables.insert(table);
    }

    let mut i = 0;
    let mut result: u128 = 0;
    let mut total = 0;
    let mut solution = None;

    'outer: for number in drawed_numbers {
        total += number;
        result += 1 << number;

        if tables.len() == 1 {
            let matches = get_matches(result, (&combinations).into_iter());
            if matches.len() == 1 {
                solution = Some(difference_sum(result, tables.iter().sum()) * number);
                break 'outer;
            }
        } else if i >= 4 {
            let matches = get_matches(result, (&combinations).into_iter());
            if matches.len() > 0 {
                combinations.retain(|(_, table)| !matches.contains(table));
                tables = tables.difference(&matches).map(|v| *v).collect();
            }

        }
        i += 1;
    }
    solution
}

fn get_matches(result: u128, combinations: Iter<(u128, u128)>) -> HashSet<u128> {
    let mut matches = HashSet::new();
    for (combination, table) in combinations {
        if combination == &(result & combination) {
            matches.insert(*table);
        }
    }
    matches
}

fn difference_sum(result: u128, table: u128) -> usize {
    let difference = (!result) & table;
    let mut difference_sum: usize = 0;
    for n in 0..u128::BITS {
        if difference & (1 << n) != 0 {
            difference_sum += n as usize;
        }
    }
    difference_sum
}