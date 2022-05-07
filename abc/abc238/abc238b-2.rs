use proconio::input;

fn main() {
    input! {
        n: usize,
        va: [usize; n],
    }
    let mut va2 = [false; 360];
    let mut p = 0;
    va2[0] = true;
    for a in va {
        p = (p + a) % 360;
        va2[p] = true;
    }
    let (mut maxa, mut cnt) = (0, 0);
    for i in 0..=360 {
        if va2[i % 360] {
            if cnt > maxa {
                maxa = cnt;
            }
            cnt = 0;
        }
        cnt += 1;
    }
    println!("{}", maxa);
}
