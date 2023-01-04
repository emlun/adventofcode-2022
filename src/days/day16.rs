use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::VecDeque;

use crate::common::Solution;
use crate::search::astar;

#[derive(Eq, PartialEq)]
struct Game<'game> {
    players: usize,
    max_t: u32,
    valves: &'game HashMap<u128, Valve>,
    move_map: &'game HashMap<u128, Vec<(u128, u32)>>,
}

#[derive(Debug, Eq, PartialEq)]
struct Valve {
    rate: u32,
    tunnels: Vec<u128>,
}

#[derive(Eq, PartialEq)]
struct State<'game> {
    game: &'game Game<'game>,
    max_potential: u32,
    opened: u128,
    locked_rate: u32,
    released: u32,
    players: Vec<Player>,
}

impl<'game> State<'game> {
    fn new(game: &'game Game) -> Self {
        let locked_rate = game.valves.values().map(|v| v.rate).sum();
        Self {
            game,
            max_potential: (game.max_t - 1) * locked_rate,
            opened: 0,
            locked_rate,
            released: 0,
            players: vec![Player { t: 0, pos: 1 }; game.players],
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Player {
    t: u32,
    pos: u128,
}

impl<'game> astar::State for State<'game> {
    type DuplicationKey = u128;
    type Value = Reverse<u32>;
    type NewStates = Box<dyn Iterator<Item = Self> + 'game>;

    fn value(&self) -> Self::Value {
        Reverse(self.released)
    }

    fn estimate(&self) -> Self::Value {
        Reverse(self.max_potential)
    }

    fn duplication_key(&self) -> Self::DuplicationKey {
        self.opened
    }

    fn generate_moves(self) -> Self::NewStates {
        Box::new(
            (0..self.players.len())
                .map(move |i| {
                    let mut players = self.players.clone();
                    let player = players.remove(i);
                    (player, players)
                })
                .filter(|(p, _)| p.t < self.game.max_t)
                .flat_map(move |(player, other_players)| {
                    self.game.move_map[&player.pos]
                        .iter()
                        .filter(move |(_, dt)| player.t + *dt + 1 < self.game.max_t)
                        .filter(move |(next_pos, _)| self.opened & next_pos == 0)
                        .map(move |(next_pos, dt)| {
                            let player_t = player.t + dt + 1;
                            let players: Vec<Player> = other_players
                                .iter()
                                .cloned()
                                .chain(Some(Player {
                                    t: player_t,
                                    pos: *next_pos,
                                }))
                                .collect();

                            let t = players.iter().map(|p| p.t).min().unwrap();
                            let released_rate = self.game.valves[next_pos].rate;
                            let released =
                                self.released + released_rate * (self.game.max_t - player_t);
                            let locked_rate = self.locked_rate - released_rate;

                            State {
                                game: self.game,
                                max_potential: released
                                    + (self.game.max_t.saturating_sub(t + 1)) * locked_rate,
                                opened: self.opened | next_pos,
                                locked_rate,
                                released,
                                players,
                            }
                        })
                }),
        )
    }
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

    let sol_a = astar::astar_optimize(State::new(&Game {
        valves: &flag_valves,
        move_map: &move_map,
        players: 1,
        max_t: 30,
    }))
    .0;

    let sol_b = astar::astar_optimize(State::new(&Game {
        valves: &flag_valves,
        move_map: &move_map,
        players: 2,
        max_t: 30 - 4,
    }))
    .0;

    (sol_a.to_string(), sol_b.to_string())
}
