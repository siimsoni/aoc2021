use bit_vec::BitVec;

type Version = usize;

pub fn solve(input: &Vec<BitVec>) -> usize {
    let mut result = 0;
    for transmission in input {
        match eat_packet(transmission, 0) {
            (version, _) => {
                result += version;
            }
        }
    }
    result
}

fn eat_packet(transmission: &BitVec, pos: usize) -> (Version, usize) {
    let (mut version, pos) = eat_packet_version(transmission, pos);
    let (packet_kind, mut pos) = eat_packet_kind(transmission, pos);
    match packet_kind {
        PacketKind::Literal => match eat_literal_value(transmission, pos) {
            (_, new_pos) => {
                pos = new_pos;
            }
        },
        PacketKind::Operator => match eat_operator(transmission, pos) {
            (subversion_sum, new_pos) => {
                pos = new_pos;
                version += subversion_sum;
            }
        },
    }
    (version, pos)
}

fn eat_packet_version(transmission: &BitVec, pos: usize) -> (Version, usize) {
    let mut version = 0;
    if transmission.get(pos).unwrap() {
        version |= 1 << 2;
    }
    if transmission.get(pos + 1).unwrap() {
        version |= 1 << 1;
    }
    if transmission.get(pos + 2).unwrap() {
        version |= 1;
    }
    (version, pos + 3)
}

enum PacketKind {
    Literal,
    Operator,
}

fn eat_packet_kind(transmission: &BitVec, pos: usize) -> (PacketKind, usize) {
    let mut kind = 0;
    if transmission.get(pos).unwrap() {
        kind |= 1 << 2;
    }
    if transmission.get(pos + 1).unwrap() {
        kind |= 1 << 1;
    }
    if transmission.get(pos + 2).unwrap() {
        kind |= 1;
    }
    (
        if kind == 4 {
            PacketKind::Literal
        } else {
            PacketKind::Operator
        },
        pos + 3,
    )
}

fn eat_literal_value(transmission: &BitVec, pos: usize) -> (usize, usize) {
    let (mut value, mut is_last, mut pos) = eat_literal_value_group(transmission, pos);
    while !is_last {
        value <<= 4;
        match eat_literal_value_group(transmission, pos) {
            (new_value, new_is_last, new_pos) => {
                value |= new_value;
                is_last = new_is_last;
                pos = new_pos;
            }
        }
    }
    (value, pos)
}

fn eat_literal_value_group(transmission: &BitVec, pos: usize) -> (usize, bool, usize) {
    let is_last = !transmission.get(pos).unwrap();
    (
        parse_fixed_int(transmission, pos + 1, pos + 4),
        is_last,
        pos + 5,
    )
}

fn eat_operator(transmission: &BitVec, pos: usize) -> (Version, usize) {
    let mut pos = pos;
    match eat_operator_kind(transmission, pos) {
        (operator_kind, new_pos) => {
            pos = new_pos;
            match operator_kind {
                OperatorKind::Count => eat_count_operator(transmission, pos),
                OperatorKind::Size => eat_size_operator(transmission, pos),
            }
        }
    }
}

fn eat_count_operator(transmission: &BitVec, pos: usize) -> (Version, usize) {
    let mut pos = pos;
    let count = parse_fixed_int(transmission, pos, pos + 10);
    pos += 11;
    let mut version = 0;
    for _ in 0..count {
        match eat_packet(transmission, pos) {
            (subversion_sum, new_pos) => {
                pos = new_pos;
                version += subversion_sum;
            }
        }
    }
    (version, pos)
}

fn eat_size_operator(transmission: &BitVec, pos: usize) -> (Version, usize) {
    let mut pos = pos;
    let limit = pos + 14 + parse_fixed_int(transmission, pos, pos + 14);
    pos += 15;
    let mut version = 0;
    while pos < limit {
        match eat_packet(transmission, pos) {
            (subversion_sum, new_pos) => {
                pos = new_pos;
                version += subversion_sum;
            }
        }
    }
    (version, pos)
}

enum OperatorKind {
    Size,
    Count,
}

fn eat_operator_kind(transmission: &BitVec, pos: usize) -> (OperatorKind, usize) {
    (
        if transmission.get(pos).unwrap() {
            OperatorKind::Count
        } else {
            OperatorKind::Size
        },
        pos + 1,
    )
}

fn parse_fixed_int(transmission: &BitVec, start: usize, end: usize) -> usize {
    let mut result = 0;
    if transmission.get(start).unwrap() {
        result |= 1;
    }
    for n in (start + 1)..=end {
        result <<= 1;
        if transmission.get(n).unwrap() {
            result |= 1;
        }
    }
    result
}
