fn main() {
    let mut input = vec![0,1,0,3,12];
    move_zeroes(&mut input);

    assert!(input == vec![1, 3, 12, 0, 0])
}

fn move_zeroes(nums: &mut Vec<i32>) {

    let mut zeroes= 0;
    let mut p = 0;
    
    for i in 0..nums.len() {
        if nums[i] == 0 {
            zeroes += 1;
            continue;
        }
        nums[p] = nums[i];
        p += 1;
    }

    for i in (nums.len() - zeroes)..nums.len() {
        nums[i] = 0;
    }
}