pub fn solve(input: &Vec<u16>) -> usize {
    let mut input = input.to_vec();
    let mut count = 0;
    let mut next;
    if let Some(last) = input.pop() {
        next = last;
        while let Some(current) = input.pop() {
            if current < next {
                count += 1;
            }
            next = current;
        }
    }
    count
}
