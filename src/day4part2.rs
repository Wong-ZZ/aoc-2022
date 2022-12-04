use crate::parser::read_file;

pub fn main() {
    let result = read_file(4, 1)
        .map(|l| l.unwrap())
        .map(|l|
            l
                .split(',')
                .flat_map(|l2| l2.split('-').collect::<Vec<&str>>())
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        )
        .filter(|v| {
            if let [start1, end1, start2, end2] = v[0..4] {
                return (start1 <= end2 && start1 >= start2) || (end1 >= start2 && end1 <= end2) || (start1 <= start2 && end1 >= end2);
            } else {
                panic!();
            }
        })
        .count();
    println!("{result}");
}
