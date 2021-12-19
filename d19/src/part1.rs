use fxhash::{FxHashMap, FxHashSet};

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

    if let Some((result, used_idxs)) = combine_scanners(input, input[0].clone(), used_idxs) {
        if input.len() == used_idxs.len() {
            return Some(result.len());
        }
    }
    None
}

fn combine_scanners(
    input: &Vec<FxHashSet<[i32; 3]>>,
    scanner: FxHashSet<[i32; 3]>,
    mut used_idxs: FxHashSet<usize>,
) -> Option<(FxHashSet<[i32; 3]>, FxHashSet<usize>)> {
    if input.len() == used_idxs.len() {
        return Some((scanner, used_idxs));
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
                    used_idxs.insert(k);
                    if let Some((new_result, idxs)) =
                        combine_scanners(input, diffed, used_idxs.clone())
                    {
                        result.extend(new_result);
                        used_idxs.extend(idxs);
                        // technically this loop should not be continued when used indexes
                        // changes. it didn't cause any problems, so i guess there isn't a case
                        // where paths diverge and only one is correct
                    }
                }
            }
        }
    }
    Some((result, used_idxs))
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

#[cfg(test)]
mod tests {
    use crate::part1::transform;

    #[test]
    fn test_transformation() {
        assert_eq!(transform([2, 1, -3], [0, 1, 2, 1, 1, 1]), [2, 1, -3]);
        assert_eq!(transform([2, 1, -3], [0, 1, 2, -1, -1, -1]), [-2, -1, 3]);
        assert_eq!(transform([2, 1, -3], [1, 0, 2, 1, -1, 1]), [1, -2, -3]);
    }
}
