use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
        d: u64,
    }
    if a * 60 + b < c * 60 + d + 1 {
        println!("Takahashi");
    }
    else {
        println!("Aoki");
    }
}
