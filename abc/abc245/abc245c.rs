use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        va: [i64; n],
        vb: [i64; n],
    }
    let mut ok = (true, true);
    for i in 0..n - 1 {
        let (ok0, ok1) = ok;
        ok.0 = ok0 && (va[i] - va[i + 1]).abs() <= k || ok1 && (vb[i] - va[i + 1]).abs() <= k;
        ok.1 = ok0 && (va[i] - vb[i + 1]).abs() <= k || ok1 && (vb[i] - vb[i + 1]).abs() <= k;
        if ok == (false, false) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
