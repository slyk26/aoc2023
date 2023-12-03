pub fn solve(input: &[String]) {
    println!(
        "{}",
        input.iter().fold(0, |acc, s| acc + (format!(
            "{}{}",
            s.chars().find(|c| c.is_numeric()).unwrap(),
            s.chars().rfind(|c| c.is_numeric()).unwrap()
        )
        .parse::<u32>()
        .unwrap()))
    );
}
