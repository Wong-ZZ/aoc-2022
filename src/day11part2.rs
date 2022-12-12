use std::{num::ParseIntError};

use crate::parser::read_file;

struct Monkey {
    pub items: Vec<i64>,
    pub op: (Result<i64, ParseIntError>, String, Result<i64, ParseIntError>),
    pub factor: i64,
    pub next: (usize, usize),
    pub count: usize
}

pub fn main() {
    let mut iter = read_file(11, 1)
        .map(|l| String::from(l.unwrap().trim()))
        .filter(|s| !s.is_empty())
        .peekable();
    let mut monkeys: Vec<Monkey> = vec![];

    while iter.peek().is_some() {
        iter.next();
        let start_items: Vec<i64> = iter.next().unwrap().split(':').nth(1).unwrap().split(',').map(|s| String::from(s.trim())).map(|s| s.parse::<i64>().unwrap()).collect();
        let op: Vec<String> = iter.next().unwrap().split(':').nth(1).unwrap().split('=').nth(1).unwrap().trim().split(' ').map(String::from).collect::<Vec<String>>();
        let op = (op.get(0).unwrap().parse::<i64>(), op.get(1).unwrap().clone(), op.get(2).unwrap().parse::<i64>());
        let factor = iter.next().unwrap().split(' ').last().map(|i| i.parse::<i64>().unwrap()).unwrap();
        let next_true = iter.next().unwrap().split(' ').last().map(|i| i.parse::<usize>().unwrap()).unwrap();
        let next_false = iter.next().unwrap().split(' ').last().map(|i| i.parse::<usize>().unwrap()).unwrap();
        monkeys.push(Monkey { items: start_items, op, factor, next: (next_true, next_false), count: 0 });
    }

    let magic_num = monkeys.iter().map(|m| m.factor).reduce(|x, y| x * y).unwrap();
    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let mut monkey = monkeys.get_mut(i).unwrap();
            let mut moves = vec![];
            monkey.count += monkey.items.len();

            monkey.items.iter().for_each(|item| {
                let mut fear: i64 = *item;
                let (arg1, arg2) = (monkey.op.0.clone().unwrap_or_else(|_| fear), monkey.op.2.clone().unwrap_or_else(|_| fear));
                match monkey.op.1.clone().as_str() {
                    "+" => fear = arg1 + arg2,
                    "-" => fear = arg1 - arg2,
                    "*" => fear = (arg1) * (arg2),
                    "/" => fear = arg1 / arg2,
                    _ => panic!()
                }
                fear = fear % magic_num;
                if fear % monkey.factor == 0 {
                    moves.push((monkey.next.0, fear));
                } else {
                    moves.push((monkey.next.1, fear));
                }
            });

            monkey.items.clear();
            for (idx, item) in moves {
                monkeys.get_mut(idx).unwrap().items.push(item);
            }
        }
    }
    let mut counts = monkeys.iter().map(|m| m.count).collect::<Vec<usize>>();
    println!("{:?}", counts);
    counts.sort();
    let len = counts.len();
    println!("{}", counts.get(len-1).unwrap() * counts.get(len-2).unwrap());
}
