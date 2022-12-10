use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_visible_rec(
    arr: &Vec<Vec<u32>>,
    pos: [i32; 2],
    size: [i32; 2],
    vel: [i32; 2],
    goal: u32,
) -> bool {
    if pos[0] < 0 || pos[1] < 0 || pos[0] >= size[0] || pos[1] >= size[1] {
        return true;
    }
    let v = arr[pos[0] as usize][pos[1] as usize];
    if v >= goal {
        return false;
    }
    let pos = [pos[0] + vel[0], pos[1] + vel[1]];
    return is_visible_rec(arr, pos, size, vel, goal);
}

fn is_visible(arr: &Vec<Vec<u32>>, pos: [i32; 2], size: [i32; 2]) -> bool {
    if pos[0] == 0 || pos[1] == 0 || pos[0] == size[0] - 1 || pos[1] == size[1] - 1 {
        return true;
    }
    let goal = arr[pos[0] as usize][pos[1] as usize];
    is_visible_rec(arr, [pos[0] + 1, pos[1]], size, [1, 0], goal)
        || is_visible_rec(arr, [pos[0], pos[1] + 1], size, [0, 1], goal)
        || is_visible_rec(arr, [pos[0] - 1, pos[1]], size, [-1, 0], goal)
        || is_visible_rec(arr, [pos[0], pos[1] - 1], size, [0, -1], goal)
}

fn count_trees_rec(
    arr: &Vec<Vec<u32>>,
    pos: [i32; 2],
    size: [i32; 2],
    vel: [i32; 2],
    goal: u32,
    acc: i32,
) -> i32 {
    if pos[0] < 0 || pos[1] < 0 || pos[0] >= size[0] || pos[1] >= size[1] {
        return acc;
    }
    let v = arr[pos[0] as usize][pos[1] as usize];
    if v >= goal {
        return acc + 1;
    }
    let pos = [pos[0] + vel[0], pos[1] + vel[1]];
    return count_trees_rec(arr, pos, size, vel, goal, acc + 1);
}

fn count_trees(arr: &Vec<Vec<u32>>, pos: [i32; 2], size: [i32; 2]) -> i32 {
    let goal = arr[pos[0] as usize][pos[1] as usize];
    count_trees_rec(arr, [pos[0] + 1, pos[1]], size, [1, 0], goal, 0)
        * count_trees_rec(arr, [pos[0], pos[1] + 1], size, [0, 1], goal, 0)
        * count_trees_rec(arr, [pos[0] - 1, pos[1]], size, [-1, 0], goal, 0)
        * count_trees_rec(arr, [pos[0], pos[1] - 1], size, [0, -1], goal, 0)
}

fn run_both(path: &str) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let arr = BufReader::new(file)
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = arr.len() as i32;
    let width = arr[0].len() as i32;
    let a = arr.iter().enumerate().fold(0, |acc, (y, cur)| {
        acc + cur.iter().enumerate().fold(0, |acc, (x, _cur)| {
            acc + is_visible(&arr, [x as i32, y as i32], [width, height]) as usize
        })
    });

    let b = arr
        .iter()
        .enumerate()
        .map(|(y, cur)| {
            cur.iter()
                .enumerate()
                .map(|(x, _cur)| count_trees(&arr, [x as i32, y as i32], [width, height]) as usize)
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

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
        assert_eq!(total_score, 1801);
        assert_eq!(total_score_b, 209880);
    }
}
