use std::fs;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut ids: Vec<u32> = contents
        .lines()
        .map(|pass| seat_id(pass))
        .collect();
    ids.sort();
    let max = ids[ids.len() - 1];
    println!("First: {}", max);
    let mut counter = ids[0];
    for id in &ids {
        if counter == *id {
            counter = counter + 1;
        } else {
            break;
        }
    }
    println!("Second: {:?}", counter);
}

fn seat_id(pass: &str) -> u32 {
    let (raw_row, raw_col) = pass.split_at(7);
    let row = parse_to_binary(raw_row, "B", "F");
    let col = parse_to_binary(raw_col, "R", "L");
    row*8 + col
}

fn parse_to_binary(raw: &str, one: &str, zero: &str) -> u32 {
    let bin = &raw
        .split("")
        .map(|c| if c == one {"1"} else if c == zero {"0"} else {""})
        .collect::<Vec<&str>>();
    u32::from_str_radix(&bin.join(""), 2).unwrap()
}