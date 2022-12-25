use crate::parser::read_file;

pub fn main() {
    let input = read_file(23, 1)
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let m = input.len();
    let n = input.get(0).unwrap().len();
    let mut elfs = vec![];
    for x in 0..m {
        for y in 0..n {
            if *input.get(x).unwrap().get(y).unwrap() == '#' {
                elfs.push((x as i32, y as i32));
            }
        }
    }

    for loop_num in 0..10usize {
        let mut moves  = std::collections::HashMap::<(i32, i32),Option<(&i32, &i32)>>::new();
        let mut elfs_map = std::collections::HashSet::new();
        elfs.iter().map(|x| *x).for_each(|x| {
            elfs_map.insert(x);
        });

        for (x, y) in elfs.iter() {
            let dirs = [
                [(-1, 0), (-1, 1), (-1, -1)],
                [(1, 0), (1, 1), (1, -1)],
                [(0, -1), (-1, -1), (1, -1)],
                [(0, 1), (-1, 1), (1, 1)],
            ];
            let mut done = false;
            [
                dirs[loop_num%4],
                dirs[(loop_num+1)%4],
                dirs[(loop_num+2)%4],
                dirs[(loop_num+3)%4],
            ].iter()
                .filter(|_| {
                    [(0,1),(0,-1),(1,1),(1,-1),(-1,1),(-1,-1),(1,0),(-1,0)].iter().any(|(dir_x, dir_y)| {
                        let new_x = x + dir_x;
                        let new_y = y + dir_y;
                        elfs_map.contains(&(new_x, new_y))
                    })
                })
                .filter(|dir| dir.iter().all(|(dir_x, dir_y)| {
                    let new_x = x + dir_x;
                    let new_y = y + dir_y;
                    !elfs_map.contains(&(new_x, new_y))
                }))
                .for_each(|dir| {
                    if done {
                        return;
                    }
                    let new_x = *x + dir[0].0;
                    let new_y = *y + dir[0].1;
                    if moves.contains_key(&(new_x, new_y)) {
                        moves.insert((new_x, new_y), None);
                    } else {
                        moves.insert((new_x, new_y), Some((x, y)));
                    }
                    done = true;
                });
        }
        for (k, v) in moves.iter() {
            if v.is_none() {
                continue;
            }

            if !elfs_map.insert((k.0, k.1)) {
                panic!()
            }
            if !elfs_map.remove(&(*v.unwrap().0, *v.unwrap().1)) {
                panic!()
            }
        }
        elfs = elfs_map.iter().map(|x| *x).collect::<Vec<(i32,i32)>>();
    }
    let mut bounds = [i32::MAX, i32::MIN, i32::MAX, i32::MIN];

    for (x, y) in elfs.iter() {
        bounds[0] = std::cmp::min(*x, bounds[0]);
        bounds[1] = std::cmp::max(*x, bounds[1]);
        bounds[2] = std::cmp::min(*y, bounds[2]);
        bounds[3] = std::cmp::max(*y, bounds[3]);
    }
    println!("{:?}", bounds);

    println!("{}", ((bounds[1] - bounds[0]) + 1) * ((bounds[3] - bounds[2]) + 1) - elfs.len() as i32);
}
