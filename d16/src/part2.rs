use bit_vec::BitVec;

pub fn solve(input: &Vec<BitVec>) -> usize {
    let mut result = 0;
    for transmission in input {
        match eat_packet(transmission, 0) {
            (packet_value, _) => {
                result += packet_value;
            }
        }
    }
    result
}

fn eat_packet(transmission: &BitVec, pos: usize) -> (usize, usize) {
    let pos = eat_packet_version(transmission, pos);
    let (packet_kind, pos) = eat_packet_kind(transmission, pos);
    match packet_kind {
        PacketKind::Sum => match eat_operator(transmission, pos) {
            (sub_results, new_pos) => (sub_results.iter().sum(), new_pos),
        },
        PacketKind::Product => match eat_operator(transmission, pos) {
            (sub_results, new_pos) => (sub_results.iter().product(), new_pos),
        },
        PacketKind::Max => match eat_operator(transmission, pos) {
            (sub_results, new_pos) => (*sub_results.iter().max().unwrap(), new_pos),
        },
        PacketKind::Min => match eat_operator(transmission, pos) {
            (sub_results, new_pos) => (*sub_results.iter().min().unwrap(), new_pos),
        },
        PacketKind::Literal => eat_literal_value(transmission, pos),
        PacketKind::Gt => match eat_operator(transmission, pos) {
            (sub_results, new_pos) => (
                if sub_results.get(0).unwrap() > sub_results.get(1).unwrap() {
                    1
                } else {
                    0
                },
                new_pos,
            ),
        },
        PacketKind::Lt => match eat_operator(transmission, pos) {
            (sub_results, new_pos) => (
                if sub_results.get(0).unwrap() < sub_results.get(1).unwrap() {
                    1
                } else {
                    0
                },
                new_pos,
            ),
        },
        PacketKind::Eq => match eat_operator(transmission, pos) {
            (sub_results, new_pos) => (
                if sub_results.get(0).unwrap() == sub_results.get(1).unwrap() {
                    1
                } else {
                    0
                },
                new_pos,
            ),
        },
        PacketKind::Unknown => (0, pos),
    }
}

fn eat_packet_version(_: &BitVec, pos: usize) -> usize {
    pos + 3
}

#[derive(Debug)]
enum PacketKind {
    Sum,
    Product,
    Min,
    Max,
    Literal,
    Gt,
    Lt,
    Eq,
    Unknown,
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
        match kind {
            0 => PacketKind::Sum,
            1 => PacketKind::Product,
            2 => PacketKind::Min,
            3 => PacketKind::Max,
            4 => PacketKind::Literal,
            5 => PacketKind::Gt,
            6 => PacketKind::Lt,
            7 => PacketKind::Eq,
            _ => PacketKind::Unknown,
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

fn eat_operator(transmission: &BitVec, pos: usize) -> (Vec<usize>, usize) {
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

fn eat_count_operator(transmission: &BitVec, pos: usize) -> (Vec<usize>, usize) {
    let mut pos = pos;
    let count = parse_fixed_int(transmission, pos, pos + 10);
    pos += 11;
    let mut results = Vec::new();
    for _ in 0..count {
        match eat_packet(transmission, pos) {
            (sub_result, new_pos) => {
                pos = new_pos;
                results.push(sub_result);
            }
        }
    }
    (results, pos)
}

fn eat_size_operator(transmission: &BitVec, pos: usize) -> (Vec<usize>, usize) {
    let mut pos = pos;
    let limit = pos + 14 + parse_fixed_int(transmission, pos, pos + 14);
    pos += 15;
    let mut results = Vec::new();
    while pos < limit {
        match eat_packet(transmission, pos) {
            (sub_result, new_pos) => {
                pos = new_pos;
                results.push(sub_result);
            }
        }
    }
    (results, pos)
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
