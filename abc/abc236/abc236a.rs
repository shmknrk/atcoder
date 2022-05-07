use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        a: usize,
        b: usize,
    }
    let t = s[a - 1];
    s[a - 1] = s[b - 1];
    s[b - 1] = t;
    println!("{}", s.into_iter().collect::<String>());
}
