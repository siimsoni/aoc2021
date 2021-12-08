pub fn solve(input: &Vec<(Vec<u8>, Vec<u8>)>) -> usize {
    input.iter().fold(0, |acc, (samples, output)| {
        let map = decode(samples);
        let mut modifier = 10_usize.pow(output.len() as u32 - 1);
        let mut result = 0;
        for sequence in output {
            let mut n = 0;
            for m in map.iter() {
                if sequence == m {
                    result += n * modifier;
                    break;
                }
                n += 1;
            }
            modifier /= 10;
        }
        acc + result
    })
}

pub fn decode(samples: &Vec<u8>) -> Box<[u8; 10]> {
    let mut sequences = [0; 10];
    for sequence in samples {
        match sequence.count_ones() {
            2 => sequences[1] = *sequence,
            3 => sequences[7] = *sequence,
            4 => sequences[4] = *sequence,
            7 => sequences[8] = *sequence,
            _ => (),
        }
    }

    let a = (sequences[1] | sequences[7]) ^ sequences[1];
    let eg = (sequences[4] | a) ^ sequences[8];

    for sequence in samples {
        match sequence.count_ones() {
            5 => {
                if (sequence & sequences[1]).count_ones() == 1 {
                    if (sequence & eg).count_ones() == 2 {
                        sequences[2] = *sequence;
                    } else {
                        sequences[5] = *sequence;
                    }
                } else {
                    sequences[3] = *sequence;
                }
            }
            6 => {
                if (sequence & sequences[1]).count_ones() == 1 {
                    sequences[6] = *sequence;
                } else {
                    if (sequence & eg).count_ones() == 2 {
                        sequences[0] = *sequence;
                    } else {
                        sequences[9] = *sequence;
                    }
                }
            }
            _ => (),
        }
    }

    Box::new(sequences)
}
