use std::collections::HashMap;

use crate::parser::read_file;

pub fn main() {
    let mut result = 0;
    read_file(19, 1)
        .for_each(|l| {
            let line = l.unwrap();
            let mut line = line.split(": ").map(String::from);

            let id = &line.next().unwrap()[10..].parse::<i32>().unwrap();
            let mut costs = [vec![], vec![], vec![], vec![]];
            line.next().unwrap()
                .split(".")
                .map(|s| s.trim())
                .filter(|s| !(*s).is_empty())
                .map(String::from)
                .map(|s| String::from(&s[5..]).split(" robot costs ").map(String::from).collect::<Vec<String>>())
                .for_each(|data| {
                    let robot_type = get_resource_idx(data.get(0).unwrap());
                    let cost = data.get(1).unwrap()
                        .split(" and ")
                        .map(|s| s.split(' ').map(String::from).collect::<Vec<String>>())
                        .map(|s| (s.get(0).unwrap().parse::<i32>().unwrap(), get_resource_idx(s.get(1).unwrap())))
                        .collect::<Vec<(i32, usize)>>();
                    costs[robot_type] = cost;
                });
            println!("{:?}", costs);
            let mut memo = HashMap::<((i32, i32, i32, i32), (i32, i32, i32, i32)), (usize, (i32, i32, i32, i32))>::new();
            let temp = simulate((0, 0, 0, 0), &mut [0, 0, 0, 0], &mut [1, 0, 0, 0], &costs, 0, &mut memo);
            println!("{}", memo.len());
            println!("{id}:{:?}", temp);
            result += id * temp.0;
        });
    println!("{result}")
}

fn get_resource_idx(resource: &String) -> usize {
    match resource.as_str() {
        "ore" => 0,
        "clay" => 1,
        "obsidian" => 2,
        "geode" => 3,
        _ => panic!()
    }
}

fn simulate(before_state: (i32, i32, i32, i32), resources: &mut [i32;4], robots: &mut [i32;4], costs: &[Vec<(i32, usize)>; 4], time: usize, memo: &mut HashMap::<((i32, i32, i32, i32), (i32, i32, i32, i32)), (usize, (i32, i32, i32, i32))>) -> (i32, (i32, i32, i32, i32)) {
    let current_state = (robots[0], robots[1], robots[2], robots[3]);
    if time >= 24 {
        // if resources[3] == 0 {
        //     println!("{time}: {:?}, {:?},{:?}, {}", resources, before_state, robots, memo.contains_key(&(before_state, current_state)))
        // }
        // if memo.contains_key(&(before_state, current_state)) {
        //     memo.insert((before_state, current_state), (time, resources[3]));
        // }
        // }
        return (resources[3], (robots[0], robots[1], robots[2], robots[3]));
    }

    if current_state != before_state && memo.contains_key(&(before_state, current_state))  {
        if memo.get(&(before_state, current_state)).unwrap().0 < time || (memo.get(&(before_state, current_state)).unwrap().0 == time && memo.get(&(before_state, current_state)).unwrap().1 >= (resources[3], resources[2], resources[1], resources[0])) {
            return (0, (0, 0, 0, 0));
        }
    }

    let mut result = (0, (0, 0, 0, 0));
    if current_state != before_state {
        memo.insert((before_state, current_state), (time, (resources[3], resources[2], resources[1], resources[0])));
    }
    for i in 0..4 {
        let can_build = costs[i].iter().all(|(amt, resource_idx)| resources[*resource_idx] >= *amt);
        if can_build {
            costs[i].iter().for_each(|(amt, resource_idx)| resources[*resource_idx] -= amt);
            (0..4).for_each(|j| resources[j] += robots[j]);
            robots[i] += 1;
            result = std::cmp::max(result, simulate(current_state, resources, robots, costs, time+1, memo));
            robots[i] -= 1;
            (0..4).for_each(|j| resources[j] -= robots[j]);
            costs[i].iter().for_each(|(amt, resource_idx)| resources[*resource_idx] += amt);
        }
    }

    (0..4).for_each(|i| resources[i] += robots[i]);
    result = std::cmp::max(result, simulate(current_state, resources, robots, costs, time+1, memo));
    (0..4).for_each(|i| resources[i] -= robots[i]);

    return result;
}
