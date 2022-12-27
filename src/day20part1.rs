use crate::parser::read_file;


struct Node {
    val: i32,
    idx: usize,
    prev: Option<usize>,
    next: Option<usize>
}

pub fn main() {
    let mut zero_node_idx = 0;
    let mut nodes = vec![];
    read_file(20, 1)
        .map(|l| l.unwrap().parse::<i32>().unwrap())
        .enumerate()
        .for_each(|(idx, num)| {
            let node = Node { val: num, idx, prev: None, next: None };
            if num == 0 {
                zero_node_idx = idx;
            }
            nodes.push(node);
        });
    let len = nodes.len();
    (1..len-1)
        .for_each(|i| {
            nodes.get_mut(i).unwrap().next = Some(i+1);
            nodes.get_mut(i).unwrap().prev = Some(i-1);
        });
    nodes.get_mut(len-1).unwrap().next = Some(0);
    nodes.get_mut(len-1).unwrap().prev = Some(len-2);
    nodes.get_mut(0).unwrap().next = Some(1);
    nodes.get_mut(0).unwrap().prev = Some(len-1);
    (0..len)
        .for_each(|i| {
            let curr_node = nodes.get(i).unwrap();
            let val = curr_node.val;
            let is_negative = val < 0;
            if val == 0 {
                return;
            }
            let mut other_node_idx = i;
            let mut j = 0;
            while j < val.abs() {
                if is_negative {
                    other_node_idx = nodes.get(other_node_idx).unwrap().prev.unwrap();
                } else {
                    other_node_idx = nodes.get(other_node_idx).unwrap().next.unwrap();
                }
                if other_node_idx != i {
                    j += 1;
                }
            }
            (0..(val.abs()))
                .for_each(|_| {
                });
            let curr_node_idx = i;
            let other_node_prev_idx = nodes.get(other_node_idx).unwrap().prev.unwrap();
            let other_node_next_idx = nodes.get(other_node_idx).unwrap().next.unwrap();
            let curr_node_prev_idx = curr_node.prev.unwrap();
            let curr_node_next_idx = curr_node.next.unwrap();

            if !is_negative {
                if nodes.get(other_node_idx).unwrap().idx == i || nodes.get(other_node_next_idx).unwrap().idx == i {
                    return;
                }

                nodes.get_mut(curr_node_prev_idx).unwrap().next = Some(curr_node_next_idx);
                nodes.get_mut(curr_node_next_idx).unwrap().prev = Some(curr_node_prev_idx);
                nodes.get_mut(other_node_idx).unwrap().next = Some(curr_node_idx);
                nodes.get_mut(curr_node_idx).unwrap().prev = Some(other_node_idx);
                nodes.get_mut(curr_node_idx).unwrap().next = Some(other_node_next_idx);
                nodes.get_mut(other_node_next_idx).unwrap().prev = Some(curr_node_idx);
            } else {
                if nodes.get(other_node_idx).unwrap().idx == i || nodes.get(other_node_prev_idx).unwrap().idx == i {
                    return;
                }

                nodes.get_mut(curr_node_prev_idx).unwrap().next = Some(curr_node_next_idx);
                nodes.get_mut(curr_node_next_idx).unwrap().prev = Some(curr_node_prev_idx);
                nodes.get_mut(other_node_prev_idx).unwrap().next = Some(curr_node_idx);
                nodes.get_mut(curr_node_idx).unwrap().prev = Some(other_node_prev_idx);
                nodes.get_mut(curr_node_idx).unwrap().next = Some(other_node_idx);
                nodes.get_mut(other_node_idx).unwrap().prev = Some(curr_node_idx);
            }
        });
    let mut result = 0;
    let mut curr_idx = zero_node_idx;
    (0..3)
        .for_each(|_| {
            (0..1000)
                .for_each(|_| {
                    curr_idx = nodes.get(curr_idx).unwrap().next.unwrap();
                });
            let n = nodes.get(curr_idx).unwrap().val;
            println!("{n}");
            result += n;
        });


    let mut debug = [-1;7];
    let mut curr_idx = zero_node_idx;
    (0..7)
        .for_each(|i| {
            debug[i] = nodes.get(curr_idx).unwrap().val;
            curr_idx = nodes.get(curr_idx).unwrap().next.unwrap();
        });
    println!("{:?}", debug);


    println!("len: {len}");
    println!("result: {result}");
}
