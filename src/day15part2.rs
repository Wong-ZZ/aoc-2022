use crate::parser::read_file;

pub fn main() {
    let mut beacons = std::collections::HashSet::new();
    let mut candidates = vec![];
    let coord_pairs = read_file(15, 1)
        .map(|l| (&l.unwrap())[10..].split(": closest beacon is at ").map(String::from).collect::<Vec<String>>())
        .map(|v| {
            let pair1 = v.get(0).unwrap().split(", ").collect::<Vec<&str>>();
            let pair2 = v.get(1).unwrap().split(", ").collect::<Vec<&str>>();
            let x1 = pair1.get(0).unwrap()[2..].parse::<i64>().unwrap();
            let y1 = pair1.get(1).unwrap()[2..].parse::<i64>().unwrap();
            let x2 = pair2.get(0).unwrap()[2..].parse::<i64>().unwrap();
            let y2 = pair2.get(1).unwrap()[2..].parse::<i64>().unwrap();
            beacons.insert((x2, y2));
            let dist = (x1-x2).abs() + (y1-y2).abs() + 1;
            let left = (x1-dist, y1);
            let top = (x1, y1-dist);
            let right = (x1+dist, y1);
            let bottom = (x1, y1+dist);
            for ((mut x, mut y), end, (dir_x, dir_y)) in [(left, top, (1, -1)), (top, right, (1, 1)), (right, bottom, (-1, 1)), (bottom, left, (-1, -1))].iter() {
                while (x, y) != *end {
                    candidates.push((x, y));
                    x += dir_x;
                    y += dir_y;
                }
            }
            return ((x1, y1), (x2, y2), dist-1)
        })
        .collect::<Vec<((i64,i64), (i64, i64), i64)>>();

    for (x1, y1) in candidates.iter() {
        if *x1 < 0 || *x1 > 4000000 || *y1 < 0 || *y1 > 4000000 {
            continue;
        }
        if !beacons.contains(&(*x1, *y1)) && coord_pairs.iter().all(|((x2, y2), _, dist)| ((x1-x2).abs() + (y1-y2).abs()) > *dist) {
            println!("{x1}, {y1}");
            println!("{}", x1*4000000 + y1);
            break;
        }
    }
}
