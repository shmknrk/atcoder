use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        va: [u64; n],
    }
    let sa: HashSet<u64> = va.into_iter().collect();
    let mut i = 0;
    while sa.contains(&i) {
        i += 1;
    }
    println!("{}", i);
}
