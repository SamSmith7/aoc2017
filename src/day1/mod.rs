// http://adventofcode.com/2017/day/1
use std::fs::File;
use std::io::prelude::*;


pub fn run(filename: &str) {

    let mut f = File::open(filename).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input)
        .expect("error reading file");

    let initial: (u32, u32) = (6, 0);

    let chars: Vec<_> = input.chars()
        .filter(|&x| x.is_digit(10))
        .collect();

    let nums: Vec<_> = chars.iter()
        .map(|&x| { x.to_digit(10).unwrap() })
        .collect();

    let nums2 = nums.clone();

    let res = nums.iter().fold(initial, |state, &x| {

        if x == state.0 {
            (x, state.1 + x)
        } else {
            (x, state.1)
        }
    });

    println!("Part 1 Sum: {}", res.1);

    let length = chars.len();

    let res2 = nums2.iter().enumerate().fold(0, |acc, (i, &x)| {

        let next = i + length / 2;
        let compare_idx: usize;

        if next >= length {
            compare_idx = next % length;
        } else {
            compare_idx = next;
        }

        let compare = &chars[compare_idx].to_digit(10).unwrap();

        if x == *compare {
            acc + x
        } else {
            acc
        }
    });

    println!("Part 2 Sum: {}", res2)
}
