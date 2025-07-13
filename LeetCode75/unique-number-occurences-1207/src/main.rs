use std::collections::{HashMap, HashSet};

fn main() {
    let out = unique_occurrences(vec![1,2,2,1,1,3]);
    assert!(out)
}

fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut occurances = HashMap::new();

    arr.iter().for_each(|i| {
        occurances.entry(i).and_modify(|v| *v += 1).or_insert(1);
    });
    
    let set: HashSet<i32> = HashSet::from_iter(occurances.values().cloned());

    set.len() == occurances.len() 
}