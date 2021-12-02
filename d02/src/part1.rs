use crate::parser::{Stmt, StmtKind};

pub fn solve(input: &Vec<Stmt>) -> i64 {
    let mut input = input.to_vec();
    input.reverse();
    let coords = mv(input, [0,0]);
    coords[0] as i64 * coords[1] as i64
}

fn mv(mut input: Vec<Stmt>, mut coordinates: [i16;2]) -> [i16;2] {
    if let Some((keyword, value)) = input.pop() {
        match keyword {
            StmtKind::Up => coordinates[0] -= value,
            StmtKind::Down => coordinates[0] += value,
            StmtKind::Forward => coordinates[1] += value
        }
        mv(input, coordinates)
    } else {
        coordinates
    }
}
