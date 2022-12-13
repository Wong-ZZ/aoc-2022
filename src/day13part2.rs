use std::cmp::Ordering;

use crate::parser::read_file;

struct Elem {
    is_list: bool,
    val: Option<i32>,
    vals: Option<Vec<Elem>>,
    is_divider: bool
}

impl Elem {
    fn add_elem(&mut self, elem: Elem) {
        self.vals.as_mut().unwrap().push(elem);
    }
}

pub fn main() {
    let mut elems = read_file(13, 1)
        .map(|l| String::from(l.unwrap().trim()))
        .filter(|s| !s.is_empty())
        .map(|line| {
            let bytes = line.as_bytes();
            let len = bytes.len();
            let root = Elem{ is_list: true, val: None, vals: Some(vec![]), is_divider: false };
            let (root, _) = parse(bytes, 1, len, root);
            return root;
        })
        .collect::<Vec<Elem>>();
    let divider1 = Elem{
        is_list: true,
        val: None,
        vals: Some(vec![
            Elem{
                is_list: false,
                val: Some(2),
                vals: None,
                is_divider: false
            }
        ]),
        is_divider: true
    };
    let divider2 = Elem{
        is_list: true,
        val: None,
        vals: Some(vec![
            Elem{
                is_list: false,
                val: Some(6),
                vals: None,
                is_divider: false
            }
        ]),
        is_divider: true
    };
    elems.push(divider1);
    elems.push(divider2);
    elems.sort_by(compare);
    let mut result = 1;
    for (i, elem) in elems.iter().enumerate() {
        if elem.is_divider {
            result *= i+1;
        }
    }
    println!("{result}");
}

fn compare(left: &Elem, right: &Elem) -> Ordering {
    if !left.is_list && !right.is_list {
        if left.val == right.val {
            return Ordering::Equal;
        } else {
            return match left.val < right.val {
                true => Ordering::Less,
                false => Ordering::Greater
            };
        }
    } else {
        let left_vals = match left.is_list {
            true => left.vals.as_ref().unwrap().iter().collect::<Vec<&Elem>>(),
            false => vec![left]
        };
        let right_vals = match right.is_list {
            true => right.vals.as_ref().unwrap().iter().collect::<Vec<&Elem>>(),
            false => vec![right]
        };

        let left_len = left_vals.len();
        let right_len = right_vals.len();

        for idx in 0..std::cmp::min(left_len, right_len) {
            let outcome = compare(left_vals.get(idx).unwrap(), right_vals.get(idx).unwrap());
            if outcome == Ordering::Equal {
                continue;
            }
            return outcome;
        }
        return match left_len == right_len {
            true => Ordering::Equal,
            false => match left_len < right_len {
                true => Ordering::Less,
                false => Ordering::Greater
            }
        };
    }
}

fn parse(input: &[u8], i: usize, len: usize, mut current_node: Elem) -> (Elem, usize) {
    let mut i = i;
    let mut c = input[i] as char;

    while c != ']' {
        if c == '[' {
            let new_node = Elem{ is_list: true, val: None, vals: Some(vec![]), is_divider: false };
            let result = parse(input, i+1, len, new_node);
            i = result.1;
            current_node.add_elem(result.0);
            c = input[i] as char;
        } else if c == ',' {
            i += 1;
            c = input[i] as char;
            continue;
        } else {
            let mut val = vec![c.to_string()];
            loop {
                i += 1;
                c = input[i] as char;
                if (c as u8) < 48 || (c as u8) > 57 {
                    break;
                }
                val.push(c.to_string());
            }
            let val = val.join("").parse::<i32>().unwrap();
            current_node.add_elem(Elem{ is_list: false, val: Some(val), vals: None, is_divider: false });
        }
    }
    return (current_node, i+1);
}
