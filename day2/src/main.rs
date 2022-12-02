use std::fs::File;
use std::io::{BufRead, BufReader};

fn win_score(me: u8, elf: u8) -> i32 {
    (me != ((elf + 2) % 3)) as i32 * 6
}

fn calc_score(me: u8, elf: u8) -> i32 {
    let m = (me == elf) as i32;
    me as i32 + 1 + m * 3 + (1 - m) * win_score(me, elf)
}

fn get_choice(result: u8, elf: u8) -> u8 {
    (elf + (4 - (2 - result))) % 3
}

fn parse_line(line: &str) -> i32 {
    let chars: Vec<u8> = line.bytes().collect();
    let elf = chars[0] - b'A';
    let me = chars[2] - b'X';
    calc_score(me, elf)
}

fn parse_line_b(line: &str) -> i32 {
    let chars: Vec<u8> = line.bytes().collect();
    let elf = chars[0] - b'A';
    let me = chars[2] - b'X';
    calc_score(get_choice(me, elf), elf)
}

fn run(path: &str, λ: impl Fn(&str) -> i32) -> i32 {
    let file = File::open(path).unwrap();
    BufReader::new(file)
        .lines()
        .fold(0, |acc, line| acc + λ(&line.unwrap()))
}

fn run_both(path: &str) -> (i32, i32) {
    (run(path, parse_line), run(path, parse_line_b))
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
    fn day_one() {
        let (total_score, total_score_b) = run_both("input.txt");
        assert_eq!(total_score, 14827);
        assert_eq!(total_score_b, 13889);
    }
}
