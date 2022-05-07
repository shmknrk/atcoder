use proconio::{input, fastout};

#[fastout]
fn main() {
    input!{
        h: usize,
        w: usize,
        vva: [[u64; w]; h],
    }
    for j in 0..w {
        for i in 0..h - 1 {
            print!("{} ", vva[i][j]);
        }
        println!("{}", vva[h - 1][j]);
    }
}
