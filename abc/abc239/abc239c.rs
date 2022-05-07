use proconio::input;

fn dist_sq(a: i64, b: i64, c: i64, d: i64) -> i64 {
    (a - c).pow(2) + (b - d).pow(2)
}

fn main() {
    input! {
        x1: i64, y1: i64,
        x2: i64, y2: i64,
    }
    for x in x1 - 2..=x1 + 2 {
        for y in y1 - 2..=y1 + 2 {
            if dist_sq(x, y, x1, y1) == 5 && dist_sq(x, y, x2, y2) == 5 {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
