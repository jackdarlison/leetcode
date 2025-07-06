fn main() {
    let input = vec![1,8,6,2,5,4,8,3,7];
    let output = max_area(input);
    println!("{output}");

    assert!(output == 49);
}

fn max_area(height: Vec<i32>) -> i32 {
    let mut i = 0;
    let mut j = height.len() - 1;
    let mut max = 0;
    
    while i != j {

        let new = std::cmp::min(height[i], height[j]) * (j as i32 - i as i32);

        if new >= max {
            max = new;
        }

        if height[i] <= height[j] {
            i += 1;
        } else {
            j -= 1;
        }
    }

    max
}
