use std::collections::HashMap;

use crate::parser::read_file;

pub fn main() {
    let mut final_values = HashMap::new();
    let mut intermediate_values = HashMap::new();
    read_file(21, 1)
        .for_each(|l| {
            let line = l.unwrap();
            let mut line = line.split(": ").map(String::from);
            let name = line.next().unwrap();
            let vals = line.next().unwrap().split(' ').map(String::from).collect::<Vec<String>>();
            if name == "humn" {
                return;
            } else if vals.len() == 1 {
                final_values.insert(name, vals.get(0).unwrap().parse::<i64>().unwrap());
            } else {
                let mut op = String::from(vals.get(1).unwrap());
                if name == "root" {
                    op = "-".to_string();
                }
                intermediate_values.insert(name, (
                    String::from(vals.get(0).unwrap()),
                    op,
                    String::from(vals.get(2).unwrap())
                ));
            }
        });

    loop {
        let before_len = intermediate_values.len();
        let mut to_delete = vec![];
        intermediate_values
            .iter()
            .for_each(|(k , v)| {
                if k == "humn" || k == "root" {
                    return;
                }
                if final_values.contains_key(&v.0) && final_values.contains_key(&v.2) {
                    let left = final_values.get(&v.0).unwrap();
                    let right = final_values.get(&v.2).unwrap();
                    let val = match v.1.as_str() {
                        "+" => left + right,
                        "-" => left - right,
                        "*" => left * right,
                        "/" => left / right,
                        _ => panic!()
                    };
                    final_values.insert(String::from(k), val);
                    to_delete.push(String::from(k));
                }
            });
        to_delete
            .iter()
            .for_each(|k| {
                intermediate_values.remove(k);
            });
        let after_len = intermediate_values.len();
        if before_len == after_len {
            break;
        }
    }

    solve(&"root".to_string(), 0, &intermediate_values, &final_values);
}

fn solve(var: &String, val: i64, intermediate_values: &HashMap<String, (String, String, String)>, final_values: &HashMap<String, i64>) {
    if var == "humn" {
        println!("||||{val}");
        return;
    }

    let (left, op, right) = intermediate_values.get(var).unwrap();
    if final_values.contains_key(left) {
        let other_val = *final_values.get(left).unwrap();
        match op.as_str() {
            "+" => solve(right, val - other_val, intermediate_values, final_values),
            "-" => solve(right, other_val - val, intermediate_values, final_values),
            "*" => solve(right, val / other_val, intermediate_values, final_values),
            "/" => solve(right, other_val / val, intermediate_values, final_values),
            _ => panic!(),
        }
    } else {
        let other_val = *final_values.get(right).unwrap();
        match op.as_str() {
            "+" => solve(left, val - other_val, intermediate_values, final_values),
            "-" => solve(left, other_val + val, intermediate_values, final_values),
            "*" => solve(left, val / other_val, intermediate_values, final_values),
            "/" => solve(left, other_val * val, intermediate_values, final_values),
            _ => panic!(),
        }
    }
}
