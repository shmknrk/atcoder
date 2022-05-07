use proconio::{input, fastout};

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        vs: [String; n],
        vt: [String; m],
    }
    let mut j = 0;
    for i in 0..n {
        if vs[i] == vt[j] {
            println!("Yes");
            j += 1;
        }
        else {
            println!("No");
        }
    }
}
