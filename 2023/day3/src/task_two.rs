use std::{cmp::max, collections::VecDeque, ops::Range, str};

use regex::Regex;

fn calculate_gear_ratio(
    gears: Vec<Range<usize>>,
    digits: &VecDeque<Vec<(Range<usize>, u32)>>,
) -> u32 {
    let mut pairs = vec![];
    for gear in gears.iter() {
        let mut adjacent = vec![];
        for (digit_range, digit) in digits.iter().flatten() {
            if !(gear.end < digit_range.start || gear.start > digit_range.end) {
                adjacent.push(digit);
            }
        }

        if adjacent.len() == 2 {
            pairs.push((adjacent[0], adjacent[1]));
        }
    }
    let mut sum = 0;

    for pair in pairs {
        let (l, r) = pair;
        sum += l * r;
    }

    return sum;
}

pub fn execute(input: String) -> u32 {
    let digits = Regex::new(r"\d+").unwrap();
    let gears = Regex::new(r"[\*]").unwrap();

    let mut found_digits: VecDeque<Vec<(Range<usize>, u32)>> = VecDeque::new();
    let mut line_iterator = input.split('\n');

    let mut sum = 0;

    let next_line = line_iterator.next();
    if next_line == None {
        return 0;
    }
    let previous_line = next_line.unwrap();

    let mut found_gears: Vec<Range<usize>> =
        gears.find_iter(previous_line).map(|c| c.range()).collect();

    fn get_digit_pairs(re: Regex, line: &str) -> Vec<(Range<usize>, u32)> {
        return re
            .find_iter(line)
            .map(|c| {
                let digit_range = c.range();
                let number = str::from_utf8(&line.as_bytes()[digit_range.start..digit_range.end])
                    .ok()
                    .unwrap();

                return (digit_range, u32::from_str_radix(number, 10).ok().unwrap());
            })
            .collect();
    }

    found_digits.push_back(get_digit_pairs(digits.clone(), previous_line));

    loop {
        let next_line = line_iterator.next();
        if next_line != None {
            let data = next_line.unwrap();
            found_digits.push_back(get_digit_pairs(digits.clone(), data));
        }

        sum += calculate_gear_ratio(found_gears, &found_digits);

        if next_line != None {
            let data = next_line.unwrap();
            found_gears = gears.find_iter(data).map(|c| c.range()).collect();
        } else {
            break;
        }

        if found_digits.len() > 2 {
            found_digits.pop_front();
        }
    }

    return sum;
}
