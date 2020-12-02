use regex::Regex;
use std::fs;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let result1 = contents
        .lines()
        .map(|x| check(x))
        .sum::<u32>();
    println!("First: {}", result1);

    let result2 = contents
        .lines()
        .map(|x| check2(x))
        .sum::<u32>();
    println!("Second: {}", result2);
}


fn check(s: &str) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let groups = re.captures(s).unwrap();
    
    let min = &groups[1].parse::<u32>().unwrap();
    let max = &groups[2].parse::<u32>().unwrap();
    let key = &groups[3];
    let string = &groups[4];
    let occurences: u32 = string
        .split("")
        .filter(|k| k == &key)
        .map(|k| if k == key { 1 } else { 0 })
        .sum::<u32>();
    if &occurences >= min && &occurences <= max {
        1
    } else {
        0
    }
}

fn check2(s: &str) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    let groups = re.captures(s).unwrap();
    
    let min = &groups[1].parse::<usize>().unwrap();
    let max = &groups[2].parse::<usize>().unwrap();
    let key = &groups[3];
    let string = &groups[4];
    let occurences: Vec<&str> = string
        .split("")
        .collect();
    if (occurences[min  - 0] == key) as u32 +
       (occurences[max  - 0] == key) as u32 == 1 {
        1
    } else {
        0
    }
}
