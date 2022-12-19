use crate::parser::read_file;

pub fn main() {
    read_file(19, 0)
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
            let clay_to_ore_ratio = match costs[2].get(1).unwrap().0 % costs[2].get(0).unwrap().0 {
                0 => costs[2].get(1).unwrap().0 /  costs[2].get(0).unwrap().0,
                _ => costs[2].get(1).unwrap().0 / costs[2].get(0).unwrap().0 + 1
            };
            let ore = costs[2].get(0).unwrap().0 + costs[3].get(0).unwrap().0;
            let obsidian = costs[3].get(1).unwrap().0;
            let obsidian_to_ore_ratio = match obsidian % ore {
                0 => obsidian / ore,
                _ => obsidian / ore + 1
            };
            println!("{},{}", clay_to_ore_ratio, obsidian_to_ore_ratio);
            // let mut memo = std::collections::HashMap::<(i32, i32, i32, i32), i32>::new();
            let result = simulate(&mut [0, 0, 0, 0], &mut [1, 0, 0, 0], &costs, 0, clay_to_ore_ratio, obsidian_to_ore_ratio);
            println!("{id}:{result}");
        });
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

fn simulate(resources: &mut [i32;4], robots: &mut [i32;4], costs: &[Vec<(i32, usize)>; 4], time: usize, clay_to_ore_ratio: i32, obsidian_to_ore_ratio: i32) -> i32 {
    if time >= 24 {
        return resources[3];
    }

    (0..4).for_each(|i| resources[i] += robots[i]);
    let mut result = simulate(resources, robots, costs, time+1, clay_to_ore_ratio, obsidian_to_ore_ratio);
    (0..4).for_each(|i| resources[i] -= robots[i]);
    for i in (0..4).rev() {
        if (i == 0 && robots[i] == 2) || (i == 1 && robots[i] / robots[0] == clay_to_ore_ratio) || (i == 2 && robots[i] / robots[0] == obsidian_to_ore_ratio) {
            continue
        }
        let can_build = costs[i].iter().all(|(amt, resource_idx)| resources[*resource_idx] >= *amt);
        if can_build {
            costs[i].iter().for_each(|(amt, resource_idx)| resources[*resource_idx] -= amt);
            (0..4).for_each(|j| resources[j] += robots[j]);
            robots[i] += 1;
            result = std::cmp::max(result, simulate(resources, robots, costs, time+1, clay_to_ore_ratio, obsidian_to_ore_ratio));
            robots[i] -= 1;
            (0..4).for_each(|j| resources[j] -= robots[j]);
            costs[i].iter().for_each(|(amt, resource_idx)| resources[*resource_idx] += amt);
        }
    }

    return result;
}
