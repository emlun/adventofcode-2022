use crate::common::Solution;

use bitgrid::Direction;

mod bitgrid {
    // Flag map:
    //    x
    // y  012345678     i
    //   \SSSSSSSSS/   0-10    Left shift (<<) increases x
    // 0 WAAAAAAAAAE  11-22
    // 1 WAAAAAAAAAE  23-33
    // 2 WAAAAAAAAAE  34-44    West is lower (right-er) index
    // 3 WAAAAAAAAAE  45-55    East is higher (left-er) index
    // 4 WAAAAAAAAAE  56-66
    // 5 WAAAAAAAAAE  67-77
    // 6 WAAAAAAAAAE  78-88
    // 7 WAAAAAAAAAE  89-99
    // 8 WAAAAAAAAAE 100-110
    //   /NNNNNNNNN\ 111-120
    //   .......     121-127    Unused

    use crate::util::collections::SignedVec;
    use std::marker::PhantomData;

    const CELL_WIDTH: isize = 9;

    const SHIFT_N: isize = CELL_WIDTH + 2;
    const SHIFT_E: isize = 1;

    const OVERLAP_S: u128 = 0b11111111111 | (0b11111111111 << SHIFT_N);
    const OVERLAP_W: u128 = 0b11
        | (0b11 << SHIFT_N)
        | (0b11 << (SHIFT_N * 2))
        | (0b11 << (SHIFT_N * 3))
        | (0b11 << (SHIFT_N * 4))
        | (0b11 << (SHIFT_N * 5))
        | (0b11 << (SHIFT_N * 6))
        | (0b11 << (SHIFT_N * 7))
        | (0b11 << (SHIFT_N * 8))
        | (0b11 << (SHIFT_N * 9))
        | (0b11 << (SHIFT_N * 10));
    const OVERLAP_N: u128 = OVERLAP_S << (SHIFT_N * CELL_WIDTH);
    const OVERLAP_E: u128 = OVERLAP_W << CELL_WIDTH;

    const SHIFT_S_I_TO_N_O: isize = SHIFT_N * CELL_WIDTH;
    const SHIFT_W_I_TO_E_O: isize = CELL_WIDTH;
    const SHIFT_SW_I_TO_NE_O: isize = SHIFT_S_I_TO_N_O + SHIFT_W_I_TO_E_O;
    const SHIFT_SE_I_TO_NW_O: isize = SHIFT_S_I_TO_N_O - SHIFT_W_I_TO_E_O;

    const NEIGHBOR_MASK: u128 = 0b111 | (0b101 << SHIFT_N) | (0b111 << (SHIFT_N * 2));
    const NEIGHBOR_S_MASK: u128 = 0b111;
    const NEIGHBOR_N_MASK: u128 = 0b111 << (SHIFT_N * 2);
    const NEIGHBOR_W_MASK: u128 = 0b001 | (0b001 << SHIFT_N) | (0b001 << (SHIFT_N * 2));
    const NEIGHBOR_E_MASK: u128 = 0b100 | (0b100 << SHIFT_N) | (0b100 << (SHIFT_N * 2));

    const MOVE_S_XOR: u128 = 0b010 | (0b010 << SHIFT_N);
    const MOVE_N_XOR: u128 = (0b010 << SHIFT_N) | (0b010 << (SHIFT_N * 2));
    const MOVE_W_XOR: u128 = 0b011 << SHIFT_N;
    const MOVE_E_XOR: u128 = 0b110 << SHIFT_N;

    #[derive(Clone, Default)]
    pub struct BitGrid {
        cols: SignedVec<SignedVec<u128>>,
    }

    #[derive(Clone, Copy)]
    pub enum Direction {
        S,
        N,
        E,
        W,
    }

    impl Direction {
        pub const fn move_coords(&self, x: isize, y: isize) -> (isize, isize) {
            match self {
                Self::S => (x, y - 1),
                Self::N => (x, y + 1),
                Self::W => (x - 1, y),
                Self::E => (x + 1, y),
            }
        }

        const fn neighbor_mask(&self) -> u128 {
            match self {
                Self::S => NEIGHBOR_S_MASK,
                Self::N => NEIGHBOR_N_MASK,
                Self::W => NEIGHBOR_W_MASK,
                Self::E => NEIGHBOR_E_MASK,
            }
        }

