use proconio::input;

fn main() {
    input! {
        n: u64,
    }
    if n == 1 || 5 <= n {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
