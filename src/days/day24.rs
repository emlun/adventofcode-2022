use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::common::Solution;

type Point = (usize, usize);

#[derive(Default, Eq, PartialEq)]
struct Game {
    start: Point,
    goal: Point,
    minic: usize,
    maxxc: usize,
    minir: usize,
    maxxr: usize,
    blizzards_up: Vec<Vec<bool>>,
    blizzards_right: Vec<Vec<bool>>,
    blizzards_down: Vec<Vec<bool>>,
    blizzards_left: Vec<Vec<bool>>,
}

#[derive(Eq, PartialEq)]
struct State<'a> {
    game: &'a Game,
    t: usize,
    pos: Point,
    trips_left: usize,
}

impl<'a> State<'a> {
    fn new(game: &'a Game, trips_left: usize) -> Self {
        Self {
            game,
            t: 0,
            pos: game.start,
            trips_left,
        }
    }

    fn move_to(&self, pos: Point) -> Self {
        State {
            game: self.game,
            t: self.t + 1,
            pos,
            trips_left: if (self.trips_left % 2 == 1 && pos == self.game.goal)
                || (self.trips_left % 2 == 0 && pos == self.game.start)
            {
                self.trips_left - 1
            } else {
                self.trips_left
            },
        }
    }

    fn estimate(&self) -> usize {
        let (r, c) = self.pos;
        let (gr, gc) = self.game.goal;
        let (sr, sc) = self.game.goal;
        let trip_len = sr.abs_diff(gr) + sc.abs_diff(gc);

        if self.trips_left % 2 == 1 {
            self.t + r.abs_diff(gr) + c.abs_diff(gc) + self.trips_left * trip_len
        } else {
            self.t + r.abs_diff(sr) + c.abs_diff(sc) + self.trips_left * trip_len
        }
    }

    fn has_blizzard(&self, (r, c): Point) -> bool {
        self.game
            .blizzards_up
            .iter()
            .cycle()
            .skip(self.t)
            .nth(r - self.game.minir)
            .unwrap()[c - self.game.minic]
            || self
                .game
                .blizzards_down
                .iter()
                .cycle()
                .skip((self.game.maxxr - self.game.minir - 1) * self.t)
                .nth(r - self.game.minir)
                .unwrap()[c - self.game.minic]
            || self
                .game
                .blizzards_left
                .iter()
                .cycle()
                .skip(self.t)
                .nth(c - self.game.minic)
                .unwrap()[r - self.game.minir]
            || self
                .game
                .blizzards_right
                .iter()
                .cycle()
                .skip((self.game.maxxc - self.game.minic - 1) * self.t)
                .nth(c - self.game.minic)
                .unwrap()[r - self.game.minir]
    }
}

impl<'a> PartialOrd for State<'a> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.estimate().cmp(&rhs.estimate())
    }
}

fn generate_moves(state: State) -> impl Iterator<Item = State> {
    let (r, c) = state.pos;
    [
        (Some(r), Some(c)),
        (r.checked_sub(1), Some(c)),
        (Some(r), Some(c + 1)),
        (Some(r + 1), Some(c)),
        (Some(r), c.checked_sub(1)),
    ]
    .into_iter()
    .flat_map(move |rrcc| {
        if let (Some(rr), Some(cc)) = rrcc {
            let pos = (rr, cc);
            Some(state.move_to(pos)).filter(|st| {
                st.pos == st.game.start
                    || st.pos == st.game.goal
                    || (st.game.minir..st.game.maxxr).contains(&rr)
                        && (st.game.minic..st.game.maxxc).contains(&cc)
            })
        } else {
            None
        }
    })
    .filter(|state| {
        state.pos == state.game.start
            || state.pos == state.game.goal
            || !state.has_blizzard(state.pos)
    })
}

fn astar(game: &Game, trips_left: usize) -> usize {
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut visited: HashMap<(usize, usize, usize), HashMap<Point, usize>> = HashMap::new();

    let init_state = State::new(game, trips_left);
    queue.push(Reverse(init_state));

    while let Some(Reverse(state)) = queue.pop() {
        if state.trips_left == 0 {
            return state.t;
        } else if visited
            .get(&(
                state.t % (state.game.maxxr - state.game.minir),
                state.t % (state.game.maxxc - state.game.minic),
                state.trips_left,
            ))
            .and_then(|v| v.get(&state.pos))
            .map(|t| *t >= state.t)
            .unwrap_or(true)
        {
            for next_state in generate_moves(state) {
                if visited
                    .get(&(
                        next_state.t % (next_state.game.maxxr - next_state.game.minir),
                        next_state.t % (next_state.game.maxxc - next_state.game.minic),
                        next_state.trips_left,
                    ))
                    .and_then(|v| v.get(&next_state.pos))
                    .map(|t| *t > next_state.t)
                    .unwrap_or(true)
                {
                    visited
                        .entry((
                            next_state.t % (next_state.game.maxxr - next_state.game.minir),
                            next_state.t % (next_state.game.maxxc - next_state.game.minic),
                            next_state.trips_left,
                        ))
                        .or_default()
                        .entry(next_state.pos)
                        .and_modify(|t| *t = next_state.t)
                        .or_insert(next_state.t);
                    queue.push(Reverse(next_state));
                }
            }
        }
    }

    unimplemented!()
}

fn solve_a(game: &Game) -> usize {
    astar(game, 1)
}

fn solve_b(game: &Game) -> usize {
    astar(game, 3)
}

pub fn solve(lines: &[String]) -> Solution {
    let h = lines.iter().filter(|line| !line.is_empty()).count();
    let mut game: Game = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(Game::default(), |game, (r, line)| {
            line.chars().enumerate().fold(game, |mut game, (c, chr)| {
                if chr == '#' {
                    game.maxxc = std::cmp::max(game.maxxc, c);
                    game.maxxr = std::cmp::max(game.maxxr, r);
                }

                if r == 0 {
                    if chr == '.' {
                        game.start = (r, c);
                    } else {
                        assert_eq!(chr, '#');
                    }
                } else if r == h - 1 {
                    if chr == '.' {
                        game.goal = (r, c);
                    } else {
                        assert_eq!(chr, '#');
                    }
                } else {
                    if chr != '#' {
                        let inner_h = std::cmp::max(game.blizzards_down.len(), r);
                        let inner_w = std::cmp::max(game.blizzards_right.len(), c);

                        game.blizzards_right.resize(inner_w, vec![false; inner_h]);
                        game.blizzards_left.resize(inner_w, vec![false; inner_h]);
                        game.blizzards_up.resize(inner_h, vec![false; inner_w]);
                        game.blizzards_down.resize(inner_h, vec![false; inner_w]);

                        game.blizzards_right[c - 1].resize(inner_h, false);
                        game.blizzards_left[c - 1].resize(inner_h, false);
                        game.blizzards_up[r - 1].resize(inner_w, false);
                        game.blizzards_down[r - 1].resize(inner_w, false);
                    }

                    match chr {
                        '>' => game.blizzards_right[c - 1][r - 1] = true,
                        '<' => game.blizzards_left[c - 1][r - 1] = true,
                        '^' => game.blizzards_up[r - 1][c - 1] = true,
                        'v' => game.blizzards_down[r - 1][c - 1] = true,
                        _ => assert!(chr == '#' || chr == '.'),
                    }
                }

                game
            })
        });
    game.minic = 1;
    game.minir = 1;

    (solve_a(&game).to_string(), solve_b(&game).to_string())
}
