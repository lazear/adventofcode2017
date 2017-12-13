use std::io::{BufRead,BufReader};
use std::fs::File;

fn part1(mut instructions: Vec<i32>) -> u32 {
    let mut pc: usize = 0;
    let mut jmps: u32 = 0;

    while pc < instructions.len() {
        let x = instructions[pc];
        instructions[pc] += 1;
        pc = (pc as i32 + x) as usize;
        jmps += 1;
    }
    jmps
}

fn part2(mut instructions: Vec<i32>) -> u32 {
    let mut pc: usize = 0;
    let mut jmps: u32 = 0;

    while pc < instructions.len() {
        let x = instructions[pc];
        if x >= 3 {
            instructions[pc] -= 1;
        } else {
            instructions[pc] += 1;
        }
        
        pc = (pc as i32 + x) as usize;
        jmps += 1;
    }
    jmps
}


fn main() {
    let f = File::open("input").unwrap();
    let b = BufReader::new(&f);
    let instructions: Vec<i32> = b.lines().map(|s| s.unwrap().parse::<i32>().unwrap()).collect();

    assert_eq!(part1(vec![0, 3, 0, 1, -3]), 5);
    assert_eq!(part2(vec![0, 3, 0, 1, -3]), 10);
    println!("{}", part1(instructions.clone()));
    println!("{}", part2(instructions));
}
