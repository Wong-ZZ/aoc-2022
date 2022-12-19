use crate::parser::read_file;

pub fn main() {
    let mut visited = std::collections::HashSet::new();
    let mut h_x = 0;
    let mut h_y = 0;
    let mut t_x = 0;
    let mut t_y = 0;
    visited.insert((t_x, t_y));
    let mut dirs = std::collections::HashMap::new();
    dirs.insert("L".to_string(), (-1, 0));
    dirs.insert("R".to_string(), (1, 0));
    dirs.insert("U".to_string(), (0, 1));
    dirs.insert("D".to_string(), (0, -1));
    read_file(9, 1)
        .map(|l| l.unwrap().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>())
        .map(|v| (v.get(0).unwrap().clone(), v.get(1).unwrap().parse::<i32>().unwrap()))
        .for_each(|(cmd, steps)| {
            let (dir_x, dir_y) = dirs.get(&cmd).unwrap();
            (0..steps)
                .for_each(|_| {
                    let (temp_x, temp_y) = (h_x, h_y);
                    h_x += dir_x;
                    h_y += dir_y;
                    match (((h_x - t_x) as i32).abs(), ((h_y - t_y) as i32).abs()) {
                        (1, 1) => (),
                        (0, 1) => (),
                        (1, 0) => (),
                        (0, 0) => (),
                        _ => {
                            (t_x, t_y) = (temp_x, temp_y);
                            visited.insert((t_x, t_y));
                        }
                    }
                });
        });
    println!("{}", visited.len());
    // let mut temp = visited.iter().collect::<Vec<&(i32, i32)>>();
    // temp.sort();
    // print!("{:?}", temp);
}
