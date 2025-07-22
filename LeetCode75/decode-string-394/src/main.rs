fn main() {
    let out = decode_string(String::from("3[a2[c]]"));
    println!("{out}");
    assert!(&out == "accaccacc");
}

fn decode_string(s: String) -> String {
    let mut stack: Vec<char> = vec![];
    for c in s.chars() {
        match c {
            ']' => {
                let mut sub_stack = String::new();
                loop {
                    if let Some(c) = stack.last() {
                        if c.is_ascii_alphabetic() {
                            sub_stack.push(stack.pop().unwrap());
                            continue;
                        }
                    }
                    break;
                }
                // `pop_if` is unstable so isnt allowed on leetcode
                // while let Some(c) = stack.pop_if(|c: &mut char| c.is_ascii_alphabetic()) {
                //     sub_stack.push(c);
                // }

                // remove the '[' char, which is needed to delimit patterns such as 2[2[a]] 
                stack.pop();

                let mut mult = String::new();
                loop {
                    if let Some(c) = stack.last() {
                        if c.is_ascii_digit() {
                            mult.push(stack.pop().unwrap());
                            continue;
                        }
                    }
                    break;
                }
                // while let Some(n) = stack.pop_if(|c| c.is_ascii_digit()) {
                //    mult.push(n); 
                // }
                
                let mult = mult.chars().rev().collect::<String>().parse().unwrap();
                for _ in 0..mult {
                    for c in sub_stack.chars().rev() {
                        stack.push(c);
                    }
                }
            },
            _ => {stack.push(c);}
        }
    }
    stack.iter().collect()
}