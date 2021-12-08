pub fn solve(input: &Vec<(Vec<u8>, Vec<u8>)>) -> usize {
    input.iter().fold(0, |acc, (_, output)| {
        acc + output
            .iter()
            .filter(|val| match val.count_ones() {
                2 | 3 | 4 | 7 => true,
                _ => false,
            })
            .count()
    })
}
