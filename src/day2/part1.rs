pub fn solve(input: &[String]) {
    let red_max = 12;
    let green_max = 13;
    let blue_max = 14;
    let mut sum = 0;
    
    for (i, s) in input.iter().enumerate() {
        let game = s.split(":").last().unwrap();
        let cubes: Vec<_> = game.split(";").collect();
        let mut red_possible = true;
        let mut blue_possible = true;
        let mut green_possible = true;
        
        for cube in cubes {
            let throw : Vec<_> = cube.trim().split(",").collect();
            for c in throw {
                let d: Vec<_> = c.trim().split(" ").collect();
                let number = d.get(0).unwrap().parse::<i32>().unwrap();
                let color = d.get(1).unwrap();
                let _ = match color {
                    &"red" => red_possible &= number <= red_max,
                    &"green" => green_possible &= number <= green_max,
                    &"blue" => blue_possible &= number <= blue_max,
                    _ => ()
                };
            }
        }
        if red_possible && green_possible && blue_possible {
            sum += i+1
        }
    }
    
    println!("{sum}");
}