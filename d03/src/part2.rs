const LAST: usize = usize::BITS as usize - 1;

pub fn solve(input: &Vec<usize>) -> Option<usize> {
    let freqs = count_freqs(input.to_vec());

    let mut offset = 0;
    for n in 0..LAST {
        if freqs[n] != 0 {
            offset = n;
            break;
        }
    }

    let oxygen_generator_rating = oxygen_generator_rating(input.to_vec(), &freqs, offset);
    let co2_scrubber_rating = co2_scrubber_rating(input.to_vec(), &freqs, offset);

    oxygen_generator_rating.and_then(|o2| co2_scrubber_rating.and_then(|co2| Some(o2 * co2)))
}

fn oxygen_generator_rating(mut input: Vec<usize>, freqs: &[usize], offset: usize) -> Option<usize> {
    let len = input.len();
    let half_len = len / 2;
    input.retain(|val| {
        let val_in_position = (val & (1 << (LAST - offset))) != 0;
        let freq = freqs[offset];
        let expected = freq > half_len || (half_len * 2 == len) && freq == half_len;
        val_in_position == expected
    });
    match input.len() {
        1 => input.pop(),
        0 => None,
        _ => {
            let new_input = input.to_vec();
            return oxygen_generator_rating(input, &count_freqs(new_input), offset + 1);
        }
    }
}

fn co2_scrubber_rating(mut input: Vec<usize>, freqs: &[usize], offset: usize) -> Option<usize> {
    let len = input.len();
    let half_len = len / 2;
    input.retain(|val| {
        let val_in_position = (val & (1 << (LAST - offset))) != 0;
        let freq = freqs[offset];
        let expected = !(freq > half_len || (half_len * 2 == len) && freq == half_len);
        val_in_position == expected
    });
    match input.len() {
        1 => input.pop(),
        0 => None,
        _ => {
            let new_input = input.to_vec();
            return co2_scrubber_rating(input, &count_freqs(new_input), offset + 1);
        }
    }
}

fn count_freqs(mut values: Vec<usize>) -> Box<[usize]> {
    let mut freqs = [0; usize::BITS as usize];
    while let Some(val) = values.pop() {
        for n in 0..=LAST {
            if (val & (1 << (LAST - n))) != 0 {
                freqs[n] += 1;
            }
        }
    }
    Box::new(freqs)
}
