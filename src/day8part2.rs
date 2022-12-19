use crate::parser::read_file;

pub fn main() {
    let grid = read_file(8, 1)
        .map(|l| l.unwrap().chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();
    let n = grid.len();
    let m = grid.get(0).unwrap().len();
    let mut result = 0;
    (0..n)
        .for_each(|i| {
            for j in 0..m {
                let h = *grid.get(i).unwrap().get(j).unwrap();
                let mut temp = 1;
                for (dir_i, dir_j) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let mut x = i.clone() as i32 + dir_i;
                    let mut y = j.clone() as i32 + dir_j;
                    let mut count = 0;
                    while x >= 0 && x < n as i32 && y >= 0 && y < m as i32 {
                        count += 1;
                        let curr = grid.get(x as usize).unwrap().get(y as usize).unwrap();
                        if *curr >= h {
                            if i == 3 && j == 2 {
                                println!("tesT:{count}");
                            }
                            temp *= count;
                            count = -1;
                            break;
                        }
                        x += dir_i;
                        y += dir_j;
                    }
                    if count != -1 {
                        if i == 3 && j == 2 {
                            println!("tesT:{count}");
                        }
                        temp *= count;
                    }
                    // println!("{temp}");
                }
                result = std::cmp::max(result, temp);
            }
        });

    println!("{}", result);
}
