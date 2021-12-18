use crate::parser::ElementKind;
use std::cmp::max;

pub fn solve(input: &Box<[Vec<ElementKind>]>) -> i32 {
    let mut magnitude = 0;

    for a in input.iter() {
        for b in input.iter() {
            let mut result = add_snailfish(a.to_vec(), b.to_vec());
            result = reduce(result, usize::MAX);
            magnitude = max(magnitude, snailfish_magnitude(result).unwrap());
        }
    }
    magnitude
}

pub fn add_snailfish(a: Vec<ElementKind>, b: Vec<ElementKind>) -> Vec<ElementKind> {
    let mut result = Vec::with_capacity(a.capacity() + b.capacity() + 2);
    result.append(
        &mut b
            .iter()
            .map(|v| match v {
                ElementKind::Number(val, depth) => ElementKind::Number(*val, *depth + 1),
                ElementKind::Separator(depth) => ElementKind::Separator(*depth + 1),
            })
            .collect(),
    );
    result.push(ElementKind::Separator(1));
    result.append(
        &mut a
            .iter()
            .map(|v| match v {
                ElementKind::Number(val, depth) => ElementKind::Number(*val, *depth + 1),
                ElementKind::Separator(depth) => ElementKind::Separator(*depth + 1),
            })
            .collect(),
    );
    result
}

pub fn snailfish_magnitude(mut snailfish_magnitude: Vec<ElementKind>) -> Option<i32> {
    let max_depth = *snailfish_magnitude
        .iter()
        .map(|snailfish_magnitude| match snailfish_magnitude {
            ElementKind::Number(_, depth) => depth,
            ElementKind::Separator(depth) => depth,
        })
        .max()
        .unwrap();
    for current_depth in (1..=max_depth).rev() {
        let elements_at_depth: Vec<usize> = snailfish_magnitude
            .iter()
            .enumerate()
            .rev()
            .filter_map(|(pos, value)| match value {
                ElementKind::Number(_, depth) => {
                    if *depth == current_depth {
                        Some(pos)
                    } else {
                        None
                    }
                }
                ElementKind::Separator(depth) => {
                    if *depth == current_depth {
                        Some(pos)
                    } else {
                        None
                    }
                }
            })
            .step_by(3)
            .collect();

        // panic!("{:?}", elements_at_denth);
        for pos in elements_at_depth {
            let magnitude = if let Some(left) = snailfish_magnitude.get(pos) {
                if let Some(right) = snailfish_magnitude.get(pos - 2) {
                    match right {
                        ElementKind::Number(right_val, _) => {
                            match left {
                                ElementKind::Number(left_val, _) => {
                                    left_val * 3 + right_val * 2
                                },
                                _ => 0
                            }
                        },
                        _ => 0
                    }
                } else { 0 }
            } else { 0 };
            snailfish_magnitude.splice(pos-2..=pos, [ElementKind::Number(magnitude, current_depth - 1)]);
        }
    }


    snailfish_magnitude.get(0).and_then(|value| match value {
        ElementKind::Number(val, _) => Some(*val),
        _ => None
    })
}

pub fn reduce(mut snailfish_number: Vec<ElementKind>, limit: usize) -> Vec<ElementKind> {
    if limit == 0 {
        return snailfish_number;
    }

    let mut left = Vec::with_capacity(snailfish_number.capacity());
    let mut left_element = None;
    while let Some(value) = snailfish_number.pop() {
        if let ElementKind::Number(_, depth) = value {
            if depth > 4 {
                left_element = Some(value);
                break;
            }
        }
        left.push(value);
    }
    left.reverse();
    if let Some(element) = left_element {
        let exploded = explode(
            [
                element,
                snailfish_number.pop().unwrap(),
                snailfish_number.pop().unwrap(),
            ],
            &mut left,
            &mut snailfish_number,
        )
        .unwrap();
        // return exploded;
        return reduce(exploded, limit - 1);
    }
    snailfish_number.append(&mut left);

    let mut result = snailfish_number.clone();

    if let Some((pos, element)) = snailfish_number
        .iter()
        .enumerate()
        .rev()
        .find_map(|(k, v)| match v {
            ElementKind::Number(val, _) => {
                if *val >= 10 {
                    Some((k, v))
                } else {
                    None
                }
            }
            _ => None,
        })
    {
        result.splice(pos..=pos, split(element.clone()).unwrap());
        // return snailfish_number;
        return reduce(result, limit - 1);
    }
    result
}

