use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        va: [u64; n],
    }
    println!("{}", va.into_iter().collect::<HashSet<u64>>().len());
}
