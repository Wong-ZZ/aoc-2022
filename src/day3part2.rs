use std::{iter::FromIterator};
use std::collections::HashSet;

use crate::parser::read_file;


pub fn main() {
    let mut priorities = std::collections::HashMap::new();
    let mut i = 1;
    for c in "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars() {
        priorities.insert(c, i);
        i += 1;
    }

    let mut results = 0;
    let mut reader = read_file(3, 1).peekable();
    while reader.peek().is_some() {
        (0..3)
            .map(|_| HashSet::<char>::from_iter(reader.next().unwrap().unwrap().chars().into_iter()))
            .reduce(|accum, item| {
                return accum.intersection(&item).map(|c| *c).collect();
            })
            .unwrap_or_else(HashSet::new)
            .into_iter()
            .for_each(|c| results += priorities.get(&c).unwrap());

    }
    println!("{}", results);
}
