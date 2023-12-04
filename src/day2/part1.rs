use std::collections::HashMap;

pub fn solve(input: &[String]) {
    let mut sum = 0;
    let mut max = HashMap::new();
    max.insert("red", 12);
    max.insert("green", 13);
    max.insert("blue", 14);
    
    for (i, s) in input.iter().enumerate() {
        let game: Vec<_> = s.split(":").last().unwrap().split(";").collect();
        let mut possible = true;
        
        for turn in game {
            let cubes : Vec<_> =  turn.trim().split(",").collect();
            for cube in cubes {
                let mut cube_info = cube.trim().split(" ").into_iter();
                let grab = (cube_info.next().unwrap().parse::<i32>().unwrap(), cube_info.next().unwrap());
                possible &=  grab.0 <= *max.get(grab.1).unwrap();
            }
        }
        if possible {
            sum += i+1
        }
    }
    println!("{sum}");
}