pub fn solve(input: &Vec<Box<[u8]>>) -> usize {
    let mut result = Vec::new();
    let mut stack = Vec::new();
    'l: for line in input {
        stack.clear();
        for c in line.iter() {
            match c {
                b'(' | b'[' | b'{' | b'<' => stack.push(c),
                b')' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'(' {
                        continue 'l;
                    }
                }
                b']' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'[' {
                        continue 'l;
                    }
                }
                b'}' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'{' {
                        continue 'l;
                    }
                }
                b'>' => {
                    let delim = stack.pop();
                    if delim.is_none() || delim.unwrap() != &b'<' {
                        continue 'l;
                    }
                }
                _ => (),
            }
        }
        let mut score = 0;
        while let Some(c) = stack.pop() {
            score *= 5;
            score += match c {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => 0,
            }
        }
        result.push(score);
    }

    result.sort();
    result[result.len()/2]
}
