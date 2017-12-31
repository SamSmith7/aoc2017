// http://adventofcode.com/2017/day/3


pub fn run(input: &str) {

    let target: u32 = input.parse().unwrap();

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
        result = rem - 1;
    } else {
        result = (2 * i) - rem - 1;
    }

    println!("Part1: {}", result);
}
