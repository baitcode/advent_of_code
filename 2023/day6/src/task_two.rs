use core::num;
use std::cmp::max;

use regex::Regex;

fn get_line<'a>(input: &'a str, line_starts: &Vec<usize>, idx: usize) -> &'a str {
    let start = line_starts[idx - 1] + 1;
    let end = max(line_starts[idx], start);
    return &input[start..end];
}

#[inline]
fn calculate_sq_root(a: i64, b: i64, c: i64) -> i64 {
    let x = (b * b - 4 * a * c) as f32;
    let z = (-b as f32 - x.sqrt()) / 2.0;
    return z.floor() as i64;
}

pub fn execute(input: &mut String) -> i64 {
    let line_starts: Vec<usize> = input.match_indices('\n').map(|i| i.0).collect();

    let time = i64::from_str_radix(
        &get_line(input, &line_starts, 1)
            .get(10..)
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>(),
        10,
    )
    .unwrap();

    let distance = i64::from_str_radix(
        &get_line(input, &line_starts, 2)
            .get(10..)
            .unwrap()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>(),
        10,
    )
    .unwrap();

    return vec![time]
        .iter()
        .zip(vec![distance])
        .map(|(time, distance)| {
            let minimal_push = calculate_sq_root(1, -time, distance) + 1;
            let longest_push = time - minimal_push;
            longest_push - minimal_push + 1
        })
        .reduce(|a, b| a * b)
        .unwrap();
}
