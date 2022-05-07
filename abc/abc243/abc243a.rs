use proconio::input;

fn main() {
    input! {
        mut v: u64,
        a: u64,
        b: u64,
        c: u64,
    }
    v %= a + b + c;
    if v < a {
        println!("F");
    }
    else if v < a + b {
        println!("M");
    }
    else {
        println!("T");
    }
}
