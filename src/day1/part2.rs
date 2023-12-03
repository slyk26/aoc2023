

pub fn solve(input: &[String]) {
    let numbers_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut sum = 0;
    let mut a: Vec<(&str, i32)> = Vec::new();
    
    for s in input {
        for number in numbers_strings {
            s.match_indices(number).for_each(|(i, v)| a.push((v, i as i32)));
        }
        
        a.retain(|(_, v)| v.ne(&-1));
        a.sort_by(|a, b| a.1.cmp(&b.1));
        
        let (fw, _) = a.first().unwrap();
        let (lw, _) = a.last().unwrap();
        
        let add = format!("{}{}", pparse(fw), pparse(lw)
        }).parse::<u32>().unwrap();
        
        sum += add;
        a = Vec::new();
    }

    println!("{sum}");
}

fn pparse(s: &&str) -> u32 {
    match fw.parse::<u32>() {
        Ok(n) => n,
        Err(_) => numbers_strings.iter().position(|n| n.eq(fw)).unwrap() as u32 +1
    }
}