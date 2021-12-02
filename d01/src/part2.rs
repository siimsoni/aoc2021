pub fn solve(input: &Vec<u16>) -> usize {
    let mut count = 0;
    let mut sum1: u16;
    let mut sum2: u16;
    for i in 0..input.len() - 3 {
        sum1 = input[i..i+3].iter().sum();
        sum2 = input[i+1..i+4].iter().sum();
        if sum1 < sum2 {
            count += 1;
        }
    }
    count
}
