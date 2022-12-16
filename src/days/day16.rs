use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

struct Valve<'a> {
    name: &'a str,
    rate: u32,
    tunnels: Vec<&'a str>,
}

#[derive(Eq, PartialEq)]
struct State<'a> {
    max_t: u32,
    t: u32,
    pos: &'a str,
    opened: HashSet<&'a str>,
    rate: u32,
    max_rate: u32,
    released: u32,
}

impl<'a> State<'a> {
    fn max_potential(&self) -> u32 {
        self.released + self.max_rate * (self.max_t - self.t)
    }

    fn finished(&self) -> bool {
        self.t >= self.max_t
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.max_potential().cmp(&rhs.max_potential())
    }
}

fn dijkstra<'a>(valves: &HashMap<&str, Valve<'a>>, max_t: u32) -> Option<u32> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut best: u32 = 0;
    let mut visited: HashMap<(u32, &str), u32> = HashMap::new();

    queue.push(State {
        t: 0,
        max_t,
        pos: "AA",
        opened: HashSet::new(),
        rate: 0,
        max_rate: valves.values().map(|v| v.rate).sum(),
        released: 0,
    });

    while let Some(state) = queue.pop() {
        println!(
            "{}\t{}\t{} {} {} {}",
            queue.len(),
            visited.len(),
            state.t,
            state.released,
            state.max_potential(),
            state.opened.len(),
        );

        if state.finished() && state.released > best {
            best = state.released;
        }

        if state.max_potential() <= best {
            return Some(best);
        } else {
            let best_visited = visited
                .entry((state.t, state.pos))
                .or_insert(state.released);
            // {
            if *best_visited <= state.released {
                *best_visited = state.released;

                if state.t < max_t {
                    if !state.opened.contains(state.pos) && valves[state.pos].rate > 0 {
                        let op = {
                            let mut o = state.opened.clone();
                            o.insert(state.pos);
                            o
                        };
                        let next_state = State {
                            t: state.t + 1,
                            max_t: state.max_t,
                            pos: state.pos,
                            opened: op,
                            rate: state.rate + valves[state.pos].rate,
                            max_rate: state.max_rate,
                            released: state.released + state.rate,
                        };
                        if *visited.get(&(next_state.t, next_state.pos)).unwrap_or(&0)
                            <= next_state.released
                        {
                            queue.push(next_state);
                        }
                    }

                    if state.opened.len() < valves.values().filter(|v| v.rate > 0).count() {
                        for connection in &valves[state.pos].tunnels {
                            let next_state = State {
                                t: state.t + 1,
                                max_t: state.max_t,
                                pos: connection,
                                opened: state.opened.clone(),
                                rate: state.rate,
                                max_rate: state.max_rate,
                                released: state.released + state.rate,
                            };
                            if *visited.get(&(next_state.t, next_state.pos)).unwrap_or(&0)
                                <= next_state.released
                            {
                                queue.push(next_state);
                            }
                        }
                    } else {
                        let next_state = State {
                            t: state.max_t,
                            max_t: state.max_t,
                            pos: state.pos,
                            opened: state.opened.clone(),
                            rate: state.rate,
                            max_rate: state.max_rate,
                            released: state.released + (state.max_t - state.t) * state.rate,
                        };
                        if *visited.get(&(next_state.t, next_state.pos)).unwrap_or(&0)
                            <= next_state.released
                        {
                            queue.push(next_state);
                        }
                    }
                }
            }
        }
    }
    None
}

fn solve_a(valves: &HashMap<&str, Valve>, max_time: u32) -> u32 {
    dijkstra(valves, max_time).unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let valves: HashMap<&str, Valve> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split1 = line.strip_prefix("Valve ").unwrap().split(" has ");
            let name = split1.next().unwrap();
            let mut split2 = split1
                .next()
                .unwrap()
                .strip_prefix("flow rate=")
                .unwrap()
                .split(';');
            let rate = split2.next().unwrap().parse().unwrap();
            let tunnels = split2
                .next()
                .and_then(|s| {
                    s.strip_prefix(" tunnel leads to valve ")
                        .or(s.strip_prefix(" tunnels lead to valves "))
                })
                .unwrap()
                .split(", ")
                .collect();
            (
                name,
                Valve {
                    name,
                    rate,
                    tunnels,
                },
            )
        })
        .collect();

    (solve_a(&valves, 30).to_string(), "".to_string())
}
