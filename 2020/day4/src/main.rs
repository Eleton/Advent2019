use std::fs;
use regex::Regex;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let passports: u32 = contents
        .split("\n\n")
        .map(|p| parse(p))
        .sum::<u32>();
    let passports_real: u32 = contents
        .split("\n\n")
        .map(|p| parse_for_real(p))
        .sum::<u32>();
    println!("First: {}", passports);
    println!("Second: {}", passports_real);
}

fn parse(passport: &str) -> u32 {
    let fields_valid = [r"byr", r"iyr", r"eyr", r"hgt", r"hcl", r"ecl", r"pid"]
        .iter()
        .map(|field| Regex::new(field).unwrap())
        .map(|field| field.find(passport))
        .all(|x| x != None);
    if fields_valid {return 1} else {return 0}
}

fn parse_for_real(passport: &str) -> u32 {
    let fields_valid = [
        r"byr:(19[2-9][0-9]|200[0-2])",
        r"iyr:(201[0-9]|2020)",
        r"eyr:(202[0-9]|2030)",
        r"hgt:((1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in)",
        r"hcl:#[0-9a-f]{6}",
        r"ecl:(amb|blu|brn|gry|grn|hzl|oth)",
        r"pid:[0-9]{9}(\D|$)"
        ].iter()
        .map(|field| Regex::new(field).unwrap())
        .map(|field| field.find(passport))
        .all(|x| x != None);
    if fields_valid {return 1} else {return 0}
}