fn main() {
    let s = String::from("IceCreAm");
    let output = String::from("AceCreIm");
    assert!(reverse_vowels(s) == output);
}

fn reverse_vowels(s: String) -> String {
    // this one is a cursed solution, iterating from both ends would be more elegant

    let mut parts: Vec<char> = s.chars().collect();

    let s_vowels: Vec<(usize, char)> = s.char_indices().filter(|(_, c)| vowels().contains(c)).collect();

    s_vowels.iter().enumerate().for_each(|(x, (i, _))| {
        parts[*i] = s_vowels[s_vowels.len() - (x + 1)].1;
    });

    parts.into_iter().collect()
}

fn vowels() -> &'static [char; 10] {
    &['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
}