use std::str::FromStr;

fn check_if_game_possible(game_description: &str) -> bool {
    let rounds = game_description.trim().split(";");

    for round in rounds {
        let picks = round.trim().split(",");
        for pick in picks {
            let mut ball_pairs = pick.trim().split(" ");

            let count_text = ball_pairs.next().unwrap();
            let color = ball_pairs.next().unwrap();
            let count_to_check = i32::from_str(count_text).unwrap();

            let passed = match color {
                "red" => count_to_check <= 12,
                "green" => count_to_check <= 13,
                "blue" => count_to_check <= 14,
                _ => false,
            };

            if !passed {
                return false;
            }
        }
    }

    return true;
}

pub fn execute(input: String) -> u32 {
    let mut split = input.split('\n');
    let mut sum: u32 = 0;
    loop {
        let line = split.next();
        if line == None {
            break;
        }

        let mut game_description = line.unwrap().split(":");
        let game_id = game_description.next().unwrap().get(5..).unwrap();

        if check_if_game_possible(game_description.next().unwrap()) {
            sum += u32::from_str(game_id).unwrap();
        }
    }
    return sum;
}
