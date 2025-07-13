fn main() {
    println!("Hello, world!");
}

pub fn largest_altitude(gain: Vec<i32>) -> i32 {
   gain.iter().fold((0, 0), |(max, cur), c| {
    (max.max(cur + c), cur + c)
   }).0
}
