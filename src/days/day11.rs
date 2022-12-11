use crate::common::Solution;
use std::collections::VecDeque;

#[derive(Debug)]
struct Monkey
// where
// F: Fn(u32) -> u32,
{
    items: VecDeque<u32>,
    op: fn(u32, u32) -> u32,
    op_arg: Option<u32>,
    test_divisor: u32,
    test_true_dest: usize,
    test_false_dest: usize,
}

impl Monkey {
    fn new() -> Self {
        Self {
            items: VecDeque::new(),
            op: <u32 as std::ops::Add>::add,
            op_arg: None,
            test_divisor: 0,
            test_true_dest: 0,
            test_false_dest: 0,
        }
    }
}

fn solve_a(mut monkeys: Vec<Monkey>) -> usize {
    let mut inspects = vec![0; monkeys.len()];
    for _ in 0..20 {
        for i in 0..monkeys.len() {
            while let Some(mut worry) = monkeys[i].items.pop_front() {
                inspects[i] += 1;
                worry = (monkeys[i].op)(worry, monkeys[i].op_arg.unwrap_or(worry));
                worry /= 3;
                let dest = if worry % monkeys[i].test_divisor == 0 {
                    monkeys[i].test_true_dest
                } else {
                    monkeys[i].test_false_dest
                };
                monkeys[dest].items.push_back(worry);
            }
        }
    }
    inspects.sort();
    inspects[monkeys.len() - 2] * inspects[monkeys.len() - 1]
}

pub fn solve(lines: &[String]) -> Solution {
    let mut monkeys = Vec::with_capacity((lines.len() + 1) / 7);

    let mut lines = lines.iter();
    while let Some(line) = lines.next() {
        if let Some(i) = line
            .strip_prefix("Monkey ")
            .map(|i| i[..(i.len() - 1)].parse().unwrap())
        {
            let mut monkey = Monkey::new();
            while let Some(line) = lines.next().filter(|line| !line.is_empty()) {
                if let Some(items) = line.strip_prefix("  Starting items: ") {
                    monkey.items = items.split(',').flat_map(|s| s.trim().parse()).collect();
                } else if let Some(operation) = line.strip_prefix("  Operation: new = old ") {
                    let mut splits = operation.split(' ');
                    let op = splits.next().unwrap();
                    let arg = splits.next().unwrap();
                    monkey.op = match op {
                        "+" => <u32 as std::ops::Add>::add,
                        "*" => <u32 as std::ops::Mul>::mul,
                        _ => unimplemented!(),
                    };
                    monkey.op_arg = if arg == "old" {
                        None
                    } else if let Ok(a) = arg.parse() {
                        Some(a)
                    } else {
                        unimplemented!()
                    };
                } else if let Some(divisor) = line.strip_prefix("  Test: divisible by ") {
                    monkey.test_divisor = divisor.parse().unwrap();
                } else if let Some(dest) = line.strip_prefix("    If true: throw to monkey ") {
                    monkey.test_true_dest = dest.parse().unwrap();
                } else if let Some(dest) = line.strip_prefix("    If false: throw to monkey ") {
                    monkey.test_false_dest = dest.parse().unwrap();
                } else {
                    unimplemented!();
                }
            }
            monkeys.insert(i, monkey);
        } else {
            unimplemented!()
        }
    }
    println!("{:?}", monkeys);
    (solve_a(monkeys).to_string(), "".to_string())
}
