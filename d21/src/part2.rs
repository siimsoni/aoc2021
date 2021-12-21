use std::cmp::max;
use std::collections::HashMap;

type Score = (usize, usize);
type Position = (usize, usize);

pub fn solve(input: &(usize, usize)) -> usize {
    let mut cache_1 = HashMap::new();
    let result = count_universes((0, 0), (input.0 - 1, input.1 - 1), &mut cache_1);
    max(result.0, result.1)
}

fn count_universes(
    score: Score,
    position: Position,
    cache: &mut HashMap<(Score, Position), (usize, usize)>,
) -> (usize, usize) {
    if let Some(result) = cache.get(&(score, position)) {
        return *result;
    }
    let mut p1_universe_count = 0;
    let mut p2_universe_count = 0;
    for p1_roll_first in 1..=3 {
        for p1_roll_second in 1..=3 {
            for p1_roll_third in 1..=3 {
                let p1_position =
                    (position.0 + p1_roll_first + p1_roll_second + p1_roll_third) % 10;
                let p1_score = score.0 + p1_position + 1;
                if p1_score >= 21 {
                    p1_universe_count += 1;
                } else {
                    for p2_roll_first in 1..=3 {
                        for p2_roll_second in 1..=3 {
                            for p2_roll_third in 1..=3 {
                                let p2_position =
                                    (position.1 + p2_roll_first + p2_roll_second + p2_roll_third)
                                        % 10;
                                let p2_score = score.1 + p2_position + 1;
                                if p2_score < 21 {
                                    match count_universes(
                                        (p1_score, p2_score),
                                        (p1_position, p2_position),
                                        cache,
                                    ) {
                                        (p1, p2) => {
                                            p1_universe_count += p1;
                                            p2_universe_count += p2;
                                        }
                                    }
                                } else {
                                    p2_universe_count += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cache.insert((score, position), (p1_universe_count, p2_universe_count));

    (p1_universe_count, p2_universe_count)
}
