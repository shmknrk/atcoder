use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        mut va: [usize; n],
    }
    let mut sum = 0;
    for i in 0..n {
        let c = if va[i] / x <= k { va[i] / x } else { k };
        k -= c;
        va[i] -= x * c;
        sum += va[i];
    }
    if k > 0 {
        va.sort_unstable_by(|a, b| b.cmp(a));
        for i in 0..if k > n { n } else { k } {
            sum -= va[i];
        }
    }
    println!("{}", sum);
}
