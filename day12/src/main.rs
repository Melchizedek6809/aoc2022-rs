use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_path(map: &Vec<Vec<u8>>, start_pos: (usize, usize), end_pos: (usize, usize)) -> (u32,u32) {
    let mut m:Vec<Vec<u32>> = map.iter().map(|row| {
        row.iter().map(|_| std::u32::MAX).collect()
    }).collect();
    let width = m[0].len();
    let height = m.len();
    let mut q:HashSet<(usize, usize)> = [].into();
    q.insert(end_pos);
    let mut v = 0;
    let mut b = std::u32::MAX;
    while !q.is_empty() {
        let t = q;
        q = [].into();
        for (x,y) in t {
            if map[y][x] == 1 { b = b.min(v); }
            if x == start_pos.0 && y == start_pos.1 { return (v,b); }
            if m[y][x] <= v { continue }
            m[y][x] = v;
            let min_height = map[y][x] - 1;
            if x > 0 && map[y][x-1] >= min_height { q.insert((x-1, y)); }
            if y > 0 && map[y-1][x] >= min_height { q.insert((x, y-1)); }
            if x < width-1 && map[y][x+1] >= min_height { q.insert((x+1, y)); }
            if y < height-1 && map[y+1][x] >= min_height { q.insert((x, y+1)); }
        }
        v += 1;
    }
    panic!("Couldn't find a way");
}

fn run_both(path: &str) -> (u32, u32) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    let mut start_pos = (0,0);
    let mut end_pos = (0,0);
    let map = reader.enumerate().map(|(y,line)| {
        line.unwrap().bytes().enumerate().map(|(x,v)| {
            match v {
                b'S' => {
                    start_pos = (x,y);
                    1
                },
                b'E' => {
                    end_pos = (x,y);
                    26
                },
                b'a'..=b'z' => (v - b'a') + 1,
                _ => panic!("Unknown char in map: {}", v as char),
            }
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    find_path(&map, start_pos, end_pos)
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
        assert_eq!(total_score, 472);
        assert_eq!(total_score_b, 465);
    }
}
