use std::collections::HashMap;

fn main() {
    let out = find_difference(vec![1, 1, 2, 3], vec![1, 3, 3, 4]);
    println!("{out:?}");
    assert!(out == vec![vec![2], vec![4]]);
}

fn find_difference(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map: HashMap<i32, (bool, bool)> = HashMap::new();

    nums1.iter().for_each(|v| {
        map.entry(*v).and_modify(|(l, _)| *l = true).or_insert((true, false));
    });

    nums2.iter().for_each(|v| {
        map.entry(*v).and_modify(|(_, r)| *r = true).or_insert((false, true));
    });

    map.into_iter().fold(vec![vec![], vec![]], |mut acc, (i, (l, r))| {
        if l && !r {
            acc[0].push(i);
        }
        if r && !l {
            acc[1].push(i);
        }
        acc
    })
}