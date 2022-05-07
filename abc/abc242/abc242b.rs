use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }
    s.sort_unstable();
    println!("{}", s.into_iter().collect::<String>());
}
