use std::ops::Deref;

use crate::common::Solution;
use crate::search::astar;

#[derive(Clone, Copy, Default, Eq, PartialEq)]
struct Point(usize, usize);

impl Point {
    fn abs_diff(self, Point(ox, oy): Self) -> usize {
        let Point(sx, sy) = self;
        sx.abs_diff(ox) + sy.abs_diff(oy)
    }
}

#[derive(Default, Eq, PartialEq)]
struct Game {
    start: Point,
    goal: Point,
    trip_len: usize,
    minic: usize,
    maxxc: usize,
    minir: usize,
    maxxr: usize,
    inner_w: usize,
    inner_h: usize,
    blizzards_up: Vec<u128>,
    blizzards_right: Vec<u128>,
    blizzards_down: Vec<u128>,
    blizzards_left: Vec<u128>,
    period: usize,
}

#[derive(Eq, PartialEq)]
struct State<'game> {
    game: &'game Game,
    t: usize,
    pos: Point,
    trips_left: usize,
}

impl<'game> State<'game> {
    fn new(game: &'game Game, trips_left: usize) -> Self {
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
            trips_left: if (self.trips_left % 2 == 0 && pos == self.game.goal)
                || (self.trips_left % 2 == 1 && pos == self.game.start)
            {
                self.trips_left.saturating_sub(1)
            } else {
                self.trips_left
            },
        }
    }

    fn has_blizzard(&self, Point(r, c): Point) -> bool {
        let ri = r - self.game.minir;
        let ci = c - self.game.minic;
        let hsub = self.game.inner_h - 1;
        let wsub = self.game.inner_w - 1;

        (self.game.blizzards_up[(self.t + ri) % self.game.inner_h] & (1 << c) != 0)
            || (self.game.blizzards_down[(hsub * self.t + ri) % self.game.inner_h] & (1 << c) != 0)
            || (self.game.blizzards_left[(self.t + ci) % self.game.inner_w] & (1 << r) != 0)
            || (self.game.blizzards_right[(wsub * self.t + ci) % self.game.inner_w] & (1 << r) != 0)
    }
}

#[derive(Eq, PartialEq)]
struct AstarState<'game> {
    state: State<'game>,
    dup_key: usize,
    estimate: usize,
}

impl<'game> From<State<'game>> for AstarState<'game> {
    fn from(state: State<'game>) -> Self {
        let dup_key = ((state.t % state.game.period) << (3 * 8))
            | (state.trips_left << (2 * 8))
            | (state.pos.0 << 8)
            | (state.pos.1);

        let estimate = state.t
            + state.trips_left * state.game.trip_len
            + if state.trips_left % 2 == 0 {
                state.pos.abs_diff(state.game.goal)
            } else {
                state.pos.abs_diff(state.game.start)
            };

        Self {
            dup_key,
            estimate,
            state,
        }
    }
}

impl<'game> Deref for AstarState<'game> {
    type Target = State<'game>;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<'a> astar::State for AstarState<'a> {
    type DuplicationKey = usize;
    type Value = usize;
    type NewStates = Box<dyn Iterator<Item = Self> + 'a>;

    fn duplication_key(&self) -> Self::DuplicationKey {
        self.dup_key
    }

    fn value(&self) -> Self::Value {
        self.state.t
    }

    fn estimate(&self) -> usize {
        self.estimate
    }

    fn generate_moves(self) -> Self::NewStates {
        let Point(r, c) = self.pos;
        let game = self.game;
        Box::new(
            [
                Some((r, c)),
                r.checked_sub(1).map(|r| (r, c)),
                Some((r, c + 1)),
                Some((r + 1, c)),
                c.checked_sub(1).map(|c| (r, c)),
            ]
            .into_iter()
            .flatten()
            .map(|(rr, cc)| Point(rr, cc))
            .filter(|pos @ Point(rr, cc)| {
                *pos == game.start
                    || *pos == game.goal
                    || ((game.minir..game.maxxr).contains(&rr)
                        && (game.minic..game.maxxc).contains(&cc))
            })
            .map(move |pos| self.move_to(pos).into())
            .filter(|state: &Self| {
                state.pos == state.game.start
                    || state.pos == state.game.goal
                    || !state.has_blizzard(state.pos)
            }),
        )
    }
}

const fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

const fn lcm(a: usize, b: usize) -> usize {
    let gcdab = gcd(a, b);
    (a / gcdab) * b
}

fn solve_a(game: &Game) -> usize {
    astar::astar(AstarState::from(State::new(game, 0)))
        .unwrap()
        .t
}

fn solve_b(game: &Game) -> usize {
    astar::astar(AstarState::from(State::new(game, 2)))
        .unwrap()
        .t
}

pub fn solve(lines: &[String]) -> Solution {
    let h: usize = lines.iter().filter(|line| !line.is_empty()).count();
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
                        game.start = Point(r, c);
                    } else {
                        assert_eq!(chr, '#');
                    }
                } else if r == h - 1 {
                    if chr == '.' {
                        game.goal = Point(r, c);
                    } else {
                        assert_eq!(chr, '#');
                    }
                } else {
                    if chr != '#' {
                        let inner_h = std::cmp::max(game.blizzards_down.len(), r);
                        let inner_w = std::cmp::max(game.blizzards_right.len(), c);

                        game.blizzards_right.resize(inner_w, 0);
                        game.blizzards_left.resize(inner_w, 0);
                        game.blizzards_up.resize(inner_h, 0);
                        game.blizzards_down.resize(inner_h, 0);
                    }

                    match chr {
                        '>' => game.blizzards_right[c - 1] |= 1 << r,
                        '<' => game.blizzards_left[c - 1] |= 1 << r,
                        '^' => game.blizzards_up[r - 1] |= 1 << c,
                        'v' => game.blizzards_down[r - 1] |= 1 << c,
                        _ => assert!(chr == '#' || chr == '.'),
                    }
                }

                game
            })
        });
    game.minic = 1;
    game.minir = 1;
    game.inner_h = game.blizzards_down.len();
    game.inner_w = game.blizzards_right.len();
    game.period = lcm(game.maxxr - game.minir, game.maxxc - game.minic);
    game.trip_len = game.goal.abs_diff(game.start);

    (solve_a(&game).to_string(), solve_b(&game).to_string())
}
