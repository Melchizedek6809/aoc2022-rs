use std::fs;
use std::collections::HashSet;

fn run(path: &str, len: usize) -> i32 {
    let txt = fs::read_to_string(path).unwrap();
    let mut acc = std::iter::repeat(' ').take(len).collect::<Vec<_>>();
    for (i, c) in txt.chars().into_iter().enumerate() {
        acc.push(c);
        acc.remove(0);
        if i >= len && acc.iter().collect::<HashSet<_>>().len() == len {
            return (1 + i) as i32;
        }
    }
    panic!("Couldn't find a sequence with 4 unique chars");
}

fn run_both(path: &str) -> (i32, i32) {
    (run(path, 4), run(path, 14))
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
        assert_eq!(total_score, 1282);
        assert_eq!(total_score_b, 3513);
    }
}
