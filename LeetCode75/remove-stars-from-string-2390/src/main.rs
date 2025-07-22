fn main() {
    let out = remove_stars(String::from("leet**cod*e"));
    println!("{out}");
    assert!(out == "lecoe");
}

fn remove_stars(s: String) -> String {
    let mut char_stack = vec![];

    for c in s.chars() {
        if c == '*' {
            char_stack.pop();
        } else {
            char_stack.push(c);
        }
    }

    char_stack.iter().collect()
}
