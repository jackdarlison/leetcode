fn main() {
    let out = longest_ones(vec![0,0,1,1,0,0,1,1,1,0,1,1,0,0,0,1,1,1,1], 3);
    assert!(out == 10);
}

fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
    let (mut i, mut j, mut zeroes) = (0, 0, 0);

    while j < nums.len() {
        if nums[j] == 0 {
            zeroes += 1;
        }

        if zeroes > k {
            if nums[i] == 0 {
                zeroes -= 1;
            }
            i += 1;
        }

        j += 1;
    }

    (j - i) as i32
}
