use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

use crate::common::Solution;

struct Valve<'a> {
    rate: u32,
    tunnels: Vec<&'a str>,
}

#[derive(Eq, PartialEq)]
struct State<'a> {
    max_t: u32,
    t: u32,
    pos: Vec<&'a str>,
    opened: Vec<&'a str>,
    locked_rate: u32,
    released: u32,
}

impl<'a> State<'a> {
    fn max_potential(&self) -> u32 {
        self.released
            + if self.t < self.max_t {
                self.locked_rate * (self.max_t - self.t - 1)
            } else {
                0
            }
    }

    fn finished(&self) -> bool {
        self.locked_rate == 0 || self.t >= self.max_t
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        (self.max_potential(), self.released).cmp(&(rhs.max_potential(), rhs.released))
    }
}

fn dijkstra<'a>(valves: &HashMap<&str, Valve<'a>>, num_pos: usize, max_t: u32) -> Option<u32> {
    let relevant_valves = valves.values().map(|v| v.rate).count();
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<(u32, Vec<&str>, Vec<&str>), u32> = HashMap::new();
    // assert_eq!(num_pos, 1);

    queue.push(State {
        t: 0,
        max_t,
        pos: vec!["AA"; num_pos],
        opened: vec![],
        locked_rate: valves.values().map(|v| v.rate).sum(),
        released: 0,
    });

    while let Some(state) = queue.pop() {
        // println!(
        //     "{}\tv={}\tt={}  r={}  m={}  o={}/{}  l={}  {:?} {:?}",
        //     queue.len(),
        //     visited.len(),
        //     state.t,
        //     state.released,
        //     state.max_potential(),
        //     state.opened.len(),
        //     relevant_valves,
        //     state.locked_rate,
        //     state.pos,
        //     state.opened,
        // );
        // assert!(state.released <= state.max_potential());

        if state.finished() {
            return Some(state.released);
        } else if visited
            .get(&(state.t, state.pos.clone(), state.opened.clone()))
            .map(|b| *b <= state.released)
            .unwrap_or(true)
        {
            let mut legal_moves: Vec<Vec<Option<&str>>> = state
                .pos
                .iter()
                .map(|pos| {
                    let mut moves = Vec::new();
                    if !state.opened.contains(pos) && valves[pos].rate > 0 {
                        moves.push(None);
                    }
                    moves.extend(valves[pos].tunnels.iter().copied().map(Some));
                    moves
                })
                .collect();
            // assert_eq!(legal_moves.len(), num_pos);
            // dbg!(&legal_moves);

            for i in 0..num_pos {
                let dup_is: Vec<usize> = state.pos[i + 1..]
                    .iter()
                    .enumerate()
                    .filter(|(_, pos)| **pos == state.pos[i])
                    .map(|(ii, _)| ii + i + 1)
                    .collect();
                // dbg!(&dup_is);
                // assert_eq!(dup_is.len(), 0);
                for ii in dup_is {
                    legal_moves[ii].retain(Option::is_some);
                }
            }

            // dbg!(&legal_moves);

            let mut move_is = vec![0; num_pos];
            while move_is[0] < legal_moves[0].len() {
                // dbg!(&move_is);
                let next_move: Vec<Option<&str>> = legal_moves
                    .iter()
                    .zip(move_is.iter())
                    .map(|(moves, i)| moves[*i])
                    .collect();
                // dbg!(&next_move);

                // assert_eq!(next_move.len(), num_pos);

                let mut i = num_pos - 1;
                move_is[i] += 1;
                while i > 0 && move_is[i] >= legal_moves[i].len() {
                    move_is[i] = 0;
                    move_is[i - 1] += 1;
                    i -= 1;
                }

                let opening: Vec<&str> = state
                    .pos
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| next_move[*i].is_none())
                    .map(|(_, m)| *m)
                    .collect();
                // dbg!(&opening);
                // assert!(opening.len() <= num_pos);

                let op = {
                    let mut o = state.opened.clone();
                    o.extend(opening.iter());
                    o.sort();
                    o
                };

                let mut next_pos: Vec<&str> = next_move
                    .iter()
                    .enumerate()
                    .map(|(i, m)| m.unwrap_or(state.pos[i]))
                    .collect();
                next_pos.sort();
                // assert_eq!(next_pos.len(), num_pos);

                // dbg!(state.rate + opening.iter().map(|pos| valves[pos].rate).sum::<u32>());
                // dbg!(op.iter().map(|pos| valves[pos].rate).sum::<u32>());

                let drate = opening.iter().map(|pos| valves[pos].rate).sum::<u32>();
                // assert!(drate <= state.locked_rate);
                // dbg!(drate);
                let next_state = State {
                    t: if drate == state.locked_rate {
                        state.max_t
                    } else {
                        state.t + 1
                    },
                    max_t: state.max_t,
                    pos: next_pos.clone(),
                    opened: op.clone(),
                    locked_rate: state.locked_rate - drate,
                    released: state.released + drate * (state.max_t - state.t - 1),
                };
                // assert!(next_state.released >= state.released);

                if visited
                    .get(&(next_state.t, next_pos.clone(), op.clone()))
                    .map(|b| *b < next_state.released)
                    .unwrap_or(true)
                {
                    visited.insert((next_state.t, next_pos, op), next_state.released);
                    queue.push(next_state);
                }
            }
        }
    }
    None
}

fn solve_a(valves: &HashMap<&str, Valve>, max_time: u32) -> u32 {
    dijkstra(valves, 1, max_time).unwrap()
}

fn solve_b(valves: &HashMap<&str, Valve>, max_time: u32) -> u32 {
    dijkstra(valves, 2, max_time - 4).unwrap()
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
            (name, Valve { rate, tunnels })
        })
        .collect();

    (
        solve_a(&valves, 30).to_string(),
        solve_b(&valves, 30).to_string(),
    )
}