        const fn move_xor(&self) -> u128 {
            match self {
                Self::S => MOVE_S_XOR,
                Self::N => MOVE_N_XOR,
                Self::W => MOVE_W_XOR,
                Self::E => MOVE_E_XOR,
            }
        }
    }

    pub struct CellRef<'grid> {
        grid: PhantomData<&'grid BitGrid>,
        cell: u128,
        maskable_cell: u128,
        i: isize,
    }

    pub struct CellRefMut<'grid> {
        grid: &'grid mut BitGrid,
        cellx: isize,
        celly: isize,
        i: isize,
        i_mask: isize,
    }

    impl<'grid> CellRef<'grid> {
        pub fn is_set(&self) -> bool {
            self.cell & (1 << self.i) != 0
        }

        pub fn has_any_neighbor(&self) -> bool {
            self.maskable_cell & NEIGHBOR_MASK != 0
        }

        pub fn has_neighbor_dir(&self, dir: Direction) -> bool {
            self.maskable_cell & dir.neighbor_mask() != 0
        }
    }

    impl<'grid> CellRefMut<'grid> {
        pub fn set(self) {
            let i = self.i;
            let cell = self.grid.get_cell_mut(self.cellx, self.celly);
            let cell_before = *cell;

            *cell |= 1 << i;

            let cell_after = *cell;
            self.update_neighbors(cell_before ^ cell_after);
        }

        pub fn move_bit(self, dir: Direction) {
            let i_mask = self.i_mask;
            let cell = self.grid.get_cell_mut(self.cellx, self.celly);
            let cell_before = *cell;

            *cell ^= dir.move_xor() << i_mask;

            let cell_after = *cell;
            self.update_neighbors(cell_before ^ cell_after);
        }

        fn update_neighbors(self, cell_diff: u128) {
            let diff_s = cell_diff & OVERLAP_S;
            let diff_n = cell_diff & OVERLAP_N;
            let diff_w = cell_diff & OVERLAP_W;
            let diff_e = cell_diff & OVERLAP_E;

            let is_s = diff_s != 0;
            let is_n = diff_n != 0;
            let is_w = diff_w != 0;
            let is_e = diff_e != 0;

            if is_s {
                let neighbor = self.grid.get_cell_mut(self.cellx, self.celly - 1);
                *neighbor ^= diff_s << SHIFT_S_I_TO_N_O;
            } else if is_n {
                let neighbor = self.grid.get_cell_mut(self.cellx, self.celly + 1);
                *neighbor ^= diff_n >> SHIFT_S_I_TO_N_O;
            }
            if is_w {
                let neighbor = self.grid.get_cell_mut(self.cellx - 1, self.celly);
                *neighbor ^= diff_w << SHIFT_W_I_TO_E_O;
            } else if is_e {
                let neighbor = self.grid.get_cell_mut(self.cellx + 1, self.celly);
                *neighbor ^= diff_e >> SHIFT_W_I_TO_E_O;
            }

            if is_s {
                if is_w {
                    let neighbor = self.grid.get_cell_mut(self.cellx - 1, self.celly - 1);
                    *neighbor ^= (diff_s & diff_w) << SHIFT_SW_I_TO_NE_O;
                } else if is_e {
                    let neighbor = self.grid.get_cell_mut(self.cellx + 1, self.celly - 1);
                    *neighbor ^= (diff_s & diff_e) << SHIFT_SE_I_TO_NW_O;
                }
            } else if is_n {
                if is_w {
                    let neighbor = self.grid.get_cell_mut(self.cellx - 1, self.celly + 1);
                    *neighbor ^= (diff_n & diff_w) >> SHIFT_SE_I_TO_NW_O;
                } else if is_e {
                    let neighbor = self.grid.get_cell_mut(self.cellx + 1, self.celly + 1);
                    *neighbor ^= (diff_n & diff_e) >> SHIFT_SW_I_TO_NE_O;
                }
            }
        }
    }

    impl BitGrid {
        const fn to_coords(x: isize, y: isize) -> (isize, isize, isize, isize) {
            let remx = x.rem_euclid(CELL_WIDTH);
            let remy = y.rem_euclid(CELL_WIDTH);
            let cellx = (x - remx) / CELL_WIDTH;
            let celly = (y - remy) / CELL_WIDTH;
            let i = (remy + 1) * SHIFT_N + (remx + 1) * SHIFT_E;
            let i_mask = remy * SHIFT_N + remx * SHIFT_E;
            (cellx, celly, i, i_mask)
        }

        pub fn get(&self, x: isize, y: isize) -> CellRef {
            let (cellx, celly, i, i_mask) = Self::to_coords(x, y);
            let cell = self.get_cell(cellx, celly).copied().unwrap_or(0);
            CellRef {
                grid: PhantomData,
                cell,
                maskable_cell: cell >> i_mask,
                i,
            }
        }

        pub fn get_mut(&mut self, x: isize, y: isize) -> CellRefMut {
            let (cellx, celly, i, i_mask) = Self::to_coords(x, y);
            CellRefMut {
                grid: self,
                cellx,
                celly,
                i,
                i_mask,
            }
        }

        fn get_cell(&self, cellx: isize, celly: isize) -> Option<&u128> {
            self.cols.get(cellx).and_then(|ys| ys.get(celly))
        }

        fn get_cell_mut(&mut self, cellx: isize, celly: isize) -> &mut u128 {
            self.cols
                .get_mut_or_default(cellx)
                .get_mut_or_default(celly)
        }
    }
}