pub fn explode(
    pair: [ElementKind; 3],
    before: &mut Vec<ElementKind>,
    after: &mut Vec<ElementKind>,
) -> Result<Vec<ElementKind>, &'static str> {
    let mut result = Vec::new();
    result.append(after);
    let after_pos = result.iter().enumerate().rev().find_map(|(key, element)| {
        if let ElementKind::Number(_, _) = element {
            Some(key)
        } else {
            None
        }
    });
    if let [_, _, ElementKind::Number(right_val, depth)] = pair {
        result.push(ElementKind::Number(0, depth - 1));
        if let Some(i) = after_pos {
            if let ElementKind::Number(val, _) = result.get_mut(i).unwrap() {
                *val += right_val;
            }
        }
    }
    let before_pos = before.iter().enumerate().find_map(|(key, element)| {
        if let ElementKind::Number(_, _) = element {
            Some(key + result.len())
        } else {
            None
        }
    });
    result.append(before);
    if let [ElementKind::Number(left_val, _), _, _] = pair {
        if let Some(_) = before_pos {
            if let ElementKind::Number(val, _) = result.get_mut(before_pos.unwrap()).unwrap() {
                *val += left_val;
            }
        }
    }
    return Ok(result);
}

pub fn split(element: ElementKind) -> Result<[ElementKind; 3], &'static str> {
    match element {
        ElementKind::Number(val, depth) => {
            let half = val / 2;
            let depth = depth + 1;
            // inverse because using vec as stack
            Ok([
                ElementKind::Number(val - half, depth),
                ElementKind::Separator(depth),
                ElementKind::Number(half, depth),
            ])
        }
        _ => Err("separator cannot be split"),
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::{inverse, parse_one, ElementKind};
    use crate::part1::{add_snailfish, explode, magnitude, reduce, split};

    #[test]
    fn test_magnitude() {
        assert_eq!(
            magnitude(parse_one("[9,1]")),
            Some(29)
        );
        assert_eq!(
            magnitude(parse_one("[1,9]")),
            Some(21)
        );
        assert_eq!(
            magnitude(parse_one("[[1,2],[[3,4],5]]")),
            Some(143)
        );
        assert_eq!(
            magnitude(parse_one("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")),
            Some(1384)
        );
        assert_eq!(
            magnitude(parse_one("[[[[1,1],[2,2]],[3,3]],[4,4]]")),
            Some(445)
        );
        assert_eq!(
            magnitude(parse_one("[[[[3,0],[5,3]],[4,4]],[5,5]]")),
            Some(791)
        );
        assert_eq!(
            magnitude(parse_one("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")),
            Some(3488)
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            inverse(add_snailfish(parse_one("[1,2]"), parse_one("[[3,4],5]"))),
            "[[1,2],[[3,4],5]]"
        )
    }

    #[test]
    fn test_reduce() {
        assert_eq!(
            inverse(reduce(
                parse_one("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
                10
            )),
            "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
        )
    }

    #[test]
    fn test_explode() {
        assert_eq!(
            inverse(
                explode(
                    [
                        ElementKind::Number(9, 5),
                        ElementKind::Separator(5),
                        ElementKind::Number(8, 5),
                    ],
                    &mut Vec::new(),
                    &mut parse_one("[[[[,1],2],3],4]")
                )
                .unwrap()
            ),
            "[[[[0,9],2],3],4]"
        );
        assert_eq!(
            inverse(
                explode(
                    [
                        ElementKind::Number(3, 5),
                        ElementKind::Separator(5),
                        ElementKind::Number(2, 5),
                    ],
                    &mut parse_one("[7,[6,[5,[4,]]]]"),
                    &mut Vec::new(),
                )
                .unwrap()
            ),
            "[7,[6,[5,[7,0]]]]"
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            inverse(split(ElementKind::Number(10, 4)).unwrap().into()),
            "[[[[[5,5]]]]]"
        );
        assert_eq!(
            inverse(split(ElementKind::Number(11, 4)).unwrap().into()),
            "[[[[[5,6]]]]]"
        );
    }
}
