use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub struct Sensor {
    pub pos: [i64; 2],
    pub beacon: [i64; 2],
    pub radius: i64,
}

fn sensor_to_interval(s: &Sensor, y: i64) -> Option<[i64; 2]> {
    let r = (s.radius - (s.pos[1] - y).abs()).max(-1);
    if r >= 0 {
        Some([s.pos[0] - r, s.pos[0] + r])
    } else {
        None
    }
}

fn collated_intervals(sensors: &Vec<Sensor>, y: i64) -> Vec<[i64; 2]> {
    let mut intervals = sensors
        .iter()
        .filter_map(|s| sensor_to_interval(s, y))
        .collect::<Vec<_>>();
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));
    let mut collated: Vec<[i64; 2]> = vec![];
    let col = intervals.iter().copied().reduce(|a, b| {
        if a[1] < b[0] {
            // No overlap
            collated.push(a);
            b
        } else {
            // Overlap
            let start = a[0].min(b[0]);
            let end = a[1].max(b[1]);
            let col = [start, end];
            col
        }
    });
    collated.push(col.unwrap_or([0, 0]));
    collated
}

fn no_beacon_count(sensors: &Vec<Sensor>, y: i64) -> i64 {
    collated_intervals(sensors, y)
        .iter()
        .map(|i| i[1] - i[0])
        .sum()
}

fn search_for_beacon(sensors: &Vec<Sensor>) -> i64 {
    for y in 0..4000000 {
        let collated = collated_intervals(sensors, y);
        if collated.len() > 1 {
            let x = collated[0][1] + 1;
            return x * 4000000 + y;
        }
    }
    panic!("Couldn't determine beacon position");
}

fn run_both(path: &str) -> (i64, i64) {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file).lines();
    let sensors = reader
        .map(|line| {
            let line = line.unwrap();
            let line = line.replace(":", "");
            let line = line.replace(",", "");
            let parts = line
                .split(" ")
                .filter(|w| w.contains("="))
                .map(|w| {
                    w.replace("x=", "")
                        .replace("y=", "")
                        .parse::<i64>()
                        .unwrap()
                })
                .collect::<Vec<_>>();

            let radius = (parts[0] - parts[2]).abs() + (parts[1] - parts[3]).abs();
            Sensor {
                pos: [parts[0], parts[1]],
                beacon: [parts[2], parts[3]],
                radius,
            }
        })
        .collect::<Vec<_>>();

    let a = no_beacon_count(&sensors, 2000000);
    let b = search_for_beacon(&sensors);

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
        assert_eq!(total_score, 5367037);
        assert_eq!(total_score_b, 11914583249288);
    }
}
