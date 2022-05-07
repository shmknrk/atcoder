use proconio::input;
const MOD: u64 = 998244353;

fn main() {
    input! {
        n: usize,
    }
    let mut dp: Vec<Vec<u64>> = vec![vec![0; 11]; n];
    for i in 1..=9 {
        dp[0][i] = 1;
    }
    for i in 1..n {
        for j in 1..=9 {
            dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j] + dp[i - 1][j + 1];
            dp[i][j] %= MOD;
        }
    }
    let mut a = 0;
    for d in &dp[n - 1] {
        a += d;
        a %= MOD;
    }
    println!("{}", a);
}
