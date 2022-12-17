use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub type Pressure = i32;
pub type ValveId = u16;

#[inline]
fn read_valve_id(value: &str) -> ValveId {
    let b = value.as_bytes();
    (b[0] as ValveId) | ((b[1] as ValveId) << 8)
}

#[derive(Clone, Debug)]
pub struct Valve {
    pub id: ValveId,
    pub connections: [ValveId; 5],
    pub flow: Pressure,
}

impl From<String> for Valve {
    fn from(value: String) -> Self {
        let value = value
            .replace("Valve ", "")
            .replace("has flow rate=", "")
            .replace("; tunnels lead to valves", "")
            .replace("; tunnel leads to valve", "")
            .replace(",", "");
        let values = value.split(" ").collect::<Vec<_>>();
        let id = read_valve_id(values[0]);
        let flow = values[1].parse().unwrap();
        let mut connections: [ValveId; 5] = [0; 5];
        values.iter().skip(2).enumerate().for_each(|(i,v)| connections[i] = read_valve_id(v));
        Self {
            id,
            connections,
            flow,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ValveGraph {
    pub map: HashMap<ValveId, Valve>,
    pub distance: HashMap<(ValveId, ValveId), i16>,
}

#[derive(Copy, Clone, Debug)]
pub struct ValveActor {
    pub position: ValveId,
    pub time: i16,
}

#[derive(Clone, Debug)]
pub struct ValveState {
    pub actor: [ValveActor;2],
    pub pressure: Pressure,
    pub open_valves: Vec<ValveId>,
}

impl ValveState {
    #[inline]
    pub fn new(time: [i16;2]) -> Self {
        Self {
            actor: [ValveActor {
                time: time[0],
                position: read_valve_id("AA"),
            }, ValveActor {
                time: time[1],
                position: read_valve_id("AA"),
            }],
            pressure: 0,
            open_valves: vec![],
        }
    }

    #[inline]
    pub fn open_valve(&self, v:ValveId, flow:Pressure, distance: i16, actor_index: usize) -> Self {
        let mut open_valves = self.open_valves.clone();
        open_valves.push(v);
        let mut pressure = self.pressure;
        let actor = self.actor[actor_index];
        let actor = {
            let duration = actor.time.min(distance + 1);
            let time_left = 0.max(actor.time - (distance + 1));
            pressure += time_left as i32 * flow;
            ValveActor {
                position: v,
                time: actor.time - duration,
            }
        };
        let mut actors = self.actor;
        actors[actor_index] = actor;
        Self {
            actor: actors,
            open_valves,
            pressure,
        }
    }

    pub fn is_done(&self) -> bool {
        self.actor.iter().all(|a| a.time <= 0)
    }
}

impl ValveGraph {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_valve(&mut self, valve: Valve) {
        self.map.insert(valve.id, valve);
    }


    pub fn solve(&self, state: &ValveState, actor_index: usize) -> i32 {
        if state.is_done() { return state.pressure }

        let mut max:i32 = state.pressure;
        let actor = state.actor[actor_index];
        {
            if actor.time <= 0 {
                return self.solve(state, (actor_index+1) & 1);
            }
            if let Some(cur) = self.map.get(&actor.position) {
                for dest in self.map.values() {
                    if dest.flow <= 0 { continue }
                    if state.open_valves.contains(&dest.id) { continue }
                    if let Some(dist) = self.distance.get(&(cur.id, dest.id)) {
                        max = max.max(self.solve(&state.open_valve(dest.id, dest.flow, *dist, actor_index), (actor_index+1) & 1));
                    }
                }
            }
        };
        max
    }

    fn calc_distance(&self, from: ValveId, to: ValveId) -> i16 {
        let mut oq:Vec<(ValveId, i16)> = vec![(from, 0)];
        loop {
            let q = oq;
            oq = vec![];
            for (p,t) in q {
                if p == to { return t }
                let t = t + 1;
                if let Some(v) = self.map.get(&p) {
                    for p in v.connections {
                        if p == 0 { break }
                        oq.push((p, t));
                    }
                }
            }
        }
    }

    pub fn calc_distances(&mut self) {
        for from in self.map.values() {
            for to in self.map.values() {
                if to.id == from.id || self.distance.get(&(from.id, to.id)).is_some() { continue }
                let d = self.calc_distance(from.id, to.id);
                self.distance.insert((from.id, to.id), d);
                self.distance.insert((to.id, from.id), d);
            }
        }
    }
}

impl<T: BufRead> From<Lines<T>> for ValveGraph {
    fn from(value: Lines<T>) -> Self {
        let mut ret = Self::new();
        value.for_each(|line| {
            if let Ok(line) = line {
                ret.add_valve(line.into());
            }
        });
        ret.calc_distances();
        ret
    }
}

pub fn run_both(path: &str) -> (i32, i32) {
    let graph: ValveGraph = BufReader::new(File::open(path).unwrap()).lines().into();
    let a = graph.solve(&ValveState::new([30,0]),0);
    let b = graph.solve(&ValveState::new([26,26]),0);

    (a, b)
}
