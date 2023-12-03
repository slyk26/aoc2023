use std::fs::File;
use std::io::BufRead;
use std::io;
use std::path::Path;

mod day1;

fn main() {
    let f = read_file("./src/day1/input");
    
    day1::part1::solve(&f);
    day1::part2::solve(&f);
}

fn read_file(f: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let input = File::open(Path::new(f));

    match input {
        Ok(file) => {
            let reader = io::BufReader::new(file);
            
            for line in reader.lines() {
                if let Ok(line) = line {
                    lines.push(line);
                }
            }
        }
        Err(error) => {
            eprintln!("Problem opening the file: {:?}", error);
        }
    }
    lines
}
