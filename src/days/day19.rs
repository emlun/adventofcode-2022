use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::common::Solution;

type Resources = [u32; 4];

#[derive(Debug)]
struct Blueprint {
    id: usize,
    recipes: [Recipe; 4],
}

#[derive(Debug)]
struct Recipe {
    output: usize,
    ingredients: Resources,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    t: usize,
    resources: Resources,
    robots: Resources,
}

#[derive(Eq, PartialEq)]
struct MaxPotentialWrapper {
    state: State,
    max_potential: u32,
}

impl From<(State, usize)> for MaxPotentialWrapper {
    fn from((state, max_t): (State, usize)) -> Self {
        let dt = u32::try_from(max_t - state.t).unwrap();
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
        .filter(|recipe| {
            recipe
                .ingredients
                .iter()
                .enumerate()
                .all(|(ingredient, qty)| state.resources[ingredient] >= *qty)
        })
        .map(|recipe| Some((recipe.output, &recipe.ingredients)))
        .chain(Some(None))
        .map(|make_robot| State {
            t: state.t + 1,
            resources: {
                let mut res = state.resources;
                for i in 0..res.len() {
                    res[i] =
                        res[i] + state.robots[i] - make_robot.map(|(_, cost)| cost[i]).unwrap_or(0);
                }
                res
            },
            robots: {
                let mut rob = state.robots;
                if let Some((typ, _)) = make_robot {
                    rob[typ] += 1;
                }
                rob
            },
        })
}

fn recipe_is_relevant(state: &State, recipe: &Recipe, blueprint: &Blueprint) -> bool {
    recipe.output == 3
        || blueprint
            .recipes
            .iter()
            .any(|rcp| rcp.ingredients[recipe.output] > state.robots[recipe.output])
}

fn astar(blueprint: &Blueprint, max_t: usize) -> u32 {
    let mut queue: BinaryHeap<MaxPotentialWrapper> = BinaryHeap::new();
    let mut visited: HashMap<usize, HashMap<Resources, Resources>> = HashMap::new();
    let mut best = 0;

    let init_state = State {
        t: 0,
        resources: [0; 4],
        robots: [1, 0, 0, 0],
    };
    queue.push((init_state, max_t).into());

    while let Some(MaxPotentialWrapper {
        state,
        max_potential,
    }) = queue.pop()
    {
        if max_potential <= best {
            return best;
        } else if visited
            .get(&state.t)
            .and_then(|v| v.get(&state.robots))
            .map(|v| {
                v.iter()
                    .zip(state.resources.iter())
                    .all(|(vr, sr)| sr >= vr)
                    || v.iter().zip(state.resources.iter()).any(|(vr, sr)| sr > vr)
            })
            .unwrap_or(true)
        {
            for next_state in generate_moves(&state, blueprint) {
                if visited
                    .get(&next_state.t)
                    .and_then(|v| v.get(&next_state.robots))
                    .map(|v| {
                        v.iter()
                            .zip(next_state.resources.iter())
                            .any(|(vr, sr)| sr > vr)
                    })
                    .unwrap_or(true)
                {
                    best = std::cmp::max(best, next_state.resources[3]);
                    visited
                        .entry(next_state.t)
                        .or_default()
                        .insert(next_state.robots, next_state.resources);
                    if next_state.t < max_t {
                        queue.push((next_state, max_t).into());
                    }
                }
            }
        }
    }
    best
}

fn solve_a(blueprints: &[Blueprint], max_t: usize) -> usize {
    blueprints
        .iter()
        .map(|b| b.id * usize::try_from(astar(b, max_t)).unwrap())
        .sum()
}

fn solve_b(blueprints: &[Blueprint], max_t: usize) -> u32 {
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
