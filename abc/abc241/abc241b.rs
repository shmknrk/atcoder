use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        m: usize,
        va: [u64; n],
        vb: [u64; m],
    }
    let mut map = HashMap::with_capacity(n);
    for a in va {
        let cnt = map.entry(a).or_insert(0);
        *cnt += 1;
    }
    for b in vb {
        if let Some(cnt) = map.get_mut(&b) {
            if *cnt == 0 {
                println!("No");
                return;
            }
            *cnt -= 1;
        }
        else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
