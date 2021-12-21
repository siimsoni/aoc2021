use std::cmp::max;
use std::collections::HashMap;

type Score = (usize, usize);
type Position = (usize, usize);

pub fn solve(input: &(usize, usize)) -> usize {
    max(
        get_win_paths_p1((0, 0), (input.0 - 1, input.1 - 1), &mut HashMap::new()),
        get_win_paths_p2((0, 0), (input.0 - 1, input.1 - 1), &mut HashMap::new()),
    )
}

fn get_win_paths_p1(
    score: Score,
    position: Position,
    cache: &mut HashMap<(Score, Position), usize>,
) -> usize {
    if let Some(result) = cache.get(&(score, position)) {
        return *result;
    }
    let mut result = 0;
    for p1_roll_first in 1..=3 {
        for p1_roll_second in 1..=3 {
            for p1_roll_third in 1..=3 {
                let p1_position =
                    (position.0 + p1_roll_first + p1_roll_second + p1_roll_third) % 10;
                let p1_score = score.0 + p1_position + 1;
                if p1_score >= 21 {
                    result += 1;
                } else {
                    for p2_roll_first in 1..=3 {
                        for p2_roll_second in 1..=3 {
                            for p2_roll_third in 1..=3 {
                                let p2_position =
                                    (position.1 + p2_roll_first + p2_roll_second + p2_roll_third)
                                        % 10;
                                let p2_score = score.1 + p2_position + 1;
                                if p2_score < 21 {
                                    result += get_win_paths_p1(
                                        (p1_score, p2_score),
                                        (p1_position, p2_position),
                                        cache,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cache.insert((score, position), result);
    result
}

fn get_win_paths_p2(
    score: Score,
    position: Position,
    cache: &mut HashMap<(Score, Position), usize>,
) -> usize {
    if let Some(result) = cache.get(&(score, position)) {
        return *result;
    }
    let mut result = 0;
    for p1_roll_first in 1..=3 {
        for p1_roll_second in 1..=3 {
            for p1_roll_third in 1..=3 {
                let p1_position =
                    (position.0 + p1_roll_first + p1_roll_second + p1_roll_third) % 10;
                let p1_score = score.0 + p1_position + 1;
                if p1_score < 21 {
                    for p2_roll_first in 1..=3 {
                        for p2_roll_second in 1..=3 {
                            for p2_roll_third in 1..=3 {
                                let p2_position =
                                    (position.1 + p2_roll_first + p2_roll_second + p2_roll_third)
                                        % 10;
                                let p2_score = score.1 + p2_position + 1;
                                if p2_score >= 21 {
                                    result += 1;
                                } else {
                                    result += get_win_paths_p2(
                                        (p1_score, p2_score),
                                        (p1_position, p2_position),
                                        cache,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cache.insert((score, position), result);
    result
}
