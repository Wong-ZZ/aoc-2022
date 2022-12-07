use crate::parser::read_file;

pub fn main() {
    let iter = read_file(7, 1)
        .map(|l| l.unwrap().split(' ').map(String::from).collect::<Vec<String>>());
    let mut dirs = std::collections::HashMap::new();
    let mut curr_dir = vec!["/".to_string()];
    dirs.insert(curr_dir.join(""), 0u64);
    for line in iter {
        let first_word = line.get(0).unwrap();
        let second_word = line.get(1).unwrap();
        if first_word == "$" && second_word == "cd" {
            let dir = String::from(line.get(2).unwrap());
            match dir.as_str() {
                "/" => (),
                ".." => {
                    curr_dir.pop();
                },
                _ => {
                    curr_dir.push(format!("/{dir}"));
                    let path = curr_dir.join("");
                    if !dirs.contains_key(&path) {
                        dirs.insert(path, 0);
                    }
                }
            }
        } else if first_word != "$" {
            match first_word.as_str() {
                "dir" => (),
                _ => {
                    let mut temp_path = "".to_string();
                    for dir in curr_dir.iter() {
                        temp_path.push_str(dir);
                        *dirs.get_mut(&temp_path).unwrap() += first_word.parse::<u64>().unwrap();
                    }
                }
            }
        }
    }
    let mut result: u64 = 0;
    for (_, size) in dirs {
        if size <= 100000 {
            result += size;
        }
    }
    println!("{result}");
    
}
