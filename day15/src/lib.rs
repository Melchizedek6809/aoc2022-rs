use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug)]
pub struct Sensor {
    pub pos: [i64; 2],
    pub radius: i64,
}

pub enum IntervalCollation {
    Single([i64; 2]),
    Double([i64; 2], [i64; 2]),
}

#[inline]
fn sensor_to_interval(s: &Sensor, y: i64) -> Option<[i64; 2]> {
    let r = (s.radius - (s.pos[1] - y).abs()).max(-1);
    if r >= 0 {
        Some([s.pos[0] - r, s.pos[0] + r])
    } else {
        None
    }
}

#[inline]
fn collated_intervals(
    buf: &mut Vec<[i64; 2]>,
    collated: &mut Vec<[i64; 2]>,
    sensors: &Vec<Sensor>,
    y: i64,
) -> IntervalCollation {
    buf.clear();
    collated.clear();

    sensors
        .iter()
        .filter_map(|s| sensor_to_interval(s, y))
        .for_each(|e| buf.push(e));
    buf.sort_by(|a, b| a[0].cmp(&b[0]));
    let col = buf.iter().copied().reduce(|a, b| {
        if a[1] < b[0] {
            // No overlap
            collated.push(a);
            b
        } else {
            // Overlap
            [a[0].min(b[0]), a[1].max(b[1])]
        }
    });
    collated.push(col.unwrap_or([0, 0]));
    if collated.len() == 1 {
        IntervalCollation::Single(collated[0])
    } else {
        IntervalCollation::Double(collated[0], collated[1])
    }
}

fn no_beacon_count(sensors: &Vec<Sensor>, y: i64) -> i64 {
    let mut buf = Vec::with_capacity(64);
    let mut collated = Vec::with_capacity(4);
    match collated_intervals(&mut buf, &mut collated, sensors, y) {
        IntervalCollation::Single(a) => a[1] - a[0],
        IntervalCollation::Double(a, b) => (a[1] - a[0]) + (b[1] - b[0]),
    }
}

fn search_for_beacon(sensors: &Vec<Sensor>) -> i64 {
    let mut buf = Vec::with_capacity(64);
    let mut collated = Vec::with_capacity(4);
    for y in 0..4000000 {
        if let IntervalCollation::Double(a, _) =
            collated_intervals(&mut buf, &mut collated, sensors, y)
        {
            return (a[1] + 1) * 4000000 + y;
        }
    }
    panic!("Couldn't determine beacon position");
}

pub fn run_both(path: &str) -> (i64, i64) {
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
                radius,
            }
        })
        .collect::<Vec<_>>();

    let a = no_beacon_count(&sensors, 2000000);
    let b = search_for_beacon(&sensors);

    (a, b)
}
