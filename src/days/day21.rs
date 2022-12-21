use std::collections::HashMap;

use crate::common::Solution;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
enum Expr {
    Unknown,
    Num(i64),
    Op(Box<Expr>, Operation, Box<Expr>),
}

impl Expr {
    fn op<A, B>(a: A, op: Operation, b: B) -> Self
    where
        A: Into<Box<Expr>>,
        B: Into<Box<Expr>>,
    {
        Expr::Op(a.into(), op, b.into())
    }

    fn unwrap(self) -> i64 {
        use Expr::*;
        match self {
            Num(n) => n,
            other => panic!("Not a numeric value: {:?}", other),
        }
    }

    fn simplify(self) -> Self {
        use Expr::*;
        use Operation::*;

        match self {
            Op(a, op, b) => match (a.simplify(), op, b.simplify()) {
                (Num(a), Add, Num(b)) => Num(a + b),
                (Num(a), Sub, Num(b)) => Num(a - b),
                (Num(a), Mul, Num(b)) => Num(a * b),
                (Num(a), Div, Num(b)) if b != 0 && a % b == 0 => Num(a / b),

                (a, Add, Num(0)) => a,
                (Num(0), Add, b) => b,

                (a, Sub, Num(0)) => a,

                (a, Mul, Num(1)) => a,
                (Num(1), Mul, b) => b,

                (a, Div, b) => match b {
                    Num(1) => a,
                    Op(b, Div, c) => Self::op(Self::op(a, Mul, c).simplify(), Div, b).simplify(),
                    b => Self::op(a, Div, b),
                },

                (a, op, b) => Self::op(a, op, b),
            },
            other => other,
        }
    }

    fn reciprocal(self) -> Self {
        use Expr::*;
        match self {
            Unknown => Self::op(Num(1), Operation::Div, Unknown),
            n @ Num(..) => Self::op(Num(1), Operation::Div, n),
            Op(a, Operation::Div, b) => Op(b, Operation::Div, a),
            a @ Op(..) => Self::op(Num(1), Operation::Div, a),
        }
    }
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

fn expr<'a>(monkeys: &'a HashMap<&str, Instruction<'a>>, name: &str) -> Expr {
    if name == "humn" {
        Expr::Unknown
    } else {
        match &monkeys[name] {
            Instruction::Num(n) => Expr::Num(*n),
            Instruction::Op(a, op, b) => Expr::op(expr(monkeys, a), *op, expr(monkeys, b)),
        }
    }
}

fn solve_a(monkeys: &HashMap<&str, Instruction>) -> i64 {
    eval(monkeys, "root")
}

fn solve_b(monkeys: &HashMap<&str, Instruction>) -> i64 {
    use Expr::*;
    use Operation::*;

    if let Op(lhs, _, rhs) = expr(monkeys, "root") {
        let mut lhs = lhs.simplify();
        let mut rhs = rhs.simplify();

        loop {
            println!("lhs = {:?}", lhs);
            println!("rhs = {:?}", rhs);

            (lhs, rhs) = match (lhs.simplify(), rhs.simplify()) {
                (Unknown, b) => return b.unwrap(),

                (Op(a1, aop, a2), Num(b)) => match (a1.simplify(), aop, a2.simplify()) {
                    (a1, Add, Num(a2)) => (a1, Num(b - a2)),
                    (a1, Sub, Num(a2)) => (a1, Num(b + a2)),
                    (a1, Mul, Num(a2)) => (a1, Expr::op(Num(b), Div, Num(a2))),
                    (a1, Div, Num(a2)) => (a1, Num(b * a2)),

                    (Num(a1), Add, a2) => (a2, Num(b - a1)),
                    (Num(a1), Sub, a2) => (a2, Num(a1 - b)),
                    (Num(a1), Mul, a2) => (a2, Expr::op(Num(b), Div, Num(a1))),
                    (Num(a1), Div, a2) => (a2, Expr::op(Num(a1), Div, Num(b))),

                    (a1, aop, a2) => (Expr::op(a1, aop, a2), Num(b)),
                },

                (Op(a1, aop, a2), b) => match (a1.simplify(), aop, a2.simplify()) {
                    (Unknown, Add, a) => (Unknown, Expr::op(b, Sub, a)),
                    (Unknown, Sub, a) => (Unknown, Expr::op(b, Add, a)),
                    (Unknown, Mul, a) => (Unknown, Expr::op(b, Div, a)),
                    (Unknown, Div, a) => (Unknown, Expr::op(b, Mul, a)),

                    (a, Add, Unknown) => (Unknown, Expr::op(b, Sub, a)),
                    (a, Sub, Unknown) => (Unknown, Expr::op(a, Sub, b)),
                    (a, Mul, Unknown) => (Unknown, Expr::op(b, Div, a)),
                    (a, Div, Unknown) => (Unknown, Expr::op(a, Mul, b)),

                    (a1, aop, a2) => (Expr::op(a1, aop, a2), b),
                },

                (l, r) => (r, l),
            };
        }
    } else {
        unimplemented!()
    }
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

    (solve_a(&monkeys).to_string(), solve_b(&monkeys).to_string())
}
