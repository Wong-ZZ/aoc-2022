use crate::parser::read_file;

pub fn main() {
    let mut cubes = std::collections::HashSet::new();
    let mut max_bound_x = i32::MIN;
    let mut max_bound_y = i32::MIN;
    let mut max_bound_z = i32::MIN;
    let mut min_bound_x = i32::MAX;
    let mut min_bound_y = i32::MAX;
    let mut min_bound_z = i32::MAX;
    read_file(18, 1)
        .for_each(|l| {
            let line = l.unwrap().split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            if let [x, y, z] = line[0..3] {
                max_bound_x = std::cmp::max(x, max_bound_x);
                max_bound_y = std::cmp::max(y, max_bound_y);
                max_bound_z = std::cmp::max(z, max_bound_z);
                min_bound_x = std::cmp::min(x, min_bound_x);
                min_bound_y = std::cmp::min(y, min_bound_y);
                min_bound_z = std::cmp::min(z, min_bound_z);
                cubes.insert((x, y, z));
            } else {
                panic!();
            }
        });
    max_bound_x += 1;
    max_bound_y += 1;
    max_bound_z += 1;
    min_bound_x -= 1;
    min_bound_y -= 1;
    min_bound_z -= 1;
    let mut result = 0;
    let mut stack = vec![(max_bound_x, max_bound_y, max_bound_z)];
    let mut visited = std::collections::HashSet::new();
    visited.insert((max_bound_x, max_bound_y, max_bound_z));
    while !stack.is_empty() {
        let (x, y, z) = stack.pop().unwrap();
        [(-1, 0, 0), (1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, -1), (0, 0, 1)].iter()
            .for_each(|(dir_x, dir_y, dir_z)| {
                let new_x = x + dir_x;
                let new_y = y + dir_y;
                let new_z = z + dir_z;
                if new_x >= min_bound_x && new_x <= max_bound_x && new_y >= min_bound_y && new_y <= max_bound_y && new_z >= min_bound_z && new_z <= max_bound_z && !visited.contains(&(new_x, new_y, new_z)) {
                    if cubes.contains(&(new_x, new_y, new_z)) {
                        result += 1;
                    } else {
                        visited.insert((new_x, new_y, new_z));
                        stack.push((new_x, new_y, new_z));
                    }
                }
            });
    }
    println!("{result}");
}
