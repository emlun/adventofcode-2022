use crate::common::Solution;
use crate::search::astar;

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
            trips_left: if (self.trips_left % 2 == 1 && pos == self.game.goal)
                || (self.trips_left % 2 == 0 && pos == self.game.start)
            {
                self.trips_left - 1
            } else {
                self.trips_left
            },
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

impl<'a> astar::State for State<'a> {
    type DuplicationKey = (usize, usize, usize, Point);
    type Value = usize;
    type NewStates = Box<dyn Iterator<Item = Self> + 'a>;

    fn finished(&self) -> bool {
        self.trips_left == 0
    }

    fn duplication_key(&self) -> Self::DuplicationKey {
        (
            self.t % (self.game.maxxr - self.game.minir),
            self.t % (self.game.maxxc - self.game.minic),
            self.trips_left,
            self.pos,
        )
    }

    fn value(&self) -> Self::Value {
        self.t
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

    fn generate_moves(self) -> Self::NewStates {
        let (r, c) = self.pos;
        Box::new(
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
                    Some(self.move_to(pos)).filter(|st| {
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
            }),
        )
    }
}

fn solve_a(game: &Game) -> usize {
    astar::astar(State::new(game, 1)).unwrap().t
}

fn solve_b(game: &Game) -> usize {
    astar::astar(State::new(game, 3)).unwrap().t
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
