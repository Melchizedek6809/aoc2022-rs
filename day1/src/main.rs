use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
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
    let most = *elves.last().unwrap();
    let top_three = elves.iter().rev().take(3).fold(0,|a,c|a+c);
    println!("Most calories carried by a single elf: {}", most);
    println!("Calories carried by the top three elves: {}", top_three);
}
