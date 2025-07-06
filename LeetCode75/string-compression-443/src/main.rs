fn main() {
    let mut input = vec!['a','b','c'];
    let out = compress(&mut input);

    println!("{} - {:?}", out, input);

    assert!(out == 3)
}

fn compress(chars: &mut Vec<char>) -> i32 {
    let n = chars.len();

    let (mut x, mut y, mut out) = (0, 0, 0);

    while y < n {
        chars[out] = chars[x];
        out += 1;

        while y < n && chars[x] == chars[y] {
            y += 1;
        }

        if (y - x) > 1 { 
            for c in (y-x).to_string().chars() {
                chars[out] = c;
                out += 1;
            }
        }

        x = y;
    }

    out as _
}