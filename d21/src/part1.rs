use std::cmp::min;

struct Die {
    val: usize,
    rolls: usize,
}

impl Die {
    fn new() -> Self {
        Die { val: 0, rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        self.rolls += 1;
        self.val += 1;
        let result = self.val;
        if self.val == 100 {
            self.val = 0;
        }
        result
    }

    fn get_rolls(self) -> usize {
        self.rolls
    }
}

pub fn solve(input: &(usize, usize)) -> usize {
    let mut p1_pos = input.0 - 1;
    let mut p2_pos = input.1 - 1;

    let mut p1_score = 0;
    let mut p2_score = 0;

    let mut die = Die::new();

    loop {
        p1_pos = (p1_pos + die.roll() + die.roll() + die.roll()) % 10;

        p1_score += p1_pos + 1;
        if p1_score >= 1000 {
            break;
        }

        p2_pos = (p2_pos + die.roll() + die.roll() + die.roll()) % 10;
        p2_score += p2_pos + 1;
        if p2_score >= 1000 {
            break;
        }
    }

    min(p1_score, p2_score) * die.get_rolls()
}
