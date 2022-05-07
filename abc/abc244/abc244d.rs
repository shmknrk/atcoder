use proconio::input;
use std::collections::HashSet;

fn main() {
    input! {
        s: (char, char, char),
        t: (char, char, char),
    }
    let set = vec![('R', 'G', 'B'), ('G', 'B', 'R'), ('B', 'R', 'G')]
        .into_iter()
        .collect::<HashSet<(char, char, char)>>();
    if set.contains(&s) == set.contains(&t) {
        println!("Yes");
    }
    else {
        println!("No");
    }
}
