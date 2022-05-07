use proconio::input;

fn main() {
    input! {
        n: usize,
        va: [usize; 4 * n - 1],
    }
    let mut cnt = vec![0; n + 1];
    for a in va {
        cnt[a] += 1;
    }
    for i in 1..=n {
        if cnt[i] == 3 {
            println!("{}", i);
            return;
        }
    }
}
