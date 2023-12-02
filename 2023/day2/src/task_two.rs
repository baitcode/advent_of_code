use std::{cmp::max, str::FromStr};

fn check_if_game_possible(line: &str) -> u32 {
    let mut game_description = line.split(":");
    let game_id = game_description.next().unwrap().get(5..).unwrap();
    let rounds = game_description.next().unwrap().split(";");

    let mut red: u32 = 0;
    let mut blue: u32 = 0;
    let mut green: u32 = 0;

    for round in rounds {
        let picks = round.split(",");
        for pick in picks {
            let mut ball_pairs = pick.split(" ");
            ball_pairs.next();

            let count_text = ball_pairs.next().unwrap();
            let color = ball_pairs.next().unwrap();
            let count_to_check = u32::from_str(count_text).unwrap();

            match color {
                "red" => red = max(red, count_to_check),
                "green" => green = max(green, count_to_check),
                "blue" => blue = max(blue, count_to_check),
                other => {
                    println!("asd");
                }
            };
        }
    }

    return red * blue * green;
}

pub fn execute(input: String) -> u32 {
    let mut split = input.split('\n');
    let mut sum: u32 = 0;
    loop {
        let line = split.next();
        if line == None {
            break;
        }
        sum += check_if_game_possible(line.unwrap()) as u32
    }
    return sum;
}
