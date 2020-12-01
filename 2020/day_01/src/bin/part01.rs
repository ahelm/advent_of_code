use std::env;
use std::fs;
use std::io::{self};
use std::collections::HashSet;
use std::process::exit;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let file_content = fs::read_to_string(&args[1])?;
    let mut numbers = HashSet::<i32>::new();

    // go over all the numbers and put them into a set
    for line in file_content.lines() {
        let num = line.parse::<i32>().unwrap();
        if numbers.contains(&num) {
            panic!("Double numbers discovered!");
        } else {
            numbers.insert(num);
        }
    }

    // iterate over the set and find valid numbers
    for num in numbers.iter() {
        let rest = 2020 - num;
        if numbers.contains(&rest) {
            println!("Found {0} and {1} -> {0} * {1} = {2}", num, rest, num*rest);
            exit(0)
        }
    }

    println!("Did not found a proper value!");
    exit(1)
}
