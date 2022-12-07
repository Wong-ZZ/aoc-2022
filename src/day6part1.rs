use crate::parser::read_file;

pub fn main() {
    let mut iter = read_file(6, 1)
        .map(|l| l.unwrap());
    let line = iter.next().unwrap().chars().collect::<Vec<char>>();
    let n = 4;
    let mut counter = std::collections::HashMap::new();
    (0..n)
        .map(|i| line.get(i).unwrap())
        .for_each(|c| {
            if !counter.contains_key(c) {
                counter.insert(c, 0);
            }
            *counter.get_mut(c).unwrap() += 1;
        });

    (n..line.len())
        .map(|i| {
            let temp = counter.len();
            let c = line.get(i-n).unwrap();
            *counter.get_mut(c).unwrap() -= 1;
            if *counter.get(c).unwrap() == 0 {
                counter.remove(c);
            }

            let c = line.get(i).unwrap();
            if !counter.contains_key(c) {
                counter.insert(c, 0);
            }
            *counter.get_mut(c).unwrap() += 1;
            return (temp, i);
        })
        .filter(|(len, _)| *len == n)
        .take(1)
        .for_each(|(_, i)| println!("{}", i));
    
    println!("");
    
}
