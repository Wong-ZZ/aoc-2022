use crate::parser::read_file;

pub fn main() {
    let mut grid = read_file(12, 1)
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let m = grid.len() as i32;
    let n = grid.get(0).unwrap().len() as i32;
    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..m {
        for j in 0..n {
            let c = grid.get(i as usize).unwrap().get(j as usize).unwrap();
            match *c {
                'S' => start = (i, j),
                'E' => end = (i, j),
                _ => ()
            }
        }
    }
    println!("{:?}", start);
    println!("{:?}", end);
    grid.get_mut(start.0 as usize).unwrap()[start.1 as usize] = 'a';
    grid.get_mut(end.0 as usize).unwrap()[end.1 as usize] = 'z';


    let mut visited = std::collections::HashSet::new();
    visited.insert(start);
    let mut frontier = vec![(start, 1)];
    'outer: loop {
        let mut temp = vec![];
        for (pt, cost) in frontier.iter() {
            let c = *grid.get(pt.0 as usize).unwrap().get(pt.1 as usize).unwrap() as i16;
            for (dir_x, dir_y) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let (new_x, new_y) = (pt.0 + dir_x, pt.1 + dir_y);
                if new_x >= 0 && new_x < m && new_y >= 0 && new_y < n && !visited.contains(&(new_x, new_y)) {
                    let other_c = *grid.get(new_x as usize).unwrap().get(new_y as usize).unwrap() as i16;
                    if other_c - c <= 1 {
                        if (new_x, new_y) == end {
                            println!("{cost}");
                            break 'outer;
                        }
                        visited.insert((new_x, new_y));
                        temp.push(((new_x, new_y), cost+1));
                    }
                }
            }
        }
        if temp.is_empty() {
            break;
        }
        frontier = temp;
    }
}
