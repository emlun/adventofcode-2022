use crate::common::Solution;

type Resources = [u32; 4];

#[derive(Debug)]
struct Blueprint {
    id: u32,
    recipes: [Recipe; 4],
}

#[derive(Debug)]
struct Recipe {
    output: usize,
    ingredients: Resources,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    t: u32,
    resources: Resources,
    robots: Resources,
}

impl State {
    fn max_potential(&self) -> u32 {
        let dt = self.t;
        if dt > 0 {
            self.resources[3] + self.robots[3] * dt + (dt * (dt - 1)) / 2
        } else {
            self.resources[3]
        }
    }
}

fn generate_moves<'a, 'b>(
    state: &'a State,
    blueprint: &'b Blueprint,
) -> impl Iterator<Item = State> + 'a
where
    'b: 'a,
{
    blueprint
        .recipes
        .iter()
        .filter(|recipe| recipe_is_relevant(state, recipe, blueprint))
        .flat_map(|recipe| time_to_afford_recipe(state, recipe).map(|wait_t| (recipe, wait_t)))
        .flat_map(|(recipe, wait_t)| {
            let wait_t = std::cmp::min(wait_t, state.t);
            if wait_t < state.t {
                Some(State {
                    t: state.t - wait_t - 1,
                    resources: {
                        let mut res = state.resources;
                        for (i, res) in res.iter_mut().enumerate() {
                            *res = (*res + state.robots[i] * wait_t - recipe.ingredients[i])
                                + state.robots[i];
                        }
                        res
                    },
                    robots: {
                        let mut rob = state.robots;
                        rob[recipe.output] += 1;
                        rob
                    },
                })
            } else {
                None
            }
        })
}

fn recipe_is_relevant(state: &State, recipe: &Recipe, blueprint: &Blueprint) -> bool {
    recipe.output == 3
        || blueprint
            .recipes
            .iter()
            .any(|rcp| rcp.ingredients[recipe.output] > state.robots[recipe.output])
}

fn time_to_afford_recipe(state: &State, recipe: &Recipe) -> Option<u32> {
    recipe
        .ingredients
        .iter()
        .enumerate()
        .map(|(res_type, cost)| {
            let deficit = cost.saturating_sub(state.resources[res_type]);
            if deficit == 0 {
                Some(0)
            } else {
                let rob = state.robots[res_type];
                if rob > 0 {
                    Some(deficit / rob + std::cmp::min(1, deficit % rob))
                } else {
                    None
                }
            }
        })
        .fold(Some(0), |max_t, next| {
            max_t.and_then(|max_t| next.map(|wait_t| std::cmp::max(max_t, wait_t)))
        })
}

fn search(blueprint: &Blueprint, max_t: u32) -> u32 {
    fn recurse(state: &State, blueprint: &Blueprint, mut best: u32) -> u32 {
        for next_state in generate_moves(&state, blueprint) {
            if next_state.max_potential() > best {
                best = std::cmp::max(best, next_state.resources[3]);
                best = std::cmp::max(best, recurse(&next_state, blueprint, best));
            }
        }
        best
    }

    let init_state = State {
        t: max_t,
        resources: [0; 4],
        robots: [1, 0, 0, 0],
    };

    recurse(&init_state, blueprint, init_state.resources[3])
}

fn solve_a(blueprints: &[Blueprint], max_t: u32) -> u32 {
    blueprints.iter().map(|b| b.id * search(b, max_t)).sum()
}

fn solve_b(blueprints: &[Blueprint], max_t: u32) -> u32 {
    blueprints
        .iter()
        .take(3)
        .map(|b| search(b, max_t))
        .product()
}

pub fn solve(lines: &[String]) -> Solution {
    let blueprints: Vec<Blueprint> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut split1 = line.strip_prefix("Blueprint ").unwrap().split(": ");
            let id = split1.next().unwrap().parse().unwrap();
            let mut bot_splits = split1.next().unwrap().split(". ");

            let ore_bot = bot_splits
                .next()
                .unwrap()
                .strip_prefix("Each ore robot costs ")
                .unwrap()
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap();

            let clay_bot = bot_splits
                .next()
                .unwrap()
                .strip_prefix("Each clay robot costs ")
                .unwrap()
                .strip_suffix(" ore")
                .unwrap()
                .parse()
                .unwrap();

            let mut obsidian_splits = bot_splits.next().unwrap().split(" and ");
            let obsidian_bot = (
                obsidian_splits
                    .next()
                    .unwrap()
                    .strip_prefix("Each obsidian robot costs ")
                    .unwrap()
                    .strip_suffix(" ore")
                    .unwrap()
                    .parse()
                    .unwrap(),
                obsidian_splits
                    .next()
                    .unwrap()
                    .strip_suffix(" clay")
                    .unwrap()
                    .parse()
                    .unwrap(),
            );

            let mut geode_splits = bot_splits.next().unwrap().split(" and ");
            let geode_bot = (
                geode_splits
                    .next()
                    .unwrap()
                    .strip_prefix("Each geode robot costs ")
                    .unwrap()
                    .strip_suffix(" ore")
                    .unwrap()
                    .parse()
                    .unwrap(),
                geode_splits
                    .next()
                    .unwrap()
                    .strip_suffix(" obsidian.")
                    .unwrap()
                    .parse()
                    .unwrap(),
            );

            Blueprint {
                id,
                recipes: [
                    Recipe {
                        output: 0,
                        ingredients: [ore_bot, 0, 0, 0],
                    },
                    Recipe {
                        output: 1,
                        ingredients: [clay_bot, 0, 0, 0],
                    },
                    Recipe {
                        output: 2,
                        ingredients: [obsidian_bot.0, obsidian_bot.1, 0, 0],
                    },
                    Recipe {
                        output: 3,
                        ingredients: [geode_bot.0, 0, geode_bot.1, 0],
                    },
                ],
            }
        })
        .collect();

    (
        solve_a(&blueprints, 24).to_string(),
        solve_b(&blueprints, 32).to_string(),
    )
}
