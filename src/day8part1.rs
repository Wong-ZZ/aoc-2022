use crate::parser::read_file;

pub fn main() {
    let grid = read_file(8, 1)
        .map(|l| l.unwrap().chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    let n = grid.len();
    let m = grid.get(0).unwrap().len();
    let mut result = std::collections::HashSet::new();

    for (i, v) in grid.iter().enumerate() {
        let mut max_so_far = -1;
        for (j, height) in v.iter().enumerate() {
            // if max_so_far >= *n {
            //     result.remove(&(i as i32, j as i32));
            // }
            if *height > max_so_far {
                result.insert((i as i32, j as i32));
            }
            max_so_far = std::cmp::max(max_so_far, *height);
        }
    }


    for (i, v) in grid.iter().enumerate() {
        let mut max_so_far = -1;
        for (temp, height) in v.iter().rev().enumerate() {
            let j = m - temp - 1;
            // if max_so_far >= *n {
            //     result.remove(&(i as i32, j as i32));
            // }
            if *height > max_so_far {
                result.insert((i as i32, j as i32));
            }
            max_so_far = std::cmp::max(max_so_far, *height);
        }
    }

    // from top
    for j in 0..m {
        let mut max_so_far = -1;
        for i in 0..n {
            let height = grid.get(i).unwrap().get(j).unwrap();
            // if max_so_far >= *n {
            //     result.remove(&(i as i32, j as i32));
            // }
            if *height > max_so_far {
                result.insert((i as i32, j as i32));
            }
            max_so_far = std::cmp::max(max_so_far, *height);
        }
    }

    // from btm
    for j in 0..m {
        let mut max_so_far = -1;
        for temp in 0..n {
            let i = n - temp - 1;
            let height = grid.get(i).unwrap().get(j).unwrap();
            // if max_so_far >= *n {
            //     result.remove(&(i as i32, j as i32));
            // }
            if *height > max_so_far {
                result.insert((i as i32, j as i32));
            }
            max_so_far = std::cmp::max(max_so_far, *height);
        }
    }

    println!("{}", result.len());
    // println!("{:?}", result);
}
