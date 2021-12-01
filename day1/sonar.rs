use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let file = fs::File::open(filename).expect("Unable to open file");
    let reader = io::BufReader::new(file);

    let mut ints = Vec::new();
    for line in reader.lines() {
        let text = line.unwrap();
        let int: i32 = text.parse().unwrap();
        ints.push(int)
    }

    let part1_count = part1(&ints);
    println!("part 1: {}", part1_count);

    let part2_count = part2(&ints);
    println!("part 2: {}", part2_count);

    let part2_alt_count = part2_alt(&ints);
    println!("part 2: {}", part2_alt_count);
}

fn part1(vals: &Vec<i32>) -> i32 {
    let mut prev = -1;
    let mut count = -1;

    for val in vals {
        if val > &prev {
            count += 1;
        }
        prev = *val;
    }
    return count;
}

fn part2(vals: &Vec<i32>) -> i32 {
    let mut prev = -1;
    let mut count = -1;

    for ii in 1..(vals.len() - 1) {
        let mut sum = 0;
        for jj in (ii-1)..(ii+2) {
            sum += vals[jj];
        }

        if sum > prev {
            count += 1;
        }

        prev = sum;
    }
    return count;
}

fn part2_alt(vals: &Vec<i32>) -> i32 {
    let mut count = 0;

    for ii in 0..(vals.len() - 3) {
        if vals[ii + 3] > vals[ii] {
            count += 1;
        }
    }

    return count;
}
