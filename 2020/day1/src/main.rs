use std::fs;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let two = search2(&contents);
    let three = search3(&contents);
    println!("Two numbers: {}", two);
    println!("Three numbers: {}", three);
}

fn search2(contents: &str) -> i32 {
    let data: Vec<i32> = contents
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    for &n in &data {
        match data.iter().find(|&&k| k == 2020 - n) {
            None => (),
            Some(x) => return x*n
        }
    }
    return 0
}

fn search3(contents: &str) -> i32 {
    let data: Vec<i32> = contents
        .split("\n")
        .map(|n| n.parse::<i32>().unwrap())
        .collect();
    for &i in &data {
        for &j in &data {
            match data.iter().find(|&&k| k == 2020 - i - j) {
                None => (),
                Some(x) => return i*j*x
            }
        }
    }
    return 0
}
