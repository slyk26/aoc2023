pub fn solve(input: &[String]) {
    let numbers_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut sum = 0;
    let mut a: Vec<(usize, &str)> = Vec::new();
    
    for s in input {
        for number in numbers_strings {
            s.match_indices(number).for_each(|i| a.push(i));
        }
        
        a.sort_by(|a, b| a.0.cmp(&b.0));
        sum += format!("{}{}", pparse(a.first().unwrap().1, &numbers_strings), pparse(a.last().unwrap().1, &numbers_strings)).parse::<usize>().unwrap();
        a = Vec::new();
    }

    println!("{sum}");
}

fn pparse(s: &str, numbers_strings: &[&str]) -> usize {
    match s.parse::<usize>() {
        Ok(n) => n,
        Err(_) => numbers_strings.iter().position(|n| n.eq(&s)).unwrap() +1
    }
}