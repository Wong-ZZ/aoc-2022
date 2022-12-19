use crate::parser::read_file;

pub fn main() {
    let mut visited = std::collections::HashSet::new();
    let mut coords = [(0,0); 10];
    let mut prev_coords = [(0,0); 10];
    visited.insert((0, 0));
    let mut dirs = std::collections::HashMap::new();
    dirs.insert("L".to_string(), (0, -1));
    dirs.insert("R".to_string(), (0, 1));
    dirs.insert("U".to_string(), (-1, 0));
    dirs.insert("D".to_string(), (1, 0));
    read_file(9, 1)
        .map(|l| l.unwrap().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|v| (v.get(0).unwrap().clone(), v.get(1).unwrap().parse::<i32>().unwrap()))
        .for_each(|(cmd, steps)| {
            let (dir_x, dir_y) = dirs.get(&cmd).unwrap();
            (0..steps)
                .for_each(|_| {
                    let (h_x, h_y) = coords[0];
                    prev_coords[0] = (h_x, h_y);
                    coords[0] = (h_x + dir_x, h_y + dir_y);
                    let mut prev_dir = (0, 0);
                    (1..10)
                        .for_each(|i| {
                            let (h_x, h_y) = coords[i-1];
                            prev_coords[i] = coords[i];
                            let (t_x, t_y) = coords[i];
                            match (((h_x - t_x) as i32).abs(), ((h_y - t_y) as i32).abs()) {
                                (1, 1) => (),
                                (0, 1) => (),
                                (1, 0) => (),
                                (0, 0) => (),
                                (a, b) => {
                                    if a * b == 0 {
                                        coords[i] = (t_x + (h_x - t_x) / 2, t_y + (h_y - t_y) / 2);
                                    } else if (prev_dir.0 * prev_dir.1) == 0 {
                                        coords[i] = prev_coords[i-1];
                                    } else {
                                        coords[i] = (t_x + prev_dir.0, t_y + prev_dir.1);
                                    }
                                    let new_dir = ((coords[i].0 - prev_coords[i].0), (coords[i].1 - prev_coords[i].1));
                                    prev_dir = new_dir;
                                }
                            }
                        });
                    // if cmd == "L".to_string() && steps == 8 {
                    //     let mut debug = [[',';40];40];
                    //     coords.iter().for_each(|(i, j)| debug[(*i + 20) as usize][(*j + 20) as usize] = '#');
                    //     debug.iter().for_each(|r| println!("{:?}", r));
                    //     println!("");
                    // }
                    visited.insert(coords[9]);
                });
        });
    println!("{}", visited.len());
    // print!("{:?}", visited);
    // let mut temp = visited.iter().collect::<Vec<&(i32, i32)>>();
    // temp.sort();
    // print!("{:?}", temp);
}
