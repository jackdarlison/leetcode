use std::collections::HashMap;

fn main() {
    let out = equal_pairs(vec![vec![3,2,1], vec![1,7,6], vec![2,7,7]]);
    assert!(out == 1);
}

fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {

    let mut row_map: HashMap<Vec<i32>, usize> = HashMap::new(); 
    grid.iter().for_each(|row| {
        row_map.entry(row.clone()).and_modify(|v| *v += 1).or_insert(1);
    });

    let n = grid.len();
    let mut count = 0;

    for j in 0..n {
        let mut col = vec![];
        for i in 0..n {
            col.push(grid[i][j]);
        }
        count += row_map.get(&col).unwrap_or(&0);
    }
    
    count as i32
}
