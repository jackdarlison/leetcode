
fn main() {
    let input1 = String::from("ABABAB");
    let input2 = String::from("ABAB");

    let output = gcd_of_strings(input1, input2);
    println!("{}", output);
    assert!(output == "AB");
}

fn gcd_of_strings(str1: String, str2: String) -> String {
    if format!("{}{}", str1, str2) != format!("{}{}", str2, str1) {
        return String::from("");
    }
    
    let len = gcd(str1.len(), str2.len());
    str1[..len].to_owned()    
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}