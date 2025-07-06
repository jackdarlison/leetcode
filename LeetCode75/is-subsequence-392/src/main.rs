fn main() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    assert!(is_subsequence(s, t));
}

fn is_subsequence(s: String, t: String) -> bool {
    let mut t_iter = t.chars();
    s.chars().all(|c| t_iter.any(|tc| tc == c))
}