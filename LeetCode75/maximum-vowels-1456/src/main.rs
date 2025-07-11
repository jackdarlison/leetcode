fn main() {
    let out = max_vowels(String::from("abciiidef"), 3);
    assert!(out == 3);
}

fn max_vowels(s: String, k: i32) -> i32 {
    let is_vowel = |c: char| { "aeiouAEIOU".contains(c) };
    " ".repeat(k as usize).chars().chain(s.chars())
        .zip(s.chars())
        .fold((0, 0), |(max, mut cur), (c1, c2)| {
            if is_vowel(c1) { cur -= 1; }
            if is_vowel(c2) { cur += 1; }
            (max.max(cur), cur)
        }).0

    // too slow :/
    // let cs: Vec<char> = s.chars().into_iter().collect();
    // cs.windows(k as usize)
    //     .map(|w| w.into_iter().filter(|&&w| is_vowel(w)).count() as i32)
    //     .max().unwrap()
}
