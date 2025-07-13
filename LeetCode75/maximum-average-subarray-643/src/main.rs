fn main() {

    let out = find_max_average(vec![1,12,-5,-6,50,3], 4);
    println!("{out}");

    assert!(out == 12.75);
}

fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let n = nums.len();

    let mut running_sum: i32 = (0..k).map(|i| nums[i as usize]).sum();
    let mut max: f64 = (running_sum as f64) / (k as f64);

    if n <= 1 { return max; }
    
    for i in 0..(n-(k as usize)) {
        running_sum -= nums[i];
        running_sum += nums[i+(k as usize)];

        max = f64::max(max, (running_sum as f64) / (k as f64));
    }

    max
}