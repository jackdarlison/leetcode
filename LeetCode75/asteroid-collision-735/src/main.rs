fn main() {
    let out = asteroid_collision(vec![8, -8]);
    println!("{out:?}");
    assert!(out == vec![]);
}

fn asteroid_collision(asteroids: Vec<i32>) -> Vec<i32> {
    let mut stack = vec![];

    for v in asteroids {
        loop {
            println!("{stack:?}");
            if let Some(top) = stack.pop() {
                if v < 0 && top > 0 {
                    if top > v.abs() {
                        stack.push(top);
                        break;
                    } else if top == v.abs() {
                        break;
                    }
                } else {
                    stack.push(top);
                    stack.push(v);
                    break;
                }
            } else {
                stack.push(v);
                break;
            }
        }
    }
    
    stack
}