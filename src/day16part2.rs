use std::collections::{HashMap, HashSet};

use crate::parser::read_file;

// pure brute force
pub fn main() {
    let mut rooms = HashMap::new();
    let mut to_visit = vec![];
    read_file(16, 1)
        .for_each(|l| {
            let line = l.unwrap();
            let mut line = if line.contains("; tunnels lead to valves ") {
                line.split("; tunnels lead to valves ")
            } else {
                line.split("; tunnel leads to valve ")
            };
            let mut valve = line.next().unwrap().split(" has flow rate=");
            let valve_name = String::from(&valve.next().unwrap()[6..]);
            let flow = valve.next().unwrap().parse::<usize>().unwrap();
            let next_valves = line.next().unwrap().split(", ").map(String::from).collect::<Vec<String>>();
            if flow > 0 {
                to_visit.push(valve_name.clone());
            }
            rooms.insert(valve_name, (flow, next_valves));
        });
    to_visit.push("AA".to_string());

    let mut distances = HashMap::new();
    for i in 0..to_visit.len() {
        let curr = to_visit.get(i).unwrap();
        let mut queue = vec![curr];
        let mut visited = HashSet::new();
        visited.insert(curr);
        let mut i: usize = 1;
        while !queue.is_empty() {
            let mut temp = vec![];
            for &v in queue.iter() {
                for valve in rooms.get(v).unwrap().1.iter() {
                    if !visited.contains(valve) {
                        visited.insert(valve);
                        distances.insert((curr, valve), i);
                        temp.push(valve);
                    }
                }
            }
            queue = temp;
            i += 1;
        }
    }

    let mut visited = HashSet::new();
    visited.insert("AA".to_string());
    println!("{:?}", distances);
    let result = solve(&rooms, &distances, &to_visit, 0, 0, 0, &mut visited, &mut [(0, 0, "AA".to_string()), (0, 0, "AA".to_string())]);
    println!("{result}");
}



fn solve(rooms: &HashMap<String, (usize, Vec<String>)>, distances: &HashMap<(&String, &String), usize>, to_visit: &Vec<String>, curr_minute: usize, curr_flow: usize, total_flow: usize, visited: &mut HashSet<String>, next_ready_time: &mut [(usize, usize, String); 2]) -> usize {
    if curr_minute >= 26 {
        return total_flow;
    }
    let remaining_time = 26 - curr_minute;
    let i = match next_ready_time[0].0 <= next_ready_time[1].0 {
        true => 0,
        false => 1
    };
    let valve = &next_ready_time[i].2.clone();
    let new_curr_flow = curr_flow + next_ready_time[i].1;
    if visited.len() == to_visit.len() {
        let step2 = next_ready_time[i^1].0;
        let flow2 = next_ready_time[i^1].1;

        return total_flow + new_curr_flow*(step2-curr_minute) + (new_curr_flow+flow2)*(26-step2);
    }
    let mut result = total_flow;
    for v in to_visit.iter() {
        if next_ready_time[0].0 == 0 && next_ready_time[0].1 == 0 && next_ready_time[1].0 == 0 && next_ready_time[1].1 == 0 {
            println!("{v}");
        }
        if visited.contains(v) || v == valve {
            continue;
        }
        let cost = distances.get(&(valve, &v)).unwrap();
        if remaining_time <= *cost {
            let temp = (next_ready_time[i].0, next_ready_time[i].1, next_ready_time[i].2.clone());
            next_ready_time[i] = (26, 0, "".to_string());
            let next_step = std::cmp::min(next_ready_time[0].0, next_ready_time[1].0);
            result = std::cmp::max(result, solve(rooms, distances, to_visit, next_step, new_curr_flow, total_flow+(new_curr_flow*(next_step-curr_minute)), visited, next_ready_time));
            next_ready_time[i] = temp;
            // result = std::cmp::max(result, total_flow + (remaining_time) * curr_flow);
        } else {
            visited.insert(v.clone());
            let flow = rooms.get(v).unwrap().0;
            let temp = (next_ready_time[i].0, next_ready_time[i].1, next_ready_time[i].2.clone());
            next_ready_time[i] = (curr_minute+cost+1, flow, String::from(v));
            let next_step = std::cmp::min(next_ready_time[0].0, next_ready_time[1].0);
            result = std::cmp::max(result, solve(rooms, distances, to_visit, next_step, new_curr_flow, total_flow+(new_curr_flow*(next_step-curr_minute)), visited, next_ready_time));
            next_ready_time[i] = temp;
            visited.remove(v);
        }
    }

    return result;
}
