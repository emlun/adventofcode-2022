use crate::common::Solution;

#[derive(Default)]
struct Game {
    start_inner_c: usize,
    goal_inner_c: usize,
    inner_w: usize,
    inner_h: usize,
    blizzards_up: Vec<u128>,
    blizzards_right: Vec<u128>,
    blizzards_down: Vec<u128>,
    blizzards_left: Vec<u128>,
}

// Solution method by @Hadopire on GitHub
// https://github.com/hadopire/adventofcode_2022/blob/8c4526dd8eb14648a6c31b172f43d32c2a3d301f/d24.odin
fn search(game: &Game, mut trips_left: usize) -> usize {
    let h = game.inner_h + 2;
    let inner_h_sub1 = game.inner_h - 1;
    let inbounds_mask_r0: u128 = 1 << game.start_inner_c;
    let inbounds_mask: u128 = (1 << game.inner_w) - 1;
    let inbounds_mask_rmax: u128 = 1 << game.goal_inner_c;

    let mut t = 0;
    let mut pos: Vec<u128> = vec![0; h];
    pos[0] = inbounds_mask_r0;
    let mut prev_pos: Vec<u128>;

    let mut blizzards_left = game.blizzards_left.clone();
    let mut blizzards_right = game.blizzards_right.clone();

    loop {
        prev_pos = pos.clone();

        {
            let r = 0;
            let moved_up = prev_pos[r + 1];
            pos[r] = inbounds_mask_r0 & (prev_pos[r] | moved_up);
        }

        for r in 1..(h - 1) {
            let moved_left = prev_pos[r] >> 1;
            let moved_right = prev_pos[r] << 1;
            let moved_down = prev_pos[r - 1];
            let moved_up = prev_pos[r + 1];

            let blizzard_mask = {
                let inner_r = r - 1;
                let tr = t % game.inner_h;
                let blizzard_up = game.blizzards_up[(inner_r + tr) % game.inner_h];
                let blizzard_down =
                    game.blizzards_down[(inner_r + inner_h_sub1 * tr) % game.inner_h];

                blizzard_up | blizzard_down | blizzards_left[inner_r] | blizzards_right[inner_r]
            };

            pos[r] = (inbounds_mask ^ blizzard_mask)
                & (prev_pos[r] | moved_left | moved_right | moved_down | moved_up);
        }

        {
            let r = h - 1;
            let moved_down = prev_pos[r - 1];
            pos[r] = inbounds_mask_rmax & (prev_pos[r] | moved_down);
        }

        if (trips_left % 2 == 0 && (pos[h - 1] == inbounds_mask_rmax))
            || trips_left % 2 == 1 && (pos[0] == inbounds_mask_r0)
        {
            if trips_left == 0 {
                return t;
            }

            let reset_rs = if trips_left % 2 == 0 {
                0..(h - 1)
            } else {
                1..h
            };
            for r in reset_rs {
                pos[r] = 0;
            }

            trips_left -= 1;
        }

        for r in 0..game.inner_h {
            blizzards_left[r] = ((blizzards_left[r] >> 1)
                | ((blizzards_left[r] & 1) << (game.inner_w - 1)))
                & inbounds_mask;
            blizzards_right[r] = ((blizzards_right[r] << 1)
                | ((blizzards_right[r] >> (game.inner_w - 1)) & 1))
                & inbounds_mask;
        }

        t += 1;
    }
}

fn solve_a(game: &Game) -> usize {
    search(game, 0)
}

fn solve_b(game: &Game) -> usize {
    search(game, 2)
}

pub fn solve(lines: &[String]) -> Solution {
    let h: usize = lines.iter().filter(|line| !line.is_empty()).count();
    let game: Game = lines
        .iter()
        .filter(|line| !line.is_empty())
        .enumerate()
        .fold(Game::default(), |mut game, (r, line)| {
            game.inner_w = std::cmp::max(game.inner_w, line.len() - 2);
            game.inner_h = h - 2;

            game.blizzards_right.resize(game.inner_h, 0);
            game.blizzards_left.resize(game.inner_h, 0);
            game.blizzards_up.resize(game.inner_h, 0);
            game.blizzards_down.resize(game.inner_h, 0);

            let inner_r = r.saturating_sub(1);

            line.chars().enumerate().fold(game, |mut game, (c, chr)| {
                let inner_c = c.saturating_sub(1);
                if r == 0 {
                    if chr == '.' {
                        game.start_inner_c = inner_c;
                    } else {
                        assert_eq!(chr, '#');
                    }
                } else if r == h - 1 {
                    if chr == '.' {
                        game.goal_inner_c = inner_c;
                    } else {
                        assert_eq!(chr, '#');
                    }
                } else {
                    match chr {
                        '>' => {
                            game.blizzards_right[inner_r] |= 1 << inner_c;
                        }
                        '<' => {
                            game.blizzards_left[inner_r] |= 1 << inner_c;
                        }
                        '^' => {
                            game.blizzards_up[inner_r] |= 1 << inner_c;
                        }
                        'v' => {
                            game.blizzards_down[inner_r] |= 1 << inner_c;
                        }
                        _ => assert!(chr == '#' || chr == '.'),
                    }
                }

                game
            })
        });

    (solve_a(&game).to_string(), solve_b(&game).to_string())
}
