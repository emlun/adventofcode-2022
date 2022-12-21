use std::collections::HashMap;

use crate::common::Solution;

enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

enum Instruction<'a> {
    Num(i64),
    Op(&'a str, Operation, &'a str),
}

fn eval<'a>(monkeys: &'a HashMap<&str, Instruction<'a>>, name: &str) -> i64 {
    match &monkeys[name] {
        Instruction::Num(n) => *n,
        Instruction::Op(a, op, b) => match op {
            Operation::Add => eval(monkeys, a) + eval(monkeys, b),
            Operation::Sub => eval(monkeys, a) - eval(monkeys, b),
            Operation::Mul => eval(monkeys, a) * eval(monkeys, b),
            Operation::Div => eval(monkeys, a) / eval(monkeys, b),
        },
    }
}

fn solve_a(monkeys: &HashMap<&str, Instruction>) -> i64 {
    eval(monkeys, "root")
}

pub fn solve(lines: &[String]) -> Solution {
    let monkeys: HashMap<&str, Instruction> = lines
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut parts = line.split(": ");
            let name = parts.next().unwrap();
            let rhs = parts.next().unwrap();
            (
                name,
                if let Ok(n) = rhs.parse() {
                    Instruction::Num(n)
                } else {
                    let mut expr = rhs.split(' ');
                    Instruction::Op(
                        expr.next().unwrap(),
                        match expr.next().unwrap() {
                            "+" => Operation::Add,
                            "-" => Operation::Sub,
                            "*" => Operation::Mul,
                            "/" => Operation::Div,
                            _ => unimplemented!(),
                        },
                        expr.next().unwrap(),
                    )
                },
            )
        })
        .collect();

    (solve_a(&monkeys).to_string(), "".to_string())
}
