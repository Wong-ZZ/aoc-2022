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
            // let clay_to_ore_ratio = (costs[2].get(1).unwrap().0, costs[2].get(0).unwrap().0);
            let ore_needed = [
                costs[0].get(0).unwrap().0,
                costs[1].get(0).unwrap().0,
                costs[2].get(0).unwrap().0,
                costs[3].get(0).unwrap().0,
            ];
            let clay_needed = costs[2].get(1).unwrap().0;
            let obsidian_needed = costs[3].get(1).unwrap().0;
            let mut resources = [0, 0, 0, 0];
            let mut robots = [1, 0, 0, 0];
            let mut result = 0;
            let mut build_obsidian_min = i32::MAX;
            let mut build_geode_min = i32::MAX;
            'outer: for minute in 0..25 {
                println!("1. {minute}:{build_geode_min}");
                println!("2. {minute}:{build_obsidian_min}");
                if minute >= build_geode_min && costs[3].iter().all(|(amt, resource_idx)| resources[*resource_idx] >= *amt) {
                    println!("a");
                    costs[3].iter().for_each(|(amt, resource_idx)| resources[*resource_idx] -= amt);
                    update_resource(&mut resources, &mut robots);
                    result += robots[3];
                    robots[3] += 1;
                    update_est(minute, clay_needed, obsidian_needed, &mut build_obsidian_min, &mut build_geode_min, &resources, &robots);
                } else if minute >= build_obsidian_min && costs[2].iter().all(|(amt, resource_idx)| resources[*resource_idx] >= *amt) {
                    println!("b");
                    if resources[0] - ore_needed[2] + (build_geode_min - minute) * robots[0] < ore_needed[3] {
                        update_resource(&mut resources, &mut robots);
                        update_est(minute, clay_needed, obsidian_needed, &mut build_obsidian_min, &mut build_geode_min, &resources, &robots);
                        continue;
                    }
                    costs[2].iter().for_each(|(amt, resource_idx)| resources[*resource_idx] -= amt);
                    update_resource(&mut resources, &mut robots);
                    result += robots[3];
                    robots[2] += 1;
                    update_est(minute, clay_needed, obsidian_needed, &mut build_obsidian_min, &mut build_geode_min, &resources, &robots);
                } else if costs[1].iter().all(|(amt, resource_idx)| resources[*resource_idx] >= *amt) {
                    println!("c");
                    if resources[0] - ore_needed[1] + (build_obsidian_min - minute) * robots[0] < ore_needed[2] {
                        update_resource(&mut resources, &mut robots);
                        update_est(minute, clay_needed, obsidian_needed, &mut build_obsidian_min, &mut build_geode_min, &resources, &robots);
                        continue;
                    }
                    if resources[0] - ore_needed[1] + (build_geode_min - minute) * robots[0] < ore_needed[3] {
                        update_resource(&mut resources, &mut robots);
                        update_est(minute, clay_needed, obsidian_needed, &mut build_obsidian_min, &mut build_geode_min, &resources, &robots);
                        continue;
                    }
                    costs[1].iter().for_each(|(amt, resource_idx)| resources[*resource_idx] -= amt);
                    update_resource(&mut resources, &mut robots);
                    result += robots[3];
                    robots[1] += 1;
                    update_est(minute, clay_needed, obsidian_needed, &mut build_obsidian_min, &mut build_geode_min, &resources, &robots);
                } else {
                    println!("d");
                    result += robots[3];
                    update_resource(&mut resources, &mut robots);
                    update_est(minute, clay_needed, obsidian_needed, &mut build_obsidian_min, &mut build_geode_min, &resources, &robots);
                }

                println!("{:?}", resources);
                println!("{:?}", robots);
            }
            println!("{result}");
        });
}

fn update_resource(resources: &mut [i32;4], robots: &mut [i32;4]) {
    (0..4).for_each(|j| resources[j] += robots[j]);
}

fn update_est(minute: i32, clay_needed: i32, obsidian_needed: i32, ob: &mut i32, geo: &mut i32, resources: &[i32;4], robots: &[i32;4]) {
    if robots[1] > 0 {
        *ob = match (clay_needed - resources[1]) % robots[1] {
            0 => minute + (clay_needed - resources[1]) / robots[1],
            _ => minute + (clay_needed - resources[1]) / robots[1] + 1
        };
    }
    if robots[2] > 0 {
        *geo = match (obsidian_needed - resources[2]) % robots[2] {
            0 => minute + (obsidian_needed - resources[2]) / robots[2],
            _ => minute + (obsidian_needed - resources[2]) / robots[2] + 1
        };
    }
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