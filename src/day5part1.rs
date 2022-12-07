use crate::parser::read_file;

pub fn main() {
    let mut iter = read_file(5, 2)
        .map(|l| l.unwrap());
    let mut init_stack = Vec::new();
    let mut line = iter.next().unwrap();
    while !line.is_empty() {
        init_stack.push(line);
        line = iter.next().unwrap();
    }
    let num_stacks = init_stack.pop().unwrap().split(' ').filter(|s| !s.is_empty()).count();
    let mut stacks: Vec<Vec<char>> = (0..num_stacks).map(|_| Vec::new()).collect();
    init_stack.reverse();
    init_stack
        .iter()
        .for_each(|stack| {
            let chars = stack.as_bytes();
            (0..num_stacks)
                .map(|i| (i, chars[1 + i*4] as char))
                .filter(|(_, c)| *c != ' ')
                .for_each(|(i, c)| stacks.get_mut(i).unwrap().push(c));
        });

    iter
        .map(|s| s.split(' ').map(|s| String::from(s)).collect())
        .for_each(|lst: Vec<String>| {
            let num = lst.get(1).unwrap().parse::<usize>().unwrap();
            let from = lst.get(3).unwrap().parse::<usize>().unwrap() - 1;
            let to = lst.get(5).unwrap().parse::<usize>().unwrap() - 1;
            let temp = stacks.get_mut(from).unwrap();
            let mut temp = temp.split_off(temp.len().saturating_sub(num));
            temp.reverse();
            stacks.get_mut(to).unwrap().extend(temp);
        });

    stacks
        .iter()
        .for_each(|stack| print!("{}", stack.last().unwrap()));
    println!("");
    
}
