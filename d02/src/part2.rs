use crate::parser::{Command, Keyword};

pub fn solve(input: &Vec<Command>) -> i64 {
    let mut input = input.to_vec();
    input.reverse();
    let coords = mv(input, [0,0,0]);
    (coords[0] as i64).checked_mul(coords[1] as i64).expect("overflow")
}

fn mv(mut input: Vec<Command>, mut coordinates: [i32;3]) -> [i32;3] {
    if let Some((keyword, value)) = input.pop() {
        match keyword {
            Keyword::Down => coordinates[2] += value as i32,
            Keyword::Up => coordinates[2] -= value as i32,
            Keyword::Forward => {
                coordinates[0] += value as i32;
                coordinates[1] += coordinates[2] * value as i32;
            }
        }
        mv(input, coordinates)
    } else {
        coordinates
    }
}
