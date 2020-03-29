use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let sum:u32 = stdin.lock()
    .lines()
    .map(|line| line.unwrap().parse::<u32>().unwrap())
    .sum::<u32>();
    println!("{}", sum);
}
