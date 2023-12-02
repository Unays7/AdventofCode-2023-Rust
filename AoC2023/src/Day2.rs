// only 12 red cubes, 13 green cubes, and 14 blue cubes
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

static MAX_RED: i32 = 12;
static MAX_BLUE: i32 = 14;
static MAX_GREEN: i32 = 13;

pub fn get_input() -> HashMap<i32, HashMap<String, i32>>{
    let mut input = Vec::new();
    let file = File::open("rescources/input2.txt").expect("Error cant get file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        input.push(line.unwrap());
    }

    let mut map: HashMap<i32, HashMap<String, i32>> = HashMap::new();

    for (i, line) in input.iter().enumerate() {
        let mut split: Vec<&str> = line.split_whitespace().collect();
        split.drain(0..2);

        for (j, ele) in split.iter().enumerate() {
            if ele.parse::<f64>().is_ok() {
                let curr_count: i32 = ele.parse().unwrap();
                let mut slice: String = split[j + 1].to_string();
                slice = slice.trim_end_matches(|c: char| !c.is_alphabetic()).to_string();

                map.entry(i as i32 + 1)
                    .or_insert_with(HashMap::new)
                    .entry(slice.to_string())
                    .and_modify(|v| *v += curr_count)
                    .or_insert(curr_count);
            }
        }
    }
  return map;
}


pub fn func() -> i32 {
  let mut map: HashMap<i32, HashMap<String, i32>> = get_input();
  let mut sum : i32  = 0;
  for (game, secondMap) in &map{
    let mut red_count: i32 = 0;
    let mut blue_count: i32  = 0;
    let mut green_count: i32  = 0;
    for (colour, count) in secondMap{
      if colour == "red"{ red_count += count;}
      if colour == "blue"{blue_count += count;}
      if colour == "green"{green_count += count;}
    }
    if red_count <= MAX_RED && blue_count <= MAX_BLUE && green_count <= MAX_GREEN {sum += game;}

    
  }
  return sum
}