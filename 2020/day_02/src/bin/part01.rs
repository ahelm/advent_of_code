use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
  let file_name = match env::args().nth(1) {
    None => panic!("Requires input file!"),
    Some(arg) => arg,
  };
  let lines = read_lines(file_name)?.map(|l| l.unwrap());
  let re = Regex::new(r"^(?P<min>\d+)-(?P<max>\d+) (?P<letter>.): (?P<password>\w+)$").unwrap();
  let mut valid_passwords = 0;
  for line in lines {
    let capture = re.captures(&line).unwrap();
    let min = capture["min"].parse::<usize>().unwrap();
    let max = capture["max"].parse::<usize>().unwrap();
    let password = &capture["password"];
    let count = password.matches(&capture["letter"]).count();
    if min <= count && count <= max {
      valid_passwords += 1;
    }
  }
  println!("{}", valid_passwords);

  Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
