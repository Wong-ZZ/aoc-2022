use crate::parser::read_file;

struct Elem {
    is_list: bool,
    val: Option<i32>,
    vals: Option<Vec<Elem>>
}

impl Elem {
    fn add_elem(&mut self, elem: Elem) {
        self.vals.as_mut().unwrap().push(elem);
    }
}

pub fn main() {
    let mut iter = read_file(13, 1)
        .map(|l| String::from(l.unwrap().trim()))
        .filter(|s| !s.is_empty())
        .peekable();
    let mut i = 0;
    let mut result = 0 as usize;
    while iter.peek().is_some() {
        i += 1;
        let left = iter.next().unwrap();
        let right = iter.next().unwrap();
        let left_bytes = left.as_bytes();
        let right_bytes = right.as_bytes();

        let left_root = Elem{ is_list: true, val: None, vals: Some(vec![]) };
        let right_root = Elem{ is_list: true, val: None, vals: Some(vec![]) };
        let (left_root, _) = parse(left_bytes, 1, left.len(), left_root);
        let (right_root, _) = parse(right_bytes, 1, right.len(), right_root);

        let outcome = compare(&left_root, &right_root);
        if outcome.is_some() && outcome.unwrap() {
            result += i;
        }
    }
    println!("{result}");
}

fn compare(left: &Elem, right: &Elem) -> Option<bool> {
    if !left.is_list && !right.is_list {
        if left.val == right.val {
            return None;
        } else {
            return Some(left.val < right.val);
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
            if outcome.is_none() {
                continue;
            }
            return outcome;
        }
        return match left_len == right_len {
            true => None,
            false => Some(left_len < right_len)
        };
    }
}

fn parse(input: &[u8], i: usize, len: usize, mut current_node: Elem) -> (Elem, usize) {
    let mut i = i;
    let mut c = input[i] as char;

    while c != ']' {
        if c == '[' {
            let new_node = Elem{ is_list: true, val: None, vals: Some(vec![]) };
            let result = parse(input, i+1, len, new_node);
            i = result.1;
            c = input[i] as char;
            current_node.add_elem(result.0);
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
            current_node.add_elem(Elem{ is_list: false, val: Some(val), vals: None });
        }
    }
    return (current_node, i+1);
}
