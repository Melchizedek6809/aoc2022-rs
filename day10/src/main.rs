use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone, Debug)]
enum MachineOp {
    NoOp,
    AddX(i32),
}

impl TryFrom<String> for MachineOp {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut parts = s.split(' ');
        let opcode = parts.next().ok_or("Missing opcode".to_string())?;
        match opcode {
            "noop" => return Ok(MachineOp::NoOp),
            "addx" => {
                let imm = parts.next().ok_or("Missing addx immediate".to_string())?;
                let imm = imm.parse::<i32>();
                if let Ok(imm) = imm {
                    return Ok(MachineOp::AddX(imm));
                } else {
                    return Err("Couldn't parse addx immediate".to_string());
                }
            },
            _ => return Err(format!("Unknown upcode: {}", opcode))
        }
    }
}

impl MachineOp {
    pub fn cycles(&self) -> usize {
        match self {
            Self::NoOp => 0,
            Self::AddX(_) => 1,
        }
    }

    pub fn eval(&self, machine: &mut Machine) {
        if machine.wait_cycles >= self.cycles() {
            machine.wait_cycles = 0;
            match self {
                Self::NoOp => machine.ip += 1,
                Self::AddX(imm) => {
                    machine.x += imm;
                    machine.ip += 1;
                },
            };
            return;
        } else {
            machine.wait_cycles += 1;
        }
    }
}

#[derive(Clone, Debug)]
struct Machine {
    x: i32,
    ip: usize,
    cycles: usize,
    wait_cycles: usize,
    signal_strength: isize,
    mem: Vec<MachineOp>,
}

impl Machine {
    pub fn new(mem: Vec<MachineOp>) -> Self {
        Self {
            x: 1,
            ip: 0,
            cycles: 0,
            signal_strength: 0,
            wait_cycles: 0,
            mem,
        }
    }

    fn update_signal_strength(&mut self) {
        if (self.cycles + 20) % 40 == 0 {
            self.signal_strength += self.cycles as isize * self.x as isize;
        }
    }

    pub fn run(&mut self) {
        while self.ip < self.mem.len() {
            self.cycles += 1;
            self.update_signal_strength();
            let op = self.mem[self.ip];
            op.eval(self);
        }
    }

    pub fn signal_strength(&self) -> isize {
        self.signal_strength
    }
}

fn run(path: &str) -> isize {
    let file = File::open(path).unwrap();


    let program:Vec<MachineOp> = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.try_into().unwrap()
        }).collect();
    let mut machine = Machine::new(program);
    machine.run();
    machine.signal_strength()
}

fn run_both(path: &str) -> (isize, isize) {
    (run(path), run(path))
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
        assert_eq!(total_score, 17840);
    }
}
