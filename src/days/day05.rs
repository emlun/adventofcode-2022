use crate::common::Solution;

struct Instruction {
    from: usize,
    count: usize,
    to: usize,
}

fn solve_a(mut stacks: Vec<Vec<char>>, program: &[Instruction]) -> String {
    for inst in program {
        for _ in 0..inst.count {
            let moved = stacks[inst.from].pop().unwrap();
            stacks[inst.to].push(moved);
        }
    }
    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

fn solve_b(mut stacks: Vec<Vec<char>>, program: &[Instruction]) -> String {
    for inst in program {
        let moved_first_idx = stacks[inst.from].len() - inst.count;
        let moved: Vec<char> = stacks[inst.from].drain(moved_first_idx..).collect();
        stacks[inst.to].extend(moved);
    }
    stacks
        .into_iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

pub fn solve(lines: &[String]) -> Solution {
    let (layers, program): (Vec<Vec<Option<char>>>, Vec<Instruction>) =
        lines.iter().filter(|line| !line.is_empty()).fold(
            (Vec::new(), Vec::with_capacity(lines.len())),
            |(mut layers, mut instructions), line| {
                if line.starts_with(' ') || line.starts_with('[') {
                    let mut chars = line.chars();
                    let mut layer: Vec<Option<char>> = Vec::new();
                    while let (Some(a), Some(b), Some(c)) =
                        (chars.next(), chars.next(), chars.next())
                    {
                        layer.push(match (a, b, c) {
                            (' ', ' ', ' ') => None,
                            ('[', name, ']') => Some(name),
                            (' ', '1', _) => break,
                            _ => unimplemented!(),
                        });
                        chars.next();
                    }
                    layers.push(layer);
                } else {
                    let mut parts = line[5..].split(' ').step_by(2);
                    instructions.push(Instruction {
                        count: parts.next().unwrap().parse().unwrap(),
                        from: parts.next().unwrap().parse::<usize>().unwrap() - 1,
                        to: parts.next().unwrap().parse::<usize>().unwrap() - 1,
                    });
                }
                (layers, instructions)
            },
        );

    let layer_len = layers[0].len();
    let stacks = layers
        .into_iter()
        .rev()
        .fold(vec![Vec::new(); layer_len], |mut stacks, layer| {
            for (stack, name) in stacks.iter_mut().zip(layer.into_iter()) {
                if let Some(n) = name {
                    stack.push(n);
                }
            }
            stacks
        });

    (solve_a(stacks.clone(), &program), solve_b(stacks, &program))
}
