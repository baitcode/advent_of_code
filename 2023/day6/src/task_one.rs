use core::num;
use std::cmp::max;

use regex::Regex;

fn get_line<'a>(input: &'a str, line_starts: &Vec<usize>, idx: usize) -> &'a str {
    let start = line_starts[idx - 1] + 1;
    let end = max(line_starts[idx], start);
    return &input[start..end];
}

fn calculate_sq_root(a: i32, b: i32, c: i32) -> i32 {
    let x = (b * b - 4 * a * c) as f32;
    let z = (-b as f32 - x.sqrt()) / 2.0;
    return z.floor() as i32;
}

pub fn execute(input: &mut String) -> i32 {
    let line_starts: Vec<usize> = input.match_indices('\n').map(|i| i.0).collect();

    let times_str = get_line(input, &line_starts, 1);
    let distances_str = get_line(input, &line_starts, 2);

    let numbers_pattern = Regex::new(r"\d+").unwrap();
    let times = numbers_pattern
        .find_iter(times_str)
        .map(|x| i32::from_str_radix(x.as_str(), 10).unwrap());

    let distances = numbers_pattern
        .find_iter(distances_str)
        .map(|x| i32::from_str_radix(x.as_str(), 10).unwrap());

    return times
        .zip(distances)
        .map(|(time, distance)| {
            let minimal_push = calculate_sq_root(1, -time, distance) + 1;
            let longest_push = time - minimal_push;
            longest_push - minimal_push + 1
        })
        .reduce(|a, b| a * b)
        .unwrap();
}
