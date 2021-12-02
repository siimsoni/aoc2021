use crate::parser::{Stmt, StmtKind};

pub fn solve(input: &Vec<Stmt>) -> i64 {
    let mut input = input.to_vec();
    input.reverse();
    let coords = mv(input, [0, 0, 0]);
    (coords[0] as i64)
        .checked_mul(coords[1] as i64)
        .expect("overflow")
}

fn mv(mut input: Vec<Stmt>, mut coordinates: [i32; 3]) -> [i32; 3] {
    if let Some((keyword, value)) = input.pop() {
        match keyword {
            StmtKind::Down => coordinates[2] += value as i32,
            StmtKind::Up => coordinates[2] -= value as i32,
            StmtKind::Forward => {
                coordinates[0] += value as i32;
                coordinates[1] += coordinates[2] * value as i32;
            }
        }
        mv(input, coordinates)
    } else {
        coordinates
    }
}
