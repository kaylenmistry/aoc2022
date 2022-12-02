use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let backpacks: Vec<Vec<i32>> = build_elf_backpacks();
    let mut backpack_sums: Vec<i32> = backpacks.iter().map(|x| sum_calories(x)).collect();
    backpack_sums.sort();
    backpack_sums.reverse();
    let day1res1 = backpack_sums.first().expect("No backpacks");
    let day1res2: i32 = backpack_sums.iter().take(3).sum();
    println!("Day 1 Part 1: {}", day1res1);
    println!("Day 1 Part 2: {:?}", day1res2);
}

// Day 1: AOC 2022

fn build_elf_backpacks() -> Vec<Vec<i32>> {
    let f: File = File::open("data/day1.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut elf_backpacks: Vec<Vec<i32>> = Vec::new();
    let mut current_backpack :Vec<i32> = Vec::new();

    for line in f.lines() {
        let line: String = line.expect("Unable to read line");

        if line.is_empty() {
            elf_backpacks.push(current_backpack);
            current_backpack = Vec::new();
        } else {
            let x: i32 = line.parse().unwrap();
            current_backpack.push(x);
        }
    }

    if !current_backpack.is_empty() {
        elf_backpacks.push(current_backpack)
    }

    elf_backpacks
}

fn sum_calories(backpack: &Vec<i32>) -> i32 {
    return backpack.iter().sum();
}