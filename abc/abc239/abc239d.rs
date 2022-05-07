use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }
    let mut prime = vec![true; 200 + 1];
    for i in 2..=13 {
        let mut j = i + i;
        while j <= 200 {
            prime[j] = false;
            j += i;
        }
    }
    let mut i = a;
    while i <= b {
        let mut j = c;
        while j <= d {
            if prime[i + j] {
                break;
            }
            j += 1;
        }
        if j > d {
            break;
        }
        i += 1;
    }
    if i > b {
        println!("Aoki");
    }
    else {
        println!("Takahashi");
    }
}
