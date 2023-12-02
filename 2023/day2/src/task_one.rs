use std::{collections::HashMap, default, str::FromStr};

fn check_if_game_possible(line: &str) -> Option<usize> {
    let mut game_description = line.split(":");
    let game_id = game_description.next().unwrap().get(5..).unwrap();
    let rounds = game_description.next().unwrap().split(";");

    for round in rounds {
        let picks = round.split(",");
        for pick in picks {
            let mut ball_pairs = pick.split(" ");
            ball_pairs.next();

            let count_text = ball_pairs.next().unwrap();
            let color = ball_pairs.next().unwrap();
            let count_to_check = i32::from_str(count_text).unwrap();

            let passed = match color {
                "red" => count_to_check <= 12,
                "green" => count_to_check <= 13,
                "blue" => count_to_check <= 14,
                other => false,
            };

            if !passed {
                return None;
            }
        }
    }

    return Some(usize::from_str(game_id).unwrap());
}

pub fn execute(input: String) -> u32 {
    let mut split = input.split('\n');
    let mut sum: u32 = 0;
    loop {
        let line = split.next();
        if line == None {
            break;
        }

        match check_if_game_possible(line.unwrap()) {
            None => continue,
            Some(l) => sum += l as u32,
        }
    }
    return sum;
}
