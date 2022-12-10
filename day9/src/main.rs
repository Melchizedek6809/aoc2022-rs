use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn head_step(head:[i32; 2], direction: char) -> [i32; 2] {
    match direction {
        'U' => [head[0], head[1] + 1],
        'D' => [head[0], head[1] - 1],
        'L' => [head[0] - 1, head[1]],
        'R' => [head[0] + 1, head[1]],
        _ => panic!("Unknown direction: {}", direction)
    }
}

fn tail_step(tail: [i32; 2], head:[i32; 2]) -> [i32; 2] {
    let dx = head[0] - tail[0];
    let dy = head[1] - tail[1];
    if dx.abs() > 1 || dy.abs() > 1 {
        [tail[0] + dx.clamp(-1,1), tail[1] + dy.clamp(-1,1)]
    } else {
        tail
    }
}

fn run(path: &str, knots: usize) -> usize {
    let file = File::open(path).unwrap();
    let mut rope = std::iter::repeat([0, 0]).take(knots).collect::<Vec<_>>();
    let mut visited_positions:HashSet<[i32; 2]> = [[0, 0]].into();

    BufReader::new(file)
        .lines()
        .for_each(|line| {
            let line = line.unwrap();
            let mut chars = line.chars();
            let direction = chars.next().unwrap();
            let steps = chars.collect::<String>().trim().parse::<u32>().unwrap();
            for _ in 0..steps {
                rope[0] = head_step(rope[0], direction);
                for i in 1..rope.len() {\
                    rope[i] = tail_step(rope[i], rope[i-1]);
                }
                visited_positions.insert(rope[rope.len()-1]);
            }
        });
    visited_positions.len()
}

fn run_both(path: &str) -> (usize, usize) {
    (run(path, 2), run(path, 10))
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
        assert_eq!(total_score, 6470);
        assert_eq!(total_score_b, 2658);
    }
}
