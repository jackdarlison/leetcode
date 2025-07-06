use std::collections::HashMap;

fn main() {
    let output = max_operations(vec![1, 2, 3, 4], 5);
    println!("{output}");
    assert!(output == 2);
}

fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    let mut map = HashMap::new();
    let mut ops = 0;

    for num in nums {

        if let Some(count) = map.get_mut(&(k - num)) {
            if *count > 0 {
                *count -= 1;
                ops += 1;
                continue;
            }
        }
        map.entry(num).and_modify(|x| *x += 1).or_insert(1);
    }

    ops
}
