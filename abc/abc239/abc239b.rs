use proconio::input;

fn floor(x: i64) -> i64 {
    if x < 0 && x % 10 != 0 {
        return x / 10 - 1;
    }
    x / 10
}

fn main() {
    input! {
        x: i64,
    }
    println!("{}", floor(x));
}
