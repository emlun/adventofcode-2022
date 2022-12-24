use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::common::Solution;

type Point = (usize, usize);

fn print_state(state: &State) {
    for r in 0..=state.game.maxxr {
        for c in 0..=state.game.maxxc {
            if state.pos == (r, c) {
                print!("E");
            } else if (r, c) == (0, 1) || (r, c) == (state.game.maxxr, state.game.maxxc - 1) {
                print!(".");
            } else if r == 0 || r == state.game.maxxr || c == 0 || c == state.game.maxxc {
                print!("#");
            } else {
                let (u, d, l, r) = state.get_blizzard((r, c));
                let n = u8::from(u) + u8::from(d) + u8::from(l) + u8::from(r);
                if n > 1 {
                    print!("{}", n);
                } else if n == 1 {
                    if u {
                        print!("^");
                    } else if d {
                        print!("v");
                    } else if l {
                        print!("<");
                    } else if r {
                        print!(">");
                    }
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }
    println!();
}

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

    fn get_blizzard(&self, (r, c): Point) -> (bool, bool, bool, bool) {
        (
            self.game
                .blizzards_up
                .iter()
                .cycle()
                .skip(self.t)
                .nth(r - self.game.minir)
                .unwrap()[c - self.game.minic],
            self.game
                .blizzards_down
                .iter()
                .cycle()
                .skip((self.game.maxxr - self.game.minir - 1) * self.t)
                .nth(r - self.game.minir)
                .unwrap()[c - self.game.minic],
            self.game
                .blizzards_left
                .iter()
                .cycle()
                .skip(self.t)
                .nth(c - self.game.minic)
                .unwrap()[r - self.game.minir],
            self.game
                .blizzards_right
                .iter()
                .cycle()
                .skip((self.game.maxxc - self.game.minic - 1) * self.t)
                .nth(c - self.game.minic)
                .unwrap()[r - self.game.minir],
        )
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

fn generate_moves<'b>(state: State<'b>) -> impl Iterator<Item = State<'b>> + 'b {
    let (r, c) = state.pos;
    // dbg!((r, c));
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
            // dbg!((rr, cc));
            let pos = (rr, cc);
            Some(State {
                game: state.game,
                t: state.t + 1,
                pos,
                trips_left: if (state.trips_left % 2 == 1 && pos == state.game.goal)
                    || (state.trips_left % 2 == 0 && pos == state.game.start)
                {
                    state.trips_left - 1
                } else {
                    state.trips_left
                },
            })
            .filter(|st| {
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
        // dbg!(state.pos);
        state.pos == state.game.start
            || state.pos == state.game.goal
            || !state.has_blizzard(state.pos)
    })
}

fn astar(game: &Game, trips_left: usize) -> usize {
    let mut queue: BinaryHeap<Reverse<State>> = BinaryHeap::new();
    let mut visited: HashMap<(usize, usize, usize), HashMap<Point, usize>> = HashMap::new();

    let init_state = State {
        game,
        t: 0,
        pos: game.start,
        trips_left,
    };
    queue.push(Reverse(init_state));

    while let Some(Reverse(state)) = queue.pop() {
        // println!(
        //     "l={} \tt={} \t{:?} \te={}",
        //     queue.len(),
        //     state.t,
        //     state.pos,
        //     state.estimate()
        // );
        // print_state(&state);

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
