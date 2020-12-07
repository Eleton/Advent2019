use std::fs;
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let (result, result2) = contents
        .split("\n\n")
        .map(|group| (get_any(group), get_all(group)))
        .fold((0, 0), |(acc_any, acc_all), (any, all)| (acc_any + any, acc_all + all));
    println!("First: {}\nSecond: {}", result, result2);
        
}

fn get_any(group: &str) -> usize {
    let filtered = group
        .chars()
        .filter(|&c| c != ' ' && c != '\n');
    let set: HashSet<char> = HashSet::from_iter(filtered);
    set.len()
}

fn get_all(group: &str) -> usize {
    let chars = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
    let initial_set: HashSet<char> = chars.iter().cloned().collect();
    group
        .split('\n')
        .map(|p| p.chars().collect())
        .fold(initial_set, |acc, set| &acc & &set)
        .len()
}