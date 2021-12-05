use crate::parser::{Coordinates, Segment};
use bitset_fixed::BitSet;
use std::cmp::max;

fn nearest_pow_2(val: usize) -> usize {
    let mut result = 0;
    while (1 << result) < val {
        result += 1;
    }
    1 << result
}

pub fn solve(parsed: &Vec<Segment>) -> usize {
    let mut width = 0;
    let mut height = 0;

    for segment in parsed {
        width = max(width, max(segment.start.x, segment.end.x));
        height = max(height, max(segment.start.y, segment.end.y));
    }

    width = nearest_pow_2(width);
    height = nearest_pow_2(height);

    let size = width * height;
    let mut bitset = BitSet::new(size);
    let mut result = BitSet::new(size);
    for segment in parsed {
        bitset = draw(bitset, segment, &mut result, width);
    }

    result.count_ones() as usize
}

fn draw(mut bitset: BitSet, segment: &Segment, result: &mut BitSet, width: usize) -> BitSet {
    let mut coordinates = segment.start.clone();
    let target = &segment.end;
    loop {
        let pos = (coordinates.y * width) + coordinates.x;
        if bitset[pos] {
            result.set(pos, true);
        }
        bitset.set(pos, true);
        if let Some(r) = approach(coordinates, target) {
            coordinates = r;
        } else {
            break;
        }
    }

    bitset
}

fn approach(mut current: Coordinates, target: &Coordinates) -> Option<Coordinates> {
    if current.x == target.x && current.y == target.y {
        return None;
    }

    if target.y > current.y {
        current.y += 1;
    } else if target.y < current.y {
        current.y -= 1;
    }

    if target.x > current.x {
        current.x += 1;
    } else if target.x < current.x {
        current.x -= 1;
    }

    Some(current)
}
