use std::collections::BinaryHeap;
use std::collections::HashMap;

use crate::common::Solution;

#[derive(Debug)]
struct Blueprint {
    id: usize,
    recipes: [Recipe; 4],
}

#[derive(Debug)]
struct Recipe {
    output: usize,
    ingredients: Vec<u32>,
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    max_t: usize,
    t: usize,
    resources: Vec<u32>,
    robots: Vec<u32>,
}

impl State {
    fn max_potential(&self) -> u32 {
        let dt = u32::try_from(self.max_t - self.t).unwrap();
        self.resources[3] + self.robots[3] * dt + (dt * (dt - 1)) / 2
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for State {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.max_potential().cmp(&rhs.max_potential())
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
        .filter(|recipe| {
            recipe.output == 3
                || !blueprint.recipes.iter().all(|rcp| {
                    rcp.ingredients[recipe.output] <= state.resources[recipe.output]
                        && rcp.ingredients[recipe.output] <= state.robots[recipe.output]
                })
        })
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
            max_t: state.max_t,
            t: state.t + 1,
            resources: state
                .resources
                .iter()
                .enumerate()
                .map(|(res, have_qty)| {
                    have_qty + state.robots[res]
                        - make_robot.map(|(_, cost)| cost[res]).unwrap_or(0)
                })
                .collect(),
            robots: state
                .robots
                .iter()
                .enumerate()
                .map(|(typ, num)| match make_robot {
                    Some((mk, _)) if mk == typ => num + 1,
                    _ => *num,
                })
                .collect(),
        })
}

fn astar(blueprint: &Blueprint, max_t: usize) -> u32 {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();
    let mut visited: HashMap<usize, HashMap<Vec<u32>, Vec<u32>>> = HashMap::new();
    let mut best = 0;

    let init_state = State {
        max_t,
        t: 0,
        resources: vec![0; 4],
        robots: vec![1, 0, 0, 0],
    };
    queue.push(init_state);

    while let Some(state) = queue.pop() {
        if state.max_potential() <= best {
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
                            || v.iter()
                                .zip(next_state.resources.iter())
                                .all(|(vr, sr)| sr >= vr)
                    })
                    .unwrap_or(true)
                {
                    best = std::cmp::max(best, next_state.resources[3]);
                    visited
                        .entry(next_state.t)
                        .or_default()
                        .insert(next_state.robots.clone(), next_state.resources.clone());
                    if next_state.t < max_t {
                        queue.push(next_state);
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
    let bests: Vec<u32> = blueprints.iter().take(3).map(|b| astar(b, max_t)).collect();
    bests.into_iter().product()
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
                        ingredients: vec![ore_bot, 0, 0, 0],
                    },
                    Recipe {
                        output: 1,
                        ingredients: vec![clay_bot, 0, 0, 0],
                    },
                    Recipe {
                        output: 2,
                        ingredients: vec![obsidian_bot.0, obsidian_bot.1, 0, 0],
                    },
                    Recipe {
                        output: 3,
                        ingredients: vec![geode_bot.0, 0, geode_bot.1, 0],
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
