use proconio::input;
use proconio::marker::Chars;

struct Person {
    x: i64,
    y: i64,
    dir: char,
}

impl Person {
    fn go_straight(&mut self) {
        match self.dir {
            'E' => self.x += 1,
            'S' => self.y -= 1,
            'W' => self.x -= 1,
            'N' => self.y += 1,
            _ => panic!("invalid input!"),
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            'E' => self.dir = 'S',
            'S' => self.dir = 'W',
            'W' => self.dir = 'N',
            'N' => self.dir = 'E',
            _ => panic!("invalid input!"),
        }
    }

    fn print_point(&self) {
        println!("{} {}", self.x, self.y);
    }
}

fn main() {
    input! {
        _: usize,
        vt: Chars,
    }
    let mut p = Person {
        x: 0,
        y: 0,
        dir: 'E',
    };
    for t in vt {
        match t {
            'S' => p.go_straight(),
            'R' => p.turn_right(),
            _ => panic!("invalid input"),
        }
    }
    p.print_point();
}
