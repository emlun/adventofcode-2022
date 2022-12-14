use crate::common::Solution;
use crate::search::astar;

type Point = (u8, u8);

#[derive(Default, Eq, PartialEq)]
struct Game {
    start: Point,
    goal: Point,
    minic: u8,
    maxxc: u8,
    minir: u8,
    maxxr: u8,
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
    trips_left: u8,
}

impl<'game> State<'game> {
    fn new(game: &'game Game, trips_left: u8) -> Self {
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

    fn has_blizzard(&self, (r, c): Point) -> bool {
        let ri = usize::from(r - self.game.minir);
        let ci = usize::from(c - self.game.minic);
        let hsub = self.game.inner_h - 1;
        let wsub = self.game.inner_w - 1;

        (self.game.blizzards_up[(self.t + ri) % self.game.inner_h] & (1 << c) != 0)
            || (self.game.blizzards_down[(hsub * self.t + ri) % self.game.inner_h] & (1 << c) != 0)
            || (self.game.blizzards_left[(self.t + ci) % self.game.inner_w] & (1 << r) != 0)
            || (self.game.blizzards_right[(wsub * self.t + ci) % self.game.inner_w] & (1 << r) != 0)
    }
}

impl<'a> astar::State for State<'a> {
    type DuplicationKey = usize;
    type Value = usize;
    type NewStates = Box<dyn Iterator<Item = Self> + 'a>;

    fn duplication_key(&self) -> Self::DuplicationKey {
        (self.t << (3 * u8::BITS))
            | (self.game.period << (2 * u8::BITS))
            | (usize::from(self.pos.0) << u8::BITS)
            | usize::from(self.pos.1)
    }

    fn value(&self) -> Self::Value {
        self.t
    }

    fn estimate(&self) -> usize {
        let (r, c) = self.pos;
        let (gr, gc) = self.game.goal;
        let (sr, sc) = self.game.start;
        let trip_len = sr.abs_diff(gr) + sc.abs_diff(gc);

        self.t
            + usize::from(self.trips_left) * usize::from(trip_len)
            + usize::from(if self.trips_left % 2 == 0 {
                r.abs_diff(gr) + c.abs_diff(gc)
            } else {
                r.abs_diff(sr) + c.abs_diff(sc)
            })
    }

    fn generate_moves(self) -> Self::NewStates {
        let (r, c) = self.pos;
        Box::new(
            [
                Some((r, c)),
                r.checked_sub(1).map(|r| (r, c)),
                Some((r, c + 1)),
                Some((r + 1, c)),
                c.checked_sub(1).map(|c| (r, c)),
            ]
            .into_iter()
            .flat_map(move |rrcc| {
                if let Some((rr, cc)) = rrcc {
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
    astar::astar(State::new(game, 0)).unwrap().t
}

fn solve_b(game: &Game) -> usize {
    astar::astar(State::new(game, 2)).unwrap().t
}

pub fn solve(lines: &[String]) -> Solution {
    let h: u8 = u8::try_from(lines.iter().filter(|line| !line.is_empty()).count()).unwrap();
    let mut game: Game = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(Game::default(), |game, (r, line)| {
            line.chars().enumerate().fold(game, |mut game, (c, chr)| {
                let (r, c) = (u8::try_from(r).unwrap(), u8::try_from(c).unwrap());

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
                        let inner_h = std::cmp::max(game.blizzards_down.len(), r.into());
                        let inner_w = std::cmp::max(game.blizzards_right.len(), c.into());

                        game.blizzards_right.resize(inner_w, 0);
                        game.blizzards_left.resize(inner_w, 0);
                        game.blizzards_up.resize(inner_h, 0);
                        game.blizzards_down.resize(inner_h, 0);
                    }

                    match chr {
                        '>' => game.blizzards_right[usize::from(c - 1)] |= 1 << r,
                        '<' => game.blizzards_left[usize::from(c - 1)] |= 1 << r,
                        '^' => game.blizzards_up[usize::from(r - 1)] |= 1 << c,
                        'v' => game.blizzards_down[usize::from(r - 1)] |= 1 << c,
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
    game.period = lcm(
        usize::from(game.maxxr - game.minir),
        usize::from(game.maxxc - game.minic),
    );

    (solve_a(&game).to_string(), solve_b(&game).to_string())
}
