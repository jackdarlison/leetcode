use std::collections::{HashMap, HashSet};

fn main() {
    let out = close_strings(String::from("aabbcc"), String::from("bbaacc"));
    assert!(out);
}

fn close_strings(word1: String, word2: String) -> bool {
    if word1.len() != word2.len() { return false; }

    let (mut map1, mut map2) = (HashMap::new(), HashMap::new());

    word1.chars().for_each(|c| {
        map1.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });
    word2.chars().for_each(|c| {
        map2.entry(c).and_modify(|v| *v += 1).or_insert(1);
    });

    
    if map1.keys().collect::<HashSet<_>>() != map2.keys().collect::<HashSet<_>>() {
        return false;
    }

    let mut values1: Vec<_> = map1.values().cloned().collect();
    let mut values2: Vec<_> = map2.values().cloned().collect();
    values1.sort_unstable();
    values2.sort_unstable();

    values1 == values2
    
}