use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn score_char(c:u8) -> i32 {
    match c {
        b'a'..=b'z' => 1 + (c - b'a') as i32,
        b'A'..=b'Z' => 27 + (c - b'A') as i32,
        _ => panic!("Unkown char"),
    }
}

fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| {
            let chars = line.unwrap().into_bytes();
            let comps = chars.as_slice().split_at(chars.len() / 2);
            let sets = [comps.0, comps.1].map(|s| s.iter().copied().collect::<HashSet<u8>>());
            sets[0].intersection(&sets[1]).fold(0, |acc,c| acc.max(score_char(*c)))
        }).sum()
}

fn run_two(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file)
        .lines().map(|l| {l.unwrap()}).collect::<Vec<_>>();
    lines.chunks(3).map(|c| {
        let set = c.iter().map(|l| l.bytes().collect::<HashSet<u8>>()).collect::<Vec<_>>();
        let inter = set[0].intersection(&set[1]).into_iter().copied().collect::<HashSet<_>>();
        inter.intersection(&set[2]).fold(0, |acc,c| acc.max(score_char(*c)))
    }).sum()
}

fn run_both(path: &str) -> (i32, i32) {
    (run(path), run_two(path))
}

fn main() {
    let (total_score, total_score_b) = run_both("example.txt");
    println!("The example score is: {} {}", total_score, total_score_b);

    let (total_score, total_score_b) = run_both("input.txt");
    println!("The total score is: {} {}", total_score, total_score_b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc_test() {
        let (total_score, total_score_b) = run_both("input.txt");
        assert_eq!(total_score, 8072);
        assert_eq!(total_score_b, 2567);
    }
}
