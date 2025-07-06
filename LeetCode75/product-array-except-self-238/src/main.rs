fn main() {

    let input = vec![1, 2, 3, 4];
    let expected = vec![24, 12, 8, 6];
    let output = product_except_self(input);

    println!("{:?}", output);

    assert!(expected == output);

}

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();

    let mut out = vec![1;n];
    let mut pre = 1;
    let mut post = 1;

    for i in 0..n {
        out[i] *= pre;
        pre *= nums[i];

        out[n-i-1] *= post;
        post *= nums[n-i-1];
    }

    out
}