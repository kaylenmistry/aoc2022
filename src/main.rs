use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    day1_solution();
    // day2_solution();
    // day3_solution();
    // day4_solution();
    // day5_solution();
    // day6_solution();
    // day7_solution();
    // day8_solution();
    // day9_solution();
    // day10_solution();
    // day11_solution();
    // day12_solution();
    // day13_solution();
    // day14_solution();
    // day15_solution();
    // day16_solution();
    // day17_solution();
    // day18_solution();
    // day19_solution();
    // day20_solution();
    // day21_solution();
    // day22_solution();
    // day23_solution();
    // day24_solution();
    // day25_solution();
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

fn sum_backpacks(backpacks: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut sums: Vec<i32> = backpacks.iter().map(|x| sum_calories(x)).collect();
    sums.sort();
    sums.reverse();
    return sums;
}

fn sum_calories(backpack: &Vec<i32>) -> i32 {
    return backpack.iter().sum();
}

fn day1_solution() {
    let backpacks: Vec<Vec<i32>> = build_elf_backpacks();
    let backpack_sums: Vec<i32> = sum_backpacks(&backpacks);
    println!("Day 1 Part 1: {}", backpack_sums.first().expect("No backpacks"));
    println!("Day 1 Part 2: {}", backpack_sums.iter().take(3).sum::<i32>());
}

// Day 3: AOC 2022

// fn build_elf_rucksacks() {
//     let f: File = File::open("data/day1.txt").expect("Unable to open file");
//     let f = BufReader::new(f);

//     let mut elf_rucksacks: Vec<Vec<i32>> = Vec::new();
//     let mut current_rucksack :Vec<i32> = Vec::new();

//     for line in f.lines() {
//         let line: String = line.expect("Unable to read line");

//         if line.is_empty() {
//             elf_backpacks.push(current_backpack);
//             current_backpack = Vec::new();
//         } else {
//             let x: i32 = line.parse().unwrap();
//             current_backpack.push(x);
//         }
//     }
// }