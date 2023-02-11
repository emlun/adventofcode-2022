use std::collections::BinaryHeap;

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

#[derive(Eq, PartialEq)]
struct MaxPotentialWrapper {
    state: State,
    max_potential: u32,
}

impl From<State> for MaxPotentialWrapper {
    fn from(state: State) -> Self {
        let dt = state.t;
        let max_potential = state.resources[3] + state.robots[3] * dt + (dt * (dt - 1)) / 2;
        Self {
            state,
            max_potential,
        }
    }
}

impl PartialOrd for MaxPotentialWrapper {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for MaxPotentialWrapper {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        (self.max_potential, self.state.robots[3]).cmp(&(rhs.max_potential, rhs.state.robots[3]))
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
                        for i in 0..res.len() {
                            res[i] = (res[i] + state.robots[i] * wait_t - recipe.ingredients[i])
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

fn astar(blueprint: &Blueprint, max_t: u32) -> u32 {
    let mut queue: BinaryHeap<MaxPotentialWrapper> = BinaryHeap::new();
    let mut best = 0;

    let init_state = State {
        t: max_t,
        resources: [0; 4],
        robots: [1, 0, 0, 0],
    };
    queue.push(init_state.into());

    while let Some(MaxPotentialWrapper {
        state,
        max_potential,
    }) = queue.pop()
    {
        if max_potential <= best {
            return best;
        } else {
            for next_state in generate_moves(&state, blueprint) {
                let next_state_final_geodes =
                    next_state.resources[3] + next_state.robots[3] * next_state.t;
                best = std::cmp::max(best, next_state_final_geodes);
                if next_state.t > 0 {
                    queue.push(next_state.into());
                }
            }
        }
    }
    best
}

fn solve_a(blueprints: &[Blueprint], max_t: u32) -> u32 {
    blueprints.iter().map(|b| b.id * astar(b, max_t)).sum()
}

fn solve_b(blueprints: &[Blueprint], max_t: u32) -> u32 {
    blueprints.iter().take(3).map(|b| astar(b, max_t)).product()
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
