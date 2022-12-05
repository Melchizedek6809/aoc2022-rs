use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[derive(Clone, Default, Debug)]
struct State {
    over_9000: bool,
    stacks: [Vec<char>; 9],
}

impl State {
    pub fn new(over_9000: bool) -> Self {
        Self {
            over_9000,
            ..Default::default()
        }
    }

    pub fn read_data(mut self, mut iter:Lines<BufReader<File>>) -> Self {
        let line = iter.next().unwrap().unwrap();
        if line.trim().len() > 0 {
            line.as_bytes().chunks(4).enumerate().for_each(|(i,s)| {
                if s[0] == b'[' {
                    self.stacks[i].push(s[1] as char);
                };
            });
            self.read_data(iter)
        } else {
            self.stacks.iter_mut().for_each(|s| s.reverse());
            self.eval(iter)
        }
    }

    pub fn eval(mut self, mut iter:Lines<BufReader<File>>) -> Self {
        if let Some(line) = iter.next() {
            match line.unwrap().split(" ")
                .filter_map(|w| w.parse::<usize>().ok())
                .collect::<Vec<usize>>().as_slice() {
                [count, from, to] => {
                    let mut tmp:Vec<char> = Vec::with_capacity(*count);
                    for _ in 0..*count {
                        tmp.push(self.stacks[*from-1].pop().unwrap());
                    }
                    if self.over_9000 { tmp.reverse(); }
                    tmp.iter().for_each(|c| self.stacks[*to-1].push(*c));
                },
                _ => panic!("Invalid line")
            }
            self.eval(iter)
        } else {
            self
        }
    }

    pub fn result(mut self) -> String {
        self.stacks.map(|mut s| s.pop().unwrap_or(' ').to_string()).join("")
    }
}

fn run(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    State::new(false).read_data(reader).result()
}

fn run_two(path: &str) -> String {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    State::new(true).read_data(reader).result()
}

fn run_both(path: &str) -> (String, String) {
    (run(path), run_two(path))
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
        assert_eq!(total_score.trim(), "WSFTMRHPP");
        assert_eq!(total_score_b.trim(), "GSLCMFBRP");
    }
}
