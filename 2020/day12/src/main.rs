use std::fs;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let instructions: Vec<(char, i32)> = contents
        .lines()
        .map(|instruction| {
            let (direction, value) = instruction.split_at(1);
            (direction.chars().next().unwrap(), value.parse::<i32>().unwrap())
        })
        .collect::<Vec<(char, i32)>>();
    println!(
        "First: {}\nSecond: {}",
        operate(instructions.clone()),
        operate_betterly(instructions.clone())
    );
}

fn operate(instructions: Vec<(char, i32)>) -> i32 {
    let (x, y, _): (i32, i32, i32) = instructions
        .iter()
        .fold((0,0,90), |(x, y, d), (c, v)| {
            match c {
                'N' => (x, y + v, d),
                'S' => (x, y - v, d),
                'E' => (x + v, y, d),
                'W' => (x - v, y, d),
                'L' => (x, y, d - v),
                'R' => (x, y, d + v),
                'F' => {
                    let mut dir = d / 90;
                    dir = dir % 4;
                    if dir < 0 {
                        dir = dir + 4;
                    }
                    match dir {
                        0 => (x, y + v, d),
                        1 => (x + v, y, d),
                        2 => (x, y - v, d),
                        3 => (x - v, y, d),
                        _ => (x, y, d)
                    }
                },
                _ => (x, y, d)
            }
        });
    x.abs() + y.abs()
}

fn operate_betterly(instructions: Vec<(char, i32)>) -> i32 {
    let (x, y, _): (i32, i32, (i32, i32)) = instructions
        .iter()
        .fold((0,0,(10,1)), |(x, y, (wx, wy)), (c, v)| {
            match c {
                'N' => (x, y, (wx, wy + v)),
                'S' => (x, y, (wx, wy - v)),
                'E' => (x, y, (wx + v, wy)),
                'W' => (x, y, (wx - v, wy)),
                'F' => (x + v*wx, y + v*wy, (wx, wy)),
                'L' => {
                    let mut dir = v / 90;
                    dir = dir % 4;
                    if dir < 0 {
                        dir = dir + 4;
                    }
                    match dir {
                        0 => (x, y, (wx, wy)),
                        1 => (x, y, (-wy, wx)),
                        2 => (x, y, (-wx, -wy)),
                        3 => (x, y, (wy, -wx)),
                        _ => (x, y, (wx, wy))
                    }
                },
                'R' => {
                    let mut dir = v / 90;
                    dir = dir % 4;
                    if dir < 0 {
                        dir = dir + 4;
                    }
                    match dir {
                        0 => (x, y, (wx, wy)),
                        1 => (x, y, (wy, -wx)),
                        2 => (x, y, (-wx, -wy)),
                        3 => (x, y, (-wy, wx)),
                        _ => (x, y, (wx, wy))
                    }
                },
                _ => (x, y, (wx, wy))
            }
        });
    x.abs() + y.abs()
}