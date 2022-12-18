use crate::parser::read_file;

pub fn main() {
    let mut cubes = std::collections::HashSet::new();
    let mut covered = 0;
    read_file(18, 1)
        .for_each(|l| {
            let line = l.unwrap().split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>();
            if let [x, y, z] = &line[0..3] {
                [(-1, 0, 0), (1, 0, 0), (0, 1, 0), (0, -1, 0), (0, 0, -1), (0, 0, 1)].iter()
                    .for_each(|(dir_x, dir_y, dir_z)| {
                        if cubes.contains(&(*x + *dir_x, *y + *dir_y, *z + *dir_z)) {
                            covered += 2;
                        }
                    });
                cubes.insert((*x, *y, *z));
            } else {
                panic!();
            }
        });
    println!("{}", cubes.len() * 6 - covered)
}
