use std::cmp::{max, min};
use std::collections::HashSet;

pub fn solve(input: &(i32, i32, i32, i32)) -> usize {
    let (min_x, max_x, min_y, max_y) = input;
    let mut solutions = HashSet::new();
    for start_x in 0..=*max_x {
        for start_y in *min_y..=1000 {
            if max_of_x(start_x) < *min_x {
                continue;
            }
            let mut highest_y = 0;
            for n in probe_iterator(start_x, start_y) {
                if n.y < *min_y || n.x > *max_x {
                    break;
                }
                highest_y = max(n.y, highest_y);
                if n.x >= *min_x && n.x <= *max_x && n.y >= *min_y && n.y <= *max_y {
                    solutions.insert(Position {
                        x: start_x,
                        y: start_y,
                    });
                    break;
                }
            }
        }
    }
    solutions.len()
}

pub struct ProbeIterator {
    step: i32,
    start_x: i32,
    start_y: i32,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Iterator for ProbeIterator {
    type Item = Position;
    fn next(&mut self) -> Option<Self::Item> {
        let result = Position {
            x: sum_of_x(self.start_x, min(self.step, self.start_x)),
            y: sum_of_x(self.start_y, self.step),
        };
        self.step += 1;
        Some(result)
    }
}

pub fn probe_iterator(start_x: i32, start_y: i32) -> ProbeIterator {
    ProbeIterator {
        step: 0,
        start_x,
        start_y,
    }
}

pub fn sum_of_x(initial_velocity: i32, steps: i32) -> i32 {
    ((steps) * ((initial_velocity - steps + 1) + initial_velocity)) / 2
}

pub fn max_of_x(initial_velocity: i32) -> i32 {
    sum_of_x(initial_velocity, initial_velocity)
}
