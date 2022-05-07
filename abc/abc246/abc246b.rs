use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    let (a2, b2) = ((a * a) as f64, (b * b) as f64);
    println!("{} {}", (a2 / (a2 + b2)).sqrt(), (b2 / (a2 + b2)).sqrt());
}
