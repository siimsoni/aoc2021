use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ElementKind {
    Separator(i32),
    Number(i32, i32),
}

pub fn parse<R>(mut reader: R) -> Box<[Vec<ElementKind>]>
where
    R: BufRead,
{
    let mut buffer = Vec::new();
    let mut page: [u8; 4096] = [0; 4096];
    while let Ok(page_len) = reader.read(&mut page) {
        if page_len == 0 {
            break;
        }
        buffer.extend_from_slice(&page[..page_len]);
    }

    let mut result = Vec::new();
    let mut iter = buffer.iter();
    let mut snailfish_number = Vec::new();
    let mut depth = 0;
    while let Some(byte) = iter.next() {
        match *byte {
            b'0' => snailfish_number.push(ElementKind::Number(0, depth)),
            b'1' => snailfish_number.push(ElementKind::Number(1, depth)),
            b'2' => snailfish_number.push(ElementKind::Number(2, depth)),
            b'3' => snailfish_number.push(ElementKind::Number(3, depth)),
            b'4' => snailfish_number.push(ElementKind::Number(4, depth)),
            b'5' => snailfish_number.push(ElementKind::Number(5, depth)),
            b'6' => snailfish_number.push(ElementKind::Number(6, depth)),
            b'7' => snailfish_number.push(ElementKind::Number(7, depth)),
            b'8' => snailfish_number.push(ElementKind::Number(8, depth)),
            b'9' => snailfish_number.push(ElementKind::Number(9, depth)),
            b',' => snailfish_number.push(ElementKind::Separator(depth)),
            b'[' => depth += 1,
            b']' => depth -= 1,
            b'\n' => {
                snailfish_number.reverse();
                result.push(snailfish_number.clone());
                snailfish_number.clear();
            }
            _ => (),
        }
    }
    if snailfish_number.len() > 0 {
        snailfish_number.reverse();
        result.push(snailfish_number);
    }
    result.into_boxed_slice()
}

pub fn parse_one(input: &str) -> Vec<ElementKind> {
    let lines = parse(BufReader::new(input.as_bytes()));
    lines[0].to_vec()
}

pub fn inverse(mut snailfish_number: Vec<ElementKind>) -> String {
    let mut inverse = String::new();
    let mut last_depth = 0;
    while let Some(value) = snailfish_number.pop() {
        match value {
            ElementKind::Separator(depth) => {
                while last_depth > depth {
                    inverse.push(']');
                    last_depth -= 1;
                }
                while last_depth < depth {
                    inverse.push('[');
                    last_depth += 1;
                }
                inverse.push(',');
            }
            ElementKind::Number(value, depth) => {
                while last_depth > depth {
                    inverse.push(']');
                    last_depth -= 1;
                }
                while last_depth < depth {
                    inverse.push('[');
                    last_depth += 1;
                }
                inverse.push_str(value.to_string().as_str());
            }
        }
    }
    while last_depth > 0 {
        inverse.push(']');
        last_depth -= 1;
    }
    inverse
}

#[cfg(test)]
mod tests {
    use crate::parser::{inverse, parse_one};
    use std::str;

    #[test]
    fn test_inverse_with_examples() {
        test_inverse("[1,2]");
        test_inverse("[[1,2],3]");
        test_inverse("[9,[8,7]]");
        test_inverse("[9,[8,7]]");
        test_inverse("[[1,9],[8,5]]");
        test_inverse("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]");
        test_inverse("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]");
        test_inverse("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]");
    }

    fn test_inverse(input: &str) {
        assert_eq!(
            str::from_utf8(input.as_bytes()).unwrap(),
            inverse(parse_one(input)).as_str()
        );
    }
}