type Point = (isize, isize);

#[derive(Clone)]
struct State {
    poss: Vec<Point>,
    bitgrid: bitgrid::BitGrid,
    first_dir: usize,
}

fn step(state: State) -> Option<State> {
    let mut proposals_grid1 = state.bitgrid.clone();
    let mut proposals_grid2 = state.bitgrid.clone();
    let proposals: Vec<Option<(Point, Direction)>> = state
        .poss
        .iter()
        .copied()
        .map(|(x, y)| {
            let flag = state.bitgrid.get(x, y);

            if flag.has_any_neighbor() {
                [Direction::N, Direction::S, Direction::W, Direction::E]
                    .iter()
                    .cycle()
                    .skip(state.first_dir)
                    .take(4)
                    .copied()
                    .find(|dir| !flag.has_neighbor_dir(*dir))
                    .map(|dir| {
                        let (xx, yy) = dir.move_coords(x, y);
                        if proposals_grid1.get(xx, yy).is_set() {
                            proposals_grid2.get_mut(xx, yy).set();
                        } else {
                            proposals_grid1.get_mut(xx, yy).set();
                        }
                        ((xx, yy), dir)
                    })
            } else {
                None
            }
        })
        .collect();

    if proposals.iter().all(Option::is_none) {
        None
    } else {
        let State {
            mut bitgrid,
            mut poss,
            first_dir,
        } = state;

        for (prop, pos) in proposals.into_iter().zip(poss.iter_mut()) {
            if let Some((dest @ (xx, yy), dir)) = prop {
                if proposals_grid1.get(xx, yy).is_set() && !proposals_grid2.get(xx, yy).is_set() {
                    let (ox, oy) = *pos;
                    *pos = dest;
                    bitgrid.get_mut(ox, oy).move_bit(dir);
                }
            }
        }

        Some(State {
            bitgrid,
            poss,
            first_dir: (first_dir + 1) % 4,
        })
    }
}

fn measure_size(state: &State) -> usize {
    let (minx, maxx, miny, maxy) = state.poss.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(minx, maxx, miny, maxy), (x, y)| {
            (
                std::cmp::min(minx, *x),
                std::cmp::max(maxx, *x),
                std::cmp::min(miny, *y),
                std::cmp::max(maxy, *y),
            )
        },
    );
    ((maxx + 1 - minx) * (maxy + 1 - miny)) as usize - state.poss.len()
}

fn solve_a(mut state: State) -> usize {
    for _ in 0..10 {
        state = step(state).unwrap();
    }
    measure_size(&state)
}

fn solve_b(mut state: State) -> usize {
    let mut i = 1;
    while let Some(s) = step(state) {
        state = s;
        i += 1;
    }
    i
}

pub fn solve(lines: &[String]) -> Solution {
    let poss: Vec<Point> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (isize::try_from(x).unwrap(), -isize::try_from(y).unwrap()))
        })
        .collect();
    let state = State {
        bitgrid: poss
            .iter()
            .copied()
            .fold(Default::default(), |mut bg, (x, y)| {
                bg.get_mut(x, y).set();
                bg
            }),
        poss,
        first_dir: 0,
    };

    (
        solve_a(state.clone()).to_string(),
        solve_b(state).to_string(),
    )
}
