// http://adventofcode.com/2017/day/2
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn min(vec: &Vec<u32>) -> u32 {

    vec.iter().fold(u32::max_value(), |acc, &x| {
        if x < acc { x } else { acc }
    })
}

fn max(vec: &Vec<u32>) -> u32 {

    vec.iter().fold(0, |acc, &x| {
        if x > acc { x } else { acc }
    })
}

fn parse_line(line: &str) -> Vec<u32> {

    line.split("\t")
        .filter(|x| x.to_string().is_empty() == false)
        .map(|x| x.to_string().parse::<u32>().unwrap())
        .collect()
}

fn find_divisor(line: &Vec<u32>) -> Option<u32> {

    let mut j: u32 = 0;

    let i = line.iter()
        .find(|&x| {
            let val = line.iter()
                .find(|&y| x != y && x % y == 0);

            if val != None {
                j = x / *val.unwrap();
            }

            val != None
        });

    if i != None {
        Some(j)
    } else {
        None
    }
}

pub fn run() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut f = File::open(filename).expect("file not found");

    let mut input = String::new();

    f.read_to_string(&mut input)
        .expect("error reading file");

    let result = input.split("\n")
        .map(|line| {

            let parsed_line = parse_line(line);
            let max_value = max(&parsed_line);
            let min_value = min(&parsed_line);

            if max_value > min_value {
                max_value - min_value
            } else {
                0
            }
        })
        .fold(0, |acc, x| acc + x);

    println!("Sum part1: {}", result);

    let result2 = input.split("\n")
        .map(|line| parse_line(line))
        .map(|line| {
            
            match find_divisor(&line) {
                None => 0,
                Some(i) => i,
            }
        })
        .fold(0, |acc, x| acc + x);

    println!("Sum part2: {}", result2);
}
