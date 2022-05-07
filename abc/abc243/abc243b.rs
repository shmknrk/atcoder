use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        n: usize,
        va: [u64; n],
        vb: [u64; n],
    }
    let mut cnt1 = 0;
    for i in 0..n {
        if va[i] == vb[i] {
            cnt1 += 1;
        }
    }
    let sa: HashSet<u64> = va.into_iter().collect();
    let sb: HashSet<u64> = vb.into_iter().collect();
    let cnt2 = (&sa & &sb).len() - cnt1;
    println!("{}", cnt1);
    println!("{}", cnt2);
}
