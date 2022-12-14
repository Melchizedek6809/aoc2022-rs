use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Val {
    Int(i32),
    List(Vec<Val>),
}

impl Display for Val {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int(i) => write!(f, "{}", i),
            Self::List(l) => {
                write!(f, "[{}]", l.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(","))
            }
        }
    }
}

impl Val {

    fn right_order(&self, rhs: &Self) -> Option<bool> {
        match self {
            Self::Int(l) => {
                if let Self::Int(r) = rhs {
                    println!("- Compare {} vs {}", l, r);
                    if l < r {
                        println!("- Left side is smaller");
                        return Some(true);
                    } else if r < l {
                        println!("- Right side is smaller");
                        return Some(false);
                    } else {
                        return None
                    }
                } else {
                    println!("- Mixed types; convert l to [{}]", l);
                    return Val::List(vec![Val::Int(*l)]).right_order(rhs);
                }
            },
            Self::List(la) => {
                match rhs {
                    Self::Int(r) => {
                        println!("- Mixed types; convert r to [{}]", r);
                        return self.right_order(&Val::List(vec![Val::Int(*r)]));
                    },
                    Self::List(ra) => {
                        let mut lai = la.iter();
                        let mut rai = ra.iter();
                        loop {
                            if let Some(laiv) = lai.next() {
                                if let Some(raiv) = rai.next() {
                                    if let Some(v) = laiv.right_order(raiv) {
                                        return Some(v);
                                    }
                                } else {
                                    println!("Right side ran out of items");
                                    return Some(false);
                                }
                            } else {
                                if rai.next().is_some() {
                                    return Some(true);
                                } else {
                                    return None;
                                }
                            }
                        }
                    }
                }
            },
        }
    }

    fn read_list<'a>(mut iter: &'a mut Peekable<Chars<'a>>) -> (Self, &'a mut Peekable<Chars<'a>>) {
        let mut ret:Vec<Val> = vec![];
        loop {
            let c = *iter.peek().unwrap_or(&']');
            match c {
                ']' => {
                    iter.next();
                    return (Val::List(ret), iter);
                },
                ',' => {
                    iter.next();
                },
                _ => {
                    let (v,iter_ret) = Self::read_val(iter);
                    iter = iter_ret;
                    if let Some(v) = v {
                        ret.push(v);
                    } else {
                        return (Val::List(ret), iter);
                    }
                },
            };
        }
    }

    fn read_val<'a>(iter: &'a mut Peekable<Chars<'a>>) -> (Option<Self>, &'a mut Peekable<Chars<'a>>) {
        let mut c = *iter.peek().unwrap_or(&']');
        match c {
            '[' => {
                iter.next();
                let (v,iter) = Self::read_list(iter);
                (Some(v), iter)
            },
            ']' => {
                (None, iter)
            },
            ',' => {
                iter.next();
                Self::read_val(iter)
            },
            '0'..='9' => {
                let mut val:i32 = 0;
                while c != ',' {
                    match c {
                        '0'..='9' => {
                            iter.next();
                            val = val * 10 + (c as i32 - '0' as i32)
                        },
                        _ => break,
                    };
                    c = *iter.peek().unwrap_or(&',');
                }
                return (Some(Val::Int(val)), iter);
            },
            _ => panic!("Reader error: {:?}", c),
        }
    }
}

impl From<String> for Val {
    fn from(v: String) -> Self {
        Self::read_val(&mut v.chars().into_iter().peekable()).0.unwrap()
    }
}

fn run_both(path: &str) -> (usize, usize) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    let lines = reader.map(|l| l.unwrap()).filter(|l| l.len() > 0).collect::<Vec<_>>();
    let a:usize = lines.chunks(2).enumerate().map(|(i, c)| {
        let a:Val = c[0].to_string().into();
        let b:Val = c[1].to_string().into();
        println!("\n== Pair {} ==", i+1);
        a.right_order(&b).unwrap() as usize * (i+1)
    }).sum();

    let mut b = lines.iter().map(|l| {
        Val::from(l.to_string())
    }).collect::<Vec<_>>();
    let decoder_a:Val = "[[2]]".to_string().into();
    b.push(decoder_a.clone());
    let decoder_b:Val = "[[6]]".to_string().into();
    b.push(decoder_b.clone());
    b.sort_by(|a,b| {
        if a.right_order(b).unwrap() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    let mut decoder_a_i = 0;
    let mut decoder_b_i = 0;
    b.iter().enumerate().for_each(|(i,c)| {
        if *c == decoder_a { decoder_a_i = i + 1}
        if *c == decoder_b { decoder_b_i = i + 1}
    });
    (a, decoder_a_i * decoder_b_i)
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
        assert_eq!(total_score, 6395);
        assert_eq!(total_score_b, 24921);
    }
}
