use crate::parser::read_file;

pub fn main() {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut beacons = std::collections::HashSet::new();
    let coord_pairs = read_file(15, 1)
        .map(|l| (&l.unwrap())[10..].split(": closest beacon is at ").map(String::from).collect::<Vec<String>>())
        .map(|v| {
            let pair1 = v.get(0).unwrap().split(", ").collect::<Vec<&str>>();
            let pair2 = v.get(1).unwrap().split(", ").collect::<Vec<&str>>();
            let x1 = pair1.get(0).unwrap()[2..].parse::<i32>().unwrap();
            let y1 = pair1.get(1).unwrap()[2..].parse::<i32>().unwrap();
            let x2 = pair2.get(0).unwrap()[2..].parse::<i32>().unwrap();
            let y2 = pair2.get(1).unwrap()[2..].parse::<i32>().unwrap();
            min_x = std::cmp::min(x1, min_x);
            min_x = std::cmp::min(x2, min_x);
            max_x = std::cmp::max(x1, max_x);
            max_x = std::cmp::max(x2, max_x);
            beacons.insert((x2, y2));
            let dist = (x1-x2).abs() + (y1-y2).abs();
            return ((x1, y1), (x2, y2), dist);
        })
        .collect::<Vec<((i32,i32), (i32, i32), i32)>>();

    const Y: i32 = 2000000;
    let mut result = 0;
    (min_x..max_x+1)
        .for_each(|i| {
            if !beacons.contains(&(i, Y)) && coord_pairs.iter().any(|((x, y), _, dist)| ((i-x).abs() + (Y-y).abs()) <= *dist) {
                result +=1;
            }
        });
    println!("{result}");
}
