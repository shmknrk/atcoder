use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        va: [i64; n + 1],
        vc: [i64; n + m + 1],
    }
    let mut vb: Vec<i64> = vec![0; m + 1];
    for i in (0..=m).rev() {
        let mut ab = 0;
        for j in 1..=if m > n && i < m - n { n } else { m - i } {
            ab += va[n - j] * vb[i + j];
        }
        vb[i] = (vc[n + i] - ab) / va[n];
    }
    for i in 0..m {
        print!("{} ", vb[i]);
    }
    println!("{}", vb[m]);
}
