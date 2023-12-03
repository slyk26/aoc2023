use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

mod day1;

fn main() {
    let f = read_file("./src/day1/input");
    
    day1::part1::solve(&f);
    day1::part2::solve(&f);
}

fn read_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
