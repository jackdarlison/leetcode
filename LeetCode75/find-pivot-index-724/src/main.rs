fn main() {
    let out = pivot_index(vec![1,7,3,6,5,6]);
    println!("{out}");
    assert!(out == 3);
}

fn pivot_index(nums: Vec<i32>) -> i32 {
    let sum: i32 = nums.iter().sum();
    let mut left_sum = 0;

    nums.into_iter().enumerate().find(|(_, n)| {
        if left_sum == (sum - left_sum) - *n {
            return true;
        }
        left_sum += *n;
        false
    }).map_or(-1, |(i, _)| i as i32)
}
