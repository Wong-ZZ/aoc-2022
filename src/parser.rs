use std::fs::File;
use std::io::{prelude::*, BufReader, Lines};

pub fn read_file(day: i32, part: i32) -> Lines<BufReader<File>> {
    let file = File::open(format!("{}/input/{day}-{part}.in", env!("CARGO_MANIFEST_DIR"))).expect("error reading file");
    let reader = BufReader::new(file);

    return reader.lines();
}
