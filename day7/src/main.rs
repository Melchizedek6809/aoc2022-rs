use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum INode {
    Dir { name: String, children: HashMap<String, INode> },
    File { name: String, size: usize },
}

impl INode {
    fn insert_rec(&mut self, path: &Vec<String>, i: usize, node: INode) {
        if let INode::Dir{children, ..} = self {
            if path.len() == i + 1 {
                children.insert(path[i].clone(), node);
            } else {
                if let Some(inode) = children.get_mut(&path[i]) {
                    inode.insert_rec(path, i+1, node);
                } else {
                    panic!("Couldn't find entry");
                }
            }
        } else {
            panic!("Found file in path");
        }
    }

    pub fn insert_dir(&mut self, path: &Vec<String>) {
        let node = INode::Dir{name: path[path.len()-1].clone(), children: HashMap::new() };
        self.insert_rec(path, 0, node);
    }

    pub fn insert_file(&mut self, path: &Vec<String>, size: usize) {
        let node = INode::File{name: path[path.len()-1].clone(), size };
        self.insert_rec(path, 0, node);
    }

    pub fn size(&self) -> usize {
        match self {
            INode::File{size, ..} => *size,
            INode::Dir{children, ..} => {
                children.iter().fold(0, |acc, (_,i)| acc + i.size())
            }
        }
    }

    pub fn task_a(&self) -> usize {
        if let INode::Dir{children, ..} = self {
            let size = self.size();
            let sum = children.iter().map(|(_,i)| i.task_a()).sum();
            if size <= 100000 {
                sum + size
            } else {
                sum
            }
        } else {
            0
        }
    }

    pub fn task_b_rec(&self, mut acc:Vec<usize>) -> Vec<usize> {
        match self {
            INode::Dir {children, ..} => {
                acc.push(self.size());
                children.iter().fold(acc, |acc, (_,i)| i.task_b_rec(acc))
            },
            _ => acc
        }
    }

    pub fn task_b(&self) -> usize {
        let total = 70000000;
        let used = self.size();
        let free = total - used;
        let goal = 30000000 - free;
        let sizes = self.task_b_rec(vec![]);
        *sizes.iter().filter(|v| **v >= goal).min().unwrap()
    }
}

fn run_both(path: &str) -> (usize, usize) {
    let mut root = INode::Dir { name: "".to_string(), children: HashMap::new() };
    let mut cwd = vec![];
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    reader.for_each(|line| {
        let line = line.unwrap();
        if line.starts_with("$ cd") {
            if line == "$ cd /" {
                cwd.clear();
            } else if line == "$ cd .." {
                cwd.pop();
            } else {
                cwd.push(line[5..].to_string());
            }
        } else if line.starts_with("$ ") {
            // Ignore
        } else {
            let s = line.split(" ").collect::<Vec<_>>();
            cwd.push(s[1].to_string());
            if let Ok(size) = s[0].parse::<usize>() {
                root.insert_file(&cwd, size);
            } else {
                root.insert_dir(&cwd);
            }
            cwd.pop();
        }
    });
    (root.task_a(), root.task_b())
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
        assert_eq!(total_score, 1792222);
        assert_eq!(total_score_b, 1112963);
    }
}
