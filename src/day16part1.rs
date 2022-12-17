use std::collections::{HashMap, HashSet};

use crate::parser::read_file;

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
    let start = "AA".to_string();
    visited.insert("AA".to_string());
    let result = solve(&start, &rooms, &distances, &to_visit, 0, 0, 0, &mut visited);
    println!("{result}");
}



fn solve(valve: &String, rooms: &HashMap<String, (usize, Vec<String>)>, distances: &HashMap<(&String, &String), usize>, to_visit: &Vec<String>, curr_minute: usize, curr_flow: usize, total_flow: usize, visited: &mut HashSet<String>) -> usize {
    if curr_minute >= 30 {
        return total_flow;
    }
    let remaining_time = 30 - curr_minute;
    if visited.len() == to_visit.len() {
        return total_flow + remaining_time * curr_flow;
    }
    let mut result = total_flow;
    for v in to_visit.iter() {
        if visited.contains(v) {
            continue;
        }
        let cost = distances.get(&(valve, &v)).unwrap();
        if remaining_time <= *cost {
            result = std::cmp::max(result, total_flow + (30 - curr_minute) * curr_flow);
        } else {
            visited.insert(v.clone());
            let flow = rooms.get(v).unwrap().0;
            result = std::cmp::max(result, solve(&v, rooms, distances, to_visit, curr_minute+cost+1, curr_flow+flow, total_flow+(curr_flow*(cost+1)), visited));
            visited.remove(v);
        }
    }
    return result;
}
