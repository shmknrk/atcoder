use proconio::input;

fn main() {
    input! {
        n: usize,
        va2: [u64; n],
    }
    let mut va = Vec::with_capacity(n + 2);
    va.push(0);
    va.extend(va2);

    for i in 1..=n {
        va[i] = (va[i] + va[i - 1]) % 360;
    }
    va.sort();
    va.push(360);
    let mut va2 = Vec::with_capacity(n + 1);
    for i in 0..n + 1 {
        va2.push(va[i + 1] - va[i]);
    }
    let mut maxa = 0;
    for a in &va2 {
        if *a > maxa {
            maxa = *a;
        }
    }
    println!("{}", maxa);
}
