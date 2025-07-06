
fn main() {
    let flowerbed = vec![1,0,0,0,1];
    let n = 1;

    println!("{}", can_place_flowers(flowerbed, n));

    
}

fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut flowerbed = flowerbed;

    let x = (0..flowerbed.len()).filter(|&i| {
        if flowerbed[i] == 1 { return false }

        if (i == 0 || flowerbed[i - 1] == 0) && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0) {
            flowerbed[i] = 1;
            return true
        }

        false
    }).count();

    (n as usize) <= x
}