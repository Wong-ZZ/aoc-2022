use crate::parser::read_file;

pub fn main() {
    let mut total = 0;
    read_file(25, 1)
        .for_each(|l| {
            let mut curr = 0;
            l.unwrap().chars().rev().enumerate().for_each(|(i, c)| {
                match c {
                    '0' => (),
                    '1' => curr += 5i64.pow(i as u32),
                    '2' => curr += 5i64.pow(i as u32) * 2,
                    '-' => curr -= 5i64.pow(i as u32),
                    '=' => curr -= 5i64.pow(i as u32) * 2,
                    _ => unreachable!(),
                }
            });
            total += curr;
        });
    println!("{total}");
    let mut temp = total + 2;
    let mut digits = vec![];
    loop {
        match temp % 5 {
            0 => digits.push('='),
            1 => digits.push('-'),
            2 => digits.push('0'),
            3 => digits.push('1'),
            4 => digits.push('2'),
            _ => unreachable!()
        }
        if temp >= 5 {
            temp = temp / 5 + 2
        } else {
            break;
        }
    }
    digits.iter().rev().for_each(|c| {
        print!("{c}");
    });
    println!();
}
