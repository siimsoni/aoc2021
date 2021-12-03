pub fn solve(input: &Vec<usize>) -> usize {
    let mut input = input.to_vec();

    let mut freqs = [0; usize::BITS as usize];
    let len = input.len();
    let last = usize::BITS as usize - 1;
    while let Some(val) = input.pop() {
        for n in 0..=last {
            if (val & (1 << (last - n))) != 0 {
                freqs[n] += 1;
            }
        }
    }

    let mut offset = 0;
    for n in 0..last {
        if freqs[n] != 0 {
            offset = n;
            break;
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    let half = len / 2;
    for n in 0..=(last - offset) {
        if freqs[last - n] > half {
            gamma += 1 << n;
        } else if freqs[last - n] < half {
            epsilon += 1 << n;
        }
    }

    gamma * epsilon
}
