fn main() {
    let input = vec![3, 6, 2, 2, 9];
    assert!(increasing_triplet(input));
}

fn increasing_triplet(nums: Vec<i32>) -> bool {
    if nums.len() < 3 { return false; }

    let mut smallest = i32::MAX;
    let mut sec_smallest = i32::MAX;

    for &num in nums.iter() {
        println!("{} - {} - {}", smallest, sec_smallest, num);
        if num <= smallest {
            smallest = num;
        } else if num <= sec_smallest {
            sec_smallest = num;
        } else {
            return true;
        }
    }

    false

    // questioned didnt specify contiguous :/
//    nums.iter().zip(nums.iter().skip(1)).zip(nums.iter().skip(2)).any(|((x, y), z)| {
//         x < y && y < z
//    })
}