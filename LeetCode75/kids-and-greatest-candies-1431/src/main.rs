fn main() {
    let candies = vec![2,3,5,1,3];
    let extra_candies = 3;

    let output = kids_with_candies(candies, extra_candies);
    println!("{:?}", output);
    assert!(output == vec![true,true,true,false,true]);
}


fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max = candies.iter().max().unwrap();
    candies.iter().map(|c| (c + extra_candies) >= *max).collect()
}