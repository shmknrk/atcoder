use proconio::input;

fn main() {
    input! {
        va: [usize; 10],
    }
    let mut a = 0;
    for _ in 0..3 {
        a = va[a];
    }
    println!("{}", a);
}
