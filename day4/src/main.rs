use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split(",").map(|s| {
                s.split("-").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            ((parts[0][0] >= parts[1][0] && parts[0][1] <= parts[1][1])
            || (parts[0][0] <= parts[1][0] && parts[0][1] >= parts[1][1])) as i32
        }).sum()
}

fn run_two(path: &str) -> i32 {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts = line.split(",").map(|s| {
                s.split("-").map(|v| v.parse::<i32>().unwrap()).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            ((parts[0][0] >= parts[1][0] && parts[0][0] <= parts[1][1]) ||
            (parts[0][1] >= parts[1][0] && parts[0][1] <= parts[1][1]) ||
            (parts[1][0] >= parts[0][0] && parts[1][0] <= parts[0][1]) ||
            (parts[1][1] >= parts[0][0] && parts[1][1] <= parts[0][1])) as i32
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
        assert_eq!(total_score, 538);
        assert_eq!(total_score_b, 2567);
    }
}
