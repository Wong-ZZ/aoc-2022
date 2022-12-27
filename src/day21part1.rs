use crate::parser::read_file;


pub fn main() {
    let mut final_values = std::collections::HashMap::new();
    let mut intermediate_values = std::collections::HashMap::new();
    read_file(21, 1)
        .for_each(|l| {
            let line = l.unwrap();
            let mut line = line.split(": ").map(String::from);
            let name = line.next().unwrap();
            let vals = line.next().unwrap().split(' ').map(String::from).collect::<Vec<String>>();
            if vals.len() == 1 {
                final_values.insert(name, vals.get(0).unwrap().parse::<i64>().unwrap());
            } else {
                intermediate_values.insert(name, (
                    String::from(vals.get(0).unwrap()),
                    String::from(vals.get(1).unwrap()),
                    String::from(vals.get(2).unwrap())
                ));
            }
        });
    while !final_values.contains_key("root") {
        let mut to_delete = vec![];
        intermediate_values
            .iter()
            .for_each(|(k , v)| {
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

    }
    println!("{}", final_values.get("root").unwrap());
}
