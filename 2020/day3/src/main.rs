use std::fs;

fn main() {
    let filename = "signal.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let res1 = path(&contents, 3, 1);
    let res2 = path(&contents, 1, 1);
    let res3 = path(&contents, 5, 1);
    let res4 = path(&contents, 7, 1);
    let res5 = path(&contents, 1, 2);
    println!("First: {}", res2);
    println!("Second: {}", res1*res2*res3*res4*res5);
}
    
fn path(contents: &str, right: u64, down: usize) -> u64 {
    let length = contents.lines().nth(0).unwrap().len();
    let (result, _) = contents
        .lines()
        .step_by(down)
        .skip(1)
        .fold((0, right), |(trees, pos), x| match x.chars().nth((pos as usize) % length).unwrap() {
            '#' => {
                (trees + 1, pos + right)
            }
            _ => (trees, pos + right)
        });
    result
}
