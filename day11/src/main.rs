use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Default)]
enum MonkeyOp {
    #[default]
    MultiplyOld,
    MultiplyImm(i64),
    AddImm(i64),
}

impl MonkeyOp {
    pub fn apply(&self, old: i64) -> i64 {
        match self {
            Self::MultiplyOld => old * old,
            Self::MultiplyImm(imm) => old * imm,
            Self::AddImm(imm) => old + imm,
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    op: MonkeyOp,
    test_divisor: i64,
    true_monkey_id: usize,
    false_monkey_id: usize,

    inspection_counter: usize,
}

impl Monkey {
    pub fn new(init: &Vec<String>) -> Self {
        let mut ret = Self {
            ..Default::default()
        };
        init.iter().for_each(|line| {
            let p = line.split(":").collect::<Vec<_>>();
            if p[0].starts_with("Monkey") {
                ret.id = p[0]
                    .split(" ")
                    .skip(1)
                    .next()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            } else {
                match p[0] {
                    "Starting items" => {
                        ret.items = p[1]
                            .split(",")
                            .map(|p| p.trim().parse().unwrap())
                            .collect::<Vec<_>>()
                    }
                    "Test" => ret.test_divisor = p[1].split(" ").last().unwrap().parse().unwrap(),
                    "If true" => {
                        ret.true_monkey_id = p[1].split(" ").last().unwrap().parse().unwrap()
                    }
                    "If false" => {
                        ret.false_monkey_id = p[1].split(" ").last().unwrap().parse().unwrap()
                    }
                    "Operation" => {
                        let symbols: [&str; 3] = p[1]
                            .split(" ")
                            .skip(3)
                            .collect::<Vec<_>>()
                            .try_into()
                            .unwrap();
                        ret.op = match symbols {
                            ["old", "*", "old"] => MonkeyOp::MultiplyOld,
                            ["old", "*", imm] => MonkeyOp::MultiplyImm(imm.parse().unwrap()),
                            ["old", "+", imm] => MonkeyOp::AddImm(imm.parse().unwrap()),
                            _ => panic!("Unsupported operations: {}", p[1]),
                        }
                    }
                    _ => panic!("Unknown key: {}", p[0]),
                }
            }
        });
        println!("{:?}", ret);
        ret
    }

    pub fn turn(&mut self, monkey_modulo: Option<i64>) -> Vec<(usize, i64)> {
        self.items
            .drain(..)
            .map(|item| {
                self.inspection_counter += 1;
                let worry = if let Some(_) = monkey_modulo {
                    self.op.apply(item)
                } else {
                    self.op.apply(item) / 3
                };
                let dest = if worry % self.test_divisor == 0 {
                    self.true_monkey_id
                } else {
                    self.false_monkey_id
                };
                if let Some(m) = monkey_modulo {
                    (dest, worry % m)
                } else {
                    (dest, worry)
                }
            })
            .collect()
    }

    pub fn inspection_counter(&self) -> usize {
        self.inspection_counter
    }

    fn catch(&mut self, item: i64) {
        self.items.push(item);
    }

    pub fn round(monkeys: &mut Vec<Monkey>, no_worries: bool) {
        let monkey_modulo: Option<i64> = if no_worries {
            None
        } else {
            Some(monkeys.iter().map(|m| m.test_divisor).product())
        };
        for i in 0..monkeys.len() {
            for (monkey, item) in monkeys[i].turn(monkey_modulo) {
                monkeys[monkey].catch(item);
            }
        }
    }

    pub fn business(monkeys: &Vec<Monkey>) -> usize {
        let mut arr = monkeys
            .iter()
            .map(|m| m.inspection_counter())
            .collect::<Vec<_>>();
        arr.sort();
        arr.reverse();
        arr[0] * arr[1]
    }
}

fn run_both(path: &str) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    let mut monkeys = reader
        .fold(vec![vec![]], |mut a, line| {
            let line = line.unwrap();
            let trimmed_line = line.trim();
            if trimmed_line.len() == 0 {
                a.push(vec![]);
            } else {
                a.last_mut().unwrap().push(trimmed_line.to_string());
            }
            a
        })
        .iter()
        .map(|m| Monkey::new(m))
        .collect::<Vec<_>>();
    let a = {
        let mut a = monkeys.clone();
        for r in 0..20 {
            Monkey::round(&mut a, true);
        }
        Monkey::business(&a)
    };
    let b = {
        for r in 0..10000 {
            Monkey::round(&mut monkeys, false);
        }
        Monkey::business(&monkeys)
    };
    (a, b)
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
        assert_eq!(total_score, 55930);
        assert_eq!(total_score_b, 14636993466);
    }
}
