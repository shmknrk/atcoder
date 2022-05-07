use proconio::input;
const MOD: u64 = 998244353;
const INV2: u64 = 499122177;

fn main() {
    input! {
        n: u64,
    }
    let k = n.to_string().chars().count() as u64;
    let (mut ans, mut p10, mut i) = (0, 10, 0);
    while i < k {
        let m = if p10 - 1 < n {
            p10 - p10 / 10
        } else {
            n - p10 / 10 + 1
        };
        ans = (ans + ((((m % MOD) * ((m + 1) % MOD)) % MOD) * INV2) % MOD) % MOD;
        p10 *= 10;
        i += 1;
    }
    println!("{}", ans);
}
