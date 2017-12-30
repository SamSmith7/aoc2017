// http://adventofcode.com/2017/day/3
use std::env;


pub fn run() {

    let args: Vec<String> = env::args().collect();
    let target: u32 = args[1].parse().unwrap();

    let mut done: bool = false;
    let mut i: u32 = 1;
    let mut m: u32 = 1;
    let mut n: u32 = 0;

    while !done {

        let total = (4 * n) + m;

        if total > target {
            done = true;
        } else {
            i = i + 1;
            n = n + 2;
            m = total;
        }
    }

    let diff = target - m;
    let rem = diff % (n - 1);
    let result: u32;

    if rem > i {
        result = i + (rem - i) - 1;
    } else {
        result = i + (i - rem) - 1;
    }

    println!("Part1: {}", result);
}
