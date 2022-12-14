use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Default, Debug)]
struct Sandbox {
    map:HashSet<[i32;2]>,
    pub min_y:i32,
    pub max_y:i32,
}

impl Sandbox {
    pub fn new() -> Self {
        Self {
            min_y: std::i32::MAX,
            max_y: std::i32::MIN,
            map: HashSet::new(),
        }
    }

    pub fn line(&mut self, a:[i32;2], b:[i32;2]) {
        if a[0] == b[0] {
            let x = a[0];
            for y in a[1].min(b[1])..=a[1].max(b[1]) {
                self.map.insert([x,y]);
                self.min_y = self.min_y.min(y);
                self.max_y = self.max_y.max(y);
            }
        } else {
            let y = a[1];
            self.min_y = self.min_y.min(y);
            self.max_y = self.max_y.max(y);
            for x in a[0].min(b[0])..=a[0].max(b[0]) {
                self.map.insert([x,y]);
            }
        }
    }

    pub fn sand(&mut self, x:i32, y:i32) -> bool {
        if y > self.max_y || self.map.contains(&[x, y]) {
            false
        } else {
            if self.map.contains(&[x, y+1]) {
                if !self.map.contains(&[x-1, y+1]) {
                    self.sand(x-1, y+1)
                } else if !self.map.contains(&[x+1, y+1]) {
                    self.sand(x+1, y+1)
                } else {
                    self.map.insert([x, y]);
                    true
                }
            } else {
                self.sand(x, y+1)
            }
        }
    }
}


fn run_both(path: &str) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    let mut sandbox = Sandbox::new();
    reader.for_each(|line| {
        let l = line.unwrap().split(" -> ").map(|c| {
            let pair:[i32;2] = c.split(",").map(|c| c.parse::<i32>().unwrap()).collect::<Vec<_>>().try_into().unwrap();
            pair
        }).reduce(|a,b| {
            sandbox.line(a,b);
            b
        });
    });
    let a = {
        let mut sandbox = sandbox.clone();
        let mut a = 0;
        while sandbox.sand(500,0) {
            a += 1;
        }
        a
    };
    let b = {
        sandbox.line([-1000, sandbox.max_y + 2], [1000, sandbox.max_y + 2]);
        let mut b = 0;
        while sandbox.sand(500,0) {
            b += 1;
        }
        b
    };
    (a,b)
}

fn main() {
    let (total_score, total_score_b) = run_both("example.txt");
    println!("The example score is: {} {}", total_score, total_score_b);

    let (total_score, total_score_b) = run_both("input.txt");
    println!("The score is: {} {}", total_score, total_score_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_test() {
        let (total_score, total_score_b) = run_both("input.txt");
        assert_eq!(total_score, 873);
        assert_eq!(total_score_b, 24813);
    }
}
