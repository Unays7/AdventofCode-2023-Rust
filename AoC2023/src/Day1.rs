use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn task1() -> i32 {
    let mut input = Vec::new();
    let file = File::open("src/input1.txt").expect("Error opening the file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        input.push(line.unwrap());
    }
    let mut sum = 0;
    for s in &input {
      let mut cord = String::new();
      for c in s.chars(){
        if c.is_numeric(){
          cord.push(c);
        }
      }
      let first_element = cord.chars().next().unwrap();
      let second_element = cord.chars().last().unwrap();
      let appended_string = format!("{}{}", first_element, second_element);
      let int  = appended_string.parse::<i32>().unwrap();
      sum += int;
    }
    return sum;
}