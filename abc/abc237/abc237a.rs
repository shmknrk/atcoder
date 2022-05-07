use proconio::input;

fn main() {
    input! {
        n: i64,
    }
    if -1_i64 << 31 <= n && n < 1_i64 << 31 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
