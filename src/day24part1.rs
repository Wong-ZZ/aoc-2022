use crate::parser::read_file;

pub fn main() {
    let mut blizzards = vec![];
    let input = read_file(24, 1)
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .filter(|v| !v.is_empty())
        .collect::<Vec<Vec<char>>>();
    let m = input.len() as i32;
    let n = input.get(0).unwrap().len() as i32;
    println!("{m},{n}");
    for x in 0..m {
        for y in 0..n {
            let c = *input.get(x as usize).unwrap().get(y as usize).unwrap();
            match c {
                'v' | '>' | '<' | '^' => blizzards.push((c, [x as i32, y as i32])),
                _ => ()
            }
        }
    }
    let mut possible_positions = std::collections::HashSet::new();
    possible_positions.insert((0, 1));
    let end = (m - 1, n - 2);
    for i in 1..20000 {
        println!("min {i} {}", possible_positions.len());
        let mut flag = false;
        let set = step(&mut blizzards, m, n);
        let mut next_pos = std::collections::HashSet::new();
        possible_positions
            .iter()
            .for_each(|(x, y)| {
                for (dir_x, dir_y) in [(0, 1), (0, -1), (-1, 0), (1, 0), (0, 0)].iter() {
                    let new_x = *x + *dir_x;
                    let new_y = *y + *dir_y;
                    if (new_x, new_y) == end {
                        flag = true;
                        println!("{i}");
                    }
                    if ((new_x >= 1 && new_x <= m-2 && new_y >= 1 && new_y <= n-2) || (new_x, new_y) == (0, 1)) && !set.contains(&(new_x, new_y)) {
                        next_pos.insert((new_x, new_y));
                    }
                }
            });
        possible_positions = next_pos;
        if flag {
            break;
        }
    }
}

fn step(blizzards: &mut Vec<(char, [i32;2])>, m: i32, n: i32) -> std::collections::HashSet::<(i32, i32)> {
    let mut set = std::collections::HashSet::new();
    // let mut grid = [['.'; 8];6];
    for i in 0..blizzards.len() {
        let (c, pos) = blizzards.get_mut(i).unwrap();
        let mut new_x = pos[0];
        let mut new_y = pos[1];
        match c {
            'v' => new_x += 1,
            '^' => new_x -= 1,
            '>' => new_y += 1,
            '<' => new_y -= 1,
            _ => unreachable!()
        }
        if new_x == 0 {
            new_x = m - 2;
        } else if new_x == m - 1 {
            new_x = 1;
        } else if new_y == 0 {
            new_y = n - 2;
        } else if new_y == n - 1 {
            new_y = 1;
        }
        pos[0] = new_x;
        pos[1] = new_y;
        set.insert((new_x, new_y));
        // grid[new_x as usize][new_y as usize] = *c;
    }
    // grid.iter().for_each(|r| println!("{:?}", r));
    // println!();
    set
}
