use std::collections::HashMap;

pub fn solve(parsed: &(Vec<usize>, Vec<Vec<(usize, usize, usize)>>)) -> Option<usize> {
    let (drawed_numbers, boards) = parsed;
    let mut combinations = Vec::new();

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
    }

    let mut i = 0;
    let mut result: u128 = 0;
    let mut total = 0;
    let mut solution = None;
    'outer: for number in drawed_numbers {
        total += number;
        result += 1 << number;
        if i >= 4 {
            for (combination, table) in &combinations {
                if combination == &(result & combination) {
                    let difference = (!result) & table;
                    let mut difference_sum: usize = 0;
                    for n in 0..u128::BITS {
                        if difference & (1 << n) != 0 {
                            difference_sum += n as usize;
                        }
                    }
                    solution = Some(difference_sum * number);
                    break 'outer;
                }
            }
        }
        i += 1;
    }
    solution
}
