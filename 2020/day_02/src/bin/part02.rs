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
  let re =
    Regex::new(r"^(?P<first>\d+)-(?P<second>\d+) (?P<letter>.): (?P<password>\w+)$").unwrap();
  let mut valid_passwords = 0;
  for line in lines {
    let capture = re.captures(&line).unwrap();
    let first = capture["first"].parse::<usize>().unwrap();
    let second = capture["second"].parse::<usize>().unwrap();
    let letter = (&capture["letter"]).chars().next().unwrap();
    let mut valid_chars = 0;
    for (i, c) in (&capture["password"]).chars().enumerate() {
      if i + 1 == first && c == letter {
        valid_chars += 1;
      }
      if i + 1 == second && c == letter {
        valid_chars += 1;
      }
    }
    if valid_chars == 1 {
      valid_passwords += 1;
    }
  }
  println!("number of valid passwords = {}", valid_passwords);

  Ok(())
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
  P: AsRef<Path>,
{
  let file = File::open(filename)?;
  Ok(io::BufReader::new(file).lines())
}
