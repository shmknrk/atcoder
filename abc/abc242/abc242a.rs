use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        x: u64,
    }
    if x <= a {
        println!("{}", 1.0);
    }
    else if x <= b {
        println!("{}", c as f64 / (b - a) as f64);
    }
    else {
        println!("{}", 0.0);
    }
}
