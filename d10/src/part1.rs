pub fn solve(input: &Vec<Box<[u8]>>) -> usize {
    let mut result = 0;
    let mut stack = Vec::new();
    for line in input {
        stack.clear();
        'c: for c in line.iter() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'(' {
                        result += 3;
                        break 'c;
                    }
                }
                b']' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'[' {
                        result += 57;
                        break 'c;
                    }
                }
                b'}' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'{' {
                        result += 1197;
                        break 'c;
                    }
                }
                b'>' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'<' {
                        result += 25137;
                        break 'c;
                    }
                }
                _ => (),
            }
        }
    }

    result
}
