pub fn solve(input: &(Vec<u8>, usize)) -> usize {
    let (matrix, width) = input;
    let mut matrix = matrix.to_vec().into_boxed_slice();
    let width = *width;
    let len = matrix.len();
    let mut result = 0;

    loop {
        matrix = increment(matrix);
        while let Some(flashes) = get_flashes(&matrix) {
            matrix = matrix.iter().map(|v| if *v > 9 { 0 } else { *v }).collect();
            matrix = do_flashes(matrix, flashes, width, len);
        }
        if matrix.iter().map(|v| *v as usize).sum::<usize>() == 100 {
            break;
        }
        result += 1;
    }

    result
}

fn increment(matrix: Box<[u8]>) -> Box<[u8]> {
    matrix.iter().map(|v| *v + 1).collect()
}

fn flash_increment(mut matrix: Box<[u8]>, pos: usize) -> Box<[u8]> {
    if matrix[pos] > 0 && matrix[pos] < 10 {
        matrix[pos] += 1;
    }
    matrix
}

fn get_flashes(matrix: &Box<[u8]>) -> Option<Box<[usize]>> {
    let flashes: Box<[usize]> = matrix
        .iter()
        .enumerate()
        .filter_map(|(k, v)| if *v > 9 { Some(k) } else { None })
        .collect();
    if flashes.len() > 0 {
        Some(flashes)
    } else {
        None
    }
}

fn do_flashes(mut matrix: Box<[u8]>, flashes: Box<[usize]>, width: usize, len: usize) -> Box<[u8]> {
    for pos in flashes.iter() {
        if let Some(a) = get_above(*pos, width) {
            matrix = flash_increment(matrix, a);
            if let Some(al) = get_left(a, width) {
                matrix = flash_increment(matrix, al);
            }
            if let Some(ar) = get_right(a, width) {
                matrix = flash_increment(matrix, ar);
            }
        }
        if let Some(l) = get_left(*pos, width) {
            matrix = flash_increment(matrix, l);
        }
        if let Some(r) = get_right(*pos, width) {
            matrix = flash_increment(matrix, r);
        }
        if let Some(b) = get_below(*pos, width, len) {
            matrix = flash_increment(matrix, b);
            if let Some(bl) = get_left(b, width) {
                matrix = flash_increment(matrix, bl);
            }
            if let Some(br) = get_right(b, width) {
                matrix = flash_increment(matrix, br);
            }
        }
    }
    matrix
}

fn get_above(pos: usize, width: usize) -> Option<usize> {
    if pos >= width {
        Some(pos - width)
    } else {
        None
    }
}

fn get_left(pos: usize, width: usize) -> Option<usize> {
    if pos % width != 0 {
        Some(pos - 1)
    } else {
        None
    }
}

fn get_right(pos: usize, width: usize) -> Option<usize> {
    if pos % width != (width - 1) {
        Some(pos + 1)
    } else {
        None
    }
}

fn get_below(pos: usize, width: usize, len: usize) -> Option<usize> {
    let below = pos + width;
    if below < len {
        Some(below)
    } else {
        None
    }
}
