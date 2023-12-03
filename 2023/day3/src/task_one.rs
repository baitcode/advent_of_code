use std::{cmp::max, collections::VecDeque, ops::Range, str};

use regex::Regex;

fn sum_adjacent(
    line: &str,
    digits: Vec<Range<usize>>,
    symbols: &VecDeque<Vec<Range<usize>>>,
) -> u32 {
    let mut adjacent = vec![];
    for digit_range in digits.iter() {
        for symbol_range in symbols.iter().flatten() {
            if !(digit_range.end < symbol_range.start || digit_range.start > symbol_range.end) {
                let number = str::from_utf8(&line.as_bytes()[digit_range.start..digit_range.end])
                    .ok()
                    .unwrap();

                adjacent.push(u32::from_str_radix(number, 10).ok().unwrap());
                break;
            }
        }
    }

    return adjacent.iter().sum();
}

pub fn execute(input: String) -> u32 {
    let digits = Regex::new(r"(\d+)").unwrap();
    let symbols = Regex::new(r"([^\d\.]+)").unwrap();

    let mut found_symbols: VecDeque<Vec<Range<usize>>> = VecDeque::new();
    let mut line_iterator = input.split('\n');

    let mut sum = 0;

    let next_line = line_iterator.next();
    if next_line == None {
        return 0;
    }
    let mut previous_line = next_line.unwrap();

    let mut found_digits: Vec<Range<usize>> =
        digits.find_iter(previous_line).map(|c| c.range()).collect();

    found_symbols.push_back(
        symbols
            .find_iter(previous_line)
            .map(|c| c.range())
            .collect(),
    );

    loop {
        let next_line = line_iterator.next();
        if next_line != None {
            let data = next_line.unwrap();
            found_symbols.push_back(symbols.find_iter(data).map(|c| c.range()).collect());
        }

        sum += sum_adjacent(previous_line, found_digits, &found_symbols);

        if next_line != None {
            let data = next_line.unwrap();
            found_digits = digits.find_iter(data).map(|c| c.range()).collect();
            previous_line = data;
        } else {
            break;
        }

        if found_symbols.len() > 2 {
            found_symbols.pop_front();
        }
    }

    return sum;
}
