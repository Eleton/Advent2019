use std::fs;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let numbers = contents
        .lines()
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let result1 = validify(numbers.clone());
    let result2 = checkify(numbers.clone(), result1);
    println!("First: {}\nSecond: {}", result1, result2)
}

fn validify(numbers: Vec<i64>) -> i64 {
    let pre_size = 25;
    let length = numbers.len();
    for i in pre_size..length {
        let mut valid = false;
        for pi in (i - pre_size)..i {
            for pj in (i - pre_size)..i {
                if pi == pj {
                    continue;
                }
                if numbers[pi] + numbers[pj] == numbers[i] {
                    valid = true;
                }
            }
        }
        if !valid {
            return numbers[i];
        }
    }
    0
}

fn checkify(numbers: Vec<i64>, checker: i64) -> i64 {
    let (mut small, mut high) = (0, 0);
    'outer:for i in 0..numbers.len() {
        let mut v: Vec<i64> = Vec::new();
        let mut j = i;
        loop {
            let sum: i64 = v.iter().sum();
            if sum == checker {
                small = *v.iter().min().unwrap();
                high = *v.iter().max().unwrap();
                break 'outer;
            } else if sum > checker {
                break;
            }
            v.push(numbers[j]);
            j = j + 1;
        }
    }
    small + high
}