use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Command {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
enum Run {
    Success(i32),
    Fail(i32),
}

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let instructions: Vec<Command> = contents
        .lines()
        .map(|i| parse_instruction(i))
        .collect::<Vec<Command>>();
    let result1 = operate(instructions.clone());
    let result2 = fix_bug(instructions.clone());
    println!("First: {:?}\nSecond: {:?}", result1, result2);
}

fn parse_instruction(line: &str) -> Command {
    let (command, number) = line.split_at(3);
    let n: i32 = number.trim().parse::<i32>().unwrap();
    match command {
        "acc" => return Command::Acc(n),
        "jmp" => return Command::Jmp(n),
        _ => return Command::Nop(n),
    }
}

fn operate(instructions: Vec<Command>) -> Run {
    let mut acc = 0;
    let mut index: i32 = 0;
    let mut set: HashSet<i32> = HashSet::new();
    loop {
        if set.contains(&index) {
            return Run::Fail(acc);
        }
        set.insert(index);
        let instruction: &Command = &instructions[index as usize];
        if index == instructions.len() as i32 - 1 {
            match instruction {
                Command::Acc(n) => return Run::Success(acc + n),
                _ => return Run::Success(acc),
            }
        }
        match instruction {
            Command::Acc(n) => {
                acc = acc + n;
                index = index + 1;
            },
            Command::Jmp(n) => index = index + n,
            Command::Nop(_) => index = index + 1,
        }
    }
}

fn fix_bug(instructions: Vec<Command>) -> Run {
    let range = 0..instructions.len();
    for i in range {
        let mut new_ins = instructions.clone();
        new_ins[i] = convert_command(new_ins[i]);
        match operate(new_ins) {
            Run::Success(n) => return Run::Success(n),
            _ => ()
        }
    }
    return Run::Fail(0)
}

fn convert_command(command: Command) -> Command {
    match command {
        Command::Jmp(n) => return Command::Nop(n),
        Command::Nop(n) => return Command::Jmp(n),
        c => return c
    }
}