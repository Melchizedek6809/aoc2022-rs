use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(path:&str) -> (i32, i32) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let mut elves:Vec<i32> = vec![0];
    for line in reader.lines() {
        let line = line.unwrap();
        if let Ok(num) = line.trim().parse::<i32>() {
            *elves.last_mut().unwrap() += num;
        }else{
            elves.push(0);
        }
    }
    elves.sort();
    (*elves.last().unwrap(), elves.iter().rev().take(3).fold(0,|a,c|a+c))
}

fn main() {
    let (most, top_three) = run("input.txt");
    println!("Most calories carried by a single elf: {}", most);
    println!("Calories carried by the top three elves: {}", top_three);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_one() {
        let (most, top_three) = run("input.txt");
        assert_eq!(most, 74394);
        assert_eq!(top_three, 212836);
    }
}