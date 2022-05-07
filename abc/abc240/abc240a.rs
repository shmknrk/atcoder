use proconio::input;

fn main() {
    input! {
        a: u64,
        b: u64,
    }
    if b - a == 1 || b - a == 9 {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
