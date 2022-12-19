use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::common::Solution;

#[derive(Debug)]
struct Valve {
    rate: u32,
    tunnels: Vec<u128>,
}

#[derive(Eq, PartialEq)]
struct State {
    max_t: u32,
    t: u32,
    max_potential: u32,
    opened: u128,
    locked_rate: u32,
    released: u32,
    players: Vec<Player>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Player {
    t: u32,
    pos: u128,
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for State {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.max_potential.cmp(&rhs.max_potential)
    }
}

fn generate_moves<'a, 'b>(
    state: &'a State,
    valves: &'b HashMap<u128, Valve>,
    move_map: &'b HashMap<u128, Vec<(u128, u32)>>,
) -> impl Iterator<Item = State> + 'a
where
    'b: 'a,
{
    state
        .players
        .iter()
        .enumerate()
        .filter(|(_, p)| p.t < state.max_t)
        .flat_map(move |(i, player)| {
            move_map[&player.pos]
                .iter()
                .filter(|(_, dt)| player.t + *dt + 1 < state.max_t)
                .filter(|(next_pos, _)| state.opened & next_pos == 0)
                .map(move |(next_pos, dt)| {
                    let player_t = player.t + dt + 1;
                    let players: Vec<Player> = state
                        .players
                        .iter()
                        .enumerate()
                        .map(|(ii, p)| {
                            if ii == i {
                                Player {
                                    t: player_t,
                                    pos: *next_pos,
                                }
                            } else {
                                *p
                            }
                        })
                        .collect();

                    let t = players.iter().map(|p| p.t).min().unwrap();
                    let released =
                        state.released + valves[next_pos].rate * (state.max_t - player_t);
                    let locked_rate = state.locked_rate - valves[next_pos].rate;

                    State {
                        max_t: state.max_t,
                        t,
                        max_potential: released + (state.max_t - t - 1) * locked_rate,
                        opened: state.opened | next_pos,
                        locked_rate,
                        released,
                        players,
                    }
                })
        })
}

fn bfs(valves: &HashMap<u128, Valve>, from: u128) -> Vec<(u128, u32)> {
    let mut queue: VecDeque<u128> = VecDeque::new();
    let mut shortest: HashMap<u128, u32> = HashMap::with_capacity(valves.len());

    shortest.insert(from, 0);
    queue.push_back(from);

    while let Some(pos) = queue.pop_front() {
        for next in &valves[&pos].tunnels {
            if !shortest.contains_key(next) {
                shortest.insert(*next, shortest[&pos] + 1);
                queue.push_back(*next);
            }
        }
    }

    shortest
        .into_iter()
        .filter(|(pos, _)| valves[pos].rate > 0)
        .collect()
}

fn astar(
    valves: &HashMap<u128, Valve>,
    move_map: &HashMap<u128, Vec<(u128, u32)>>,
    num_pos: usize,
    max_t: u32,
) -> u32 {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<u128, u32> = HashMap::new();
    let mut best = 0;

    queue.push(State {
        t: 0,
        max_t,
        opened: 0,
        locked_rate: valves.values().map(|v| v.rate).sum(),
        released: 0,
        max_potential: valves.values().map(|v| v.rate).sum::<u32>() * max_t,
        players: vec![Player { t: 0, pos: 1 }; num_pos],
    });

    while let Some(state) = queue.pop() {
        if state.max_potential <= best {
            return best;
        } else if visited
            .get(&state.opened)
            .map(|b| *b <= state.released)
            .unwrap_or(true)
        {
            for next_state in generate_moves(&state, valves, move_map) {
                if next_state.max_potential > best
                    && visited
                        .get(&next_state.opened)
                        .map(|b| *b < next_state.released)
                        .unwrap_or(true)
                {
                    best = std::cmp::max(best, next_state.released);
                    visited.insert(next_state.opened, next_state.released);
                    queue.push(next_state);
                }
            }
        }
    }
    best
}

pub fn solve(lines: &[String]) -> Solution {
    let mut valves: Vec<(&str, u32, Vec<&str>)> = lines
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
                        .or_else(|| s.strip_prefix(" tunnels lead to valves "))
                })
                .unwrap()
                .split(", ")
                .collect();
            (name, rate, tunnels)
        })
        .collect();
    valves.sort_by_key(|(n, _, _)| *n);

    let valve_flags: HashMap<&str, u128> = valves
        .iter()
        .map(|(n, _, _)| n)
        .enumerate()
        .map(|(i, n)| (*n, 1 << i))
        .collect();

    let flag_valves: HashMap<u128, Valve> = valves
        .into_iter()
        .map(|(name, rate, tunnels)| {
            (
                valve_flags[name],
                Valve {
                    rate,
                    tunnels: tunnels.into_iter().map(|n| valve_flags[n]).collect(),
                },
            )
        })
        .collect();

    let relevant_positions: Vec<u128> = flag_valves
        .iter()
        .filter(|(i, v)| **i == 1 || v.rate > 0)
        .map(|(i, _)| *i)
        .collect();

    let move_map: HashMap<u128, Vec<(u128, u32)>> = relevant_positions
        .iter()
        .map(|i| (*i, bfs(&flag_valves, *i)))
        .collect();

    (
        astar(&flag_valves, &move_map, 1, 30).to_string(),
        astar(&flag_valves, &move_map, 2, 30 - 4).to_string(),
    )
}
