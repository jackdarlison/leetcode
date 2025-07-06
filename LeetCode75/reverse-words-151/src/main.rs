fn main() {

    let s = String::from("the sky is blue");
    let output = String::from("blue is sky the");
    assert!(reverse_words(s) == output);
}

pub fn reverse_words(s: String) -> String {
    let mut words: Vec<&str> = s.split(" ").filter(|s| s.len() > 0).collect();
    words.reverse();
    words.join(" ")
}
