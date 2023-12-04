use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod day1;
mod day2;

fn main() {
    day1::part1::solve(&read_file("./src/day1/input"));
    day1::part2::solve(&read_file("./src/day1/input"));
    day2::part1::solve(&read_file("./src/day2/input"));
}

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
