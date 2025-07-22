fn main() {
    let mut counter = RecentCounter::new();
    let a = counter.ping(1);
    let b = counter.ping(100);
    let c = counter.ping(3001);
    let d = counter.ping(3002);
    println!("{}, {}, {}, {}", a, b, c, d);
}

struct RecentCounter {
    pings: Vec<i32>,
    start: usize,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {

    fn new() -> Self {
        RecentCounter { pings: vec![], start: 0}
    }
    
    fn ping(&mut self, t: i32) -> i32 {
        self.pings.push(t);

        let out = self.pings[self.start..].iter().enumerate().find(|(_, v)| {
            **v >= t - 3000 && **v <= t
        }).unwrap_or((0, &0)).0;

        self.start += out;

        (self.pings.len() - self.start) as i32
    }
}