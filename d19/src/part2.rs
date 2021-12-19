use fxhash::{FxHashMap, FxHashSet};
use std::cmp::max;

static TRANSFORM_MATRIX: [[i32; 6]; 24] = [
    [0, 1, 2, 1, 1, 1],
    [0, 2, 1, 1, 1, -1],
    [0, 1, 2, 1, -1, -1],
    [0, 2, 1, 1, -1, 1],
    [2, 1, 0, 1, 1, -1],
    [2, 0, 1, 1, -1, -1],
    [2, 1, 0, 1, -1, 1],
    [2, 0, 1, 1, 1, 1],
    [0, 1, 2, -1, 1, -1],
    [0, 2, 1, -1, -1, -1],
    [0, 1, 2, -1, -1, 1],
    [0, 2, 1, -1, 1, 1],
    [2, 1, 0, -1, 1, 1],
    [2, 0, 1, -1, 1, -1],
    [2, 1, 0, -1, -1, -1],
    [2, 0, 1, -1, -1, 1],
    [1, 2, 0, -1, 1, -1],
    [1, 0, 2, -1, -1, -1],
    [1, 2, 0, -1, -1, 1],
    [1, 0, 2, -1, 1, 1],
    [1, 2, 0, 1, 1, 1],
    [1, 0, 2, 1, 1, -1],
    [1, 2, 0, 1, -1, -1],
    [1, 0, 2, 1, -1, 1],
];

pub fn solve(input: &Vec<FxHashSet<[i32; 3]>>) -> Option<usize> {
    let mut used_idxs = FxHashSet::default();
    used_idxs.insert(0);
    let mut scanner_coords = FxHashSet::default();
    scanner_coords.insert([0, 0, 0]);

    if let Some((_, used_idxs, scanner_coords)) =
        combine_scanners(input, input[0].clone(), used_idxs, scanner_coords)
    {
        if input.len() == used_idxs.len() {
            let mut max_distance = 0;
            for a in &scanner_coords {
                for b in &scanner_coords {
                    max_distance = max(
                        (b[0] - a[0]).abs() + (b[1] - a[1]).abs() + (b[2] - a[2]).abs(),
                        max_distance,
                    );
                }
            }
            return Some(max_distance as usize);
        }
    }
    None
}

fn combine_scanners(
    input: &Vec<FxHashSet<[i32; 3]>>,
    scanner: FxHashSet<[i32; 3]>,
    mut used_idxs: FxHashSet<usize>,
    mut scanner_coords: FxHashSet<[i32; 3]>,
) -> Option<(FxHashSet<[i32; 3]>, FxHashSet<usize>, FxHashSet<[i32; 3]>)> {
    if input.len() == used_idxs.len() {
        return Some((scanner, used_idxs, scanner_coords));
    }
    let mut result = scanner.clone();
    let scanners_to_check = input
        .iter()
        .enumerate()
        .filter(|(k, _)| !used_idxs.contains(&k))
        .collect::<Vec<(usize, &FxHashSet<[i32; 3]>)>>();

    for (k, other_scanner) in scanners_to_check {
        for transformation in TRANSFORM_MATRIX {
            let aligned_other_scanner = orient_scanner(other_scanner.clone(), transformation);
            let diff_result = get_diffs_for_scanner(&aligned_other_scanner, &scanner);
            for (diff, count) in diff_result {
                if count >= 12 {
                    let diffed = add_scanner(&aligned_other_scanner, diff);
                    scanner_coords.insert(diff);
                    used_idxs.insert(k);
                    if let Some((new_result, idxs, new_scanner_coords)) =
                        combine_scanners(input, diffed, used_idxs.clone(), scanner_coords.clone())
                    {
                        result.extend(new_result);
                        used_idxs.extend(idxs);
                        scanner_coords.extend(new_scanner_coords);
                    }
                }
            }
        }
    }
    Some((result, used_idxs, scanner_coords))
}

fn get_diffs_for_scanner(
    a: &FxHashSet<[i32; 3]>,
    b: &FxHashSet<[i32; 3]>,
) -> FxHashMap<[i32; 3], usize> {
    let mut result = FxHashMap::default();
    for point_a in a {
        for point_b in b {
            *result.entry(diff(*point_a, *point_b)).or_insert(0) += 1;
        }
    }
    result
        .iter()
        .filter_map(|(k, v)| if *v > 1 { Some((*k, *v)) } else { None })
        .collect()
}

fn diff(a: [i32; 3], b: [i32; 3]) -> [i32; 3] {
    [b[0] - a[0], b[1] - a[1], b[2] - a[2]]
}

fn add_scanner(scanner: &FxHashSet<[i32; 3]>, val: [i32; 3]) -> FxHashSet<[i32; 3]> {
    scanner
        .iter()
        .map(|v| [v[0] + val[0], v[1] + val[1], v[2] + val[2]])
        .collect()
}

fn orient_scanner(scanner: FxHashSet<[i32; 3]>, transformation: [i32; 6]) -> FxHashSet<[i32; 3]> {
    scanner
        .iter()
        .map(|v| transform(*v, transformation))
        .collect()
}

fn transform(coordinates: [i32; 3], transformation: [i32; 6]) -> [i32; 3] {
    [
        coordinates[transformation[0] as usize] * transformation[3],
        coordinates[transformation[1] as usize] * transformation[4],
        coordinates[transformation[2] as usize] * transformation[5],
    ]
}
