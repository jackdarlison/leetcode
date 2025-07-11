fn main() {
    let out = longest_subarray(vec![1,1,1]);

    assert!(out == 2);
}

fn longest_subarray(nums: Vec<i32>) -> i32 {

    let (mut i, mut j, mut zeroes) = (0, 0, 0);

    while j < nums.len() {
        if nums[j] == 0 {
            zeroes += 1;
        };

        if zeroes > 1 {
           if nums[i] == 0 {
            zeroes -= 1;
           } 
            i += 1;
        }

        j += 1;
    }

    (j - i) as i32 - 1
}