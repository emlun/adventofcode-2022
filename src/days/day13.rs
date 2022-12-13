use crate::common::Solution;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Packet {
    Int(i32),
    Packet(Vec<Packet>),
}

impl Packet {
    fn parse(s: &str) -> Result<(Self, usize), std::num::ParseIntError> {
        if s.starts_with('[') {
            let mut i = 1;
            let mut sub: Vec<Self> = Vec::new();
            while s.chars().nth(i) != Some(']') {
                if s[i..].starts_with(',') {
                    i += 1;
                } else {
                    let (subp, sublen) = Self::parse(&s[i..])?;
                    sub.push(subp);
                    i += sublen;
                }
            }
            Ok((Self::Packet(sub), i + 1))
        } else if let Some(int_end) = s.find(|c| c == ']' || c == ',') {
            Ok((Self::Int(s[..int_end].parse()?), int_end))
        } else {
            Ok((Self::Int(s.parse()?), s.len()))
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Packet {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => a.cmp(b),
            (Self::Packet(va), Self::Int(b)) => va.cmp(&vec![Self::Int(*b)]),
            (Self::Int(a), Self::Packet(vb)) => vec![Self::Int(*a)].cmp(vb),
            (Self::Packet(va), Self::Packet(vb)) => va.cmp(vb),
        }
    }
}

fn solve_a(pairs: &[(Packet, Packet)]) -> usize {
    pairs
        .iter()
        .enumerate()
        .map(|(i, p)| (i + 1, p))
        .filter(|(i, (a, b))| a <= b)
        .map(|(i, _)| i)
        .sum()
}

fn solve_b(pairs: Vec<(Packet, Packet)>) -> usize {
    let divider_1 = Packet::Packet(vec![Packet::Int(6)]);
    let divider_2 = Packet::Packet(vec![Packet::Int(2)]);

    let mut packets: Vec<Packet> = pairs
        .into_iter()
        .flat_map(|(a, b)| std::iter::once(a).chain(std::iter::once(b)))
        .chain(std::iter::once(divider_1.clone()))
        .chain(std::iter::once(divider_2.clone()))
        .collect();
    packets.sort();
    packets
        .iter()
        .enumerate()
        .find(|(_, p)| **p == divider_1)
        .map(|(i, _)| i + 1)
        .unwrap()
        * packets
            .iter()
            .enumerate()
            .find(|(_, p)| **p == divider_2)
            .map(|(i, _)| i + 1)
            .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let mut pairs: Vec<(Packet, Packet)> = Vec::with_capacity(lines.len() / 2);
    let mut lines = lines.iter().filter(|line| !line.is_empty());
    while let Some(line) = lines.next() {
        pairs.push((
            Packet::parse(line).unwrap().0,
            Packet::parse(lines.next().unwrap()).unwrap().0,
        ));
    }
    (solve_a(&pairs).to_string(), solve_b(pairs).to_string())
}
