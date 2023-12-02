use std::{cmp::min, ops::Range};

fn forward_slice_selector(idx: usize, word_len: usize, string_len: usize) -> Range<usize> {
    return idx..min(idx + word_len, string_len);
}

fn backward_slice_selector(idx: usize, word_len: usize, string_len: usize) -> Range<usize> {
    return string_len - min(idx + word_len, string_len)..string_len - idx;
}

fn check<F>(
    at_index: usize,
    of_string: &[u8],
    has_words: [&[u8]; 20],
    slice_for_index: F,
) -> Option<usize>
where
    F: Fn(usize, usize, usize) -> Range<usize>,
{
    for i in 0..of_string.len() {
        for (idx, word) in has_words.iter().enumerate() {
            let range = slice_for_index(at_index, word.len(), of_string.len());
            let slice = &of_string[range];

            if &slice == word {
                return Some(idx % 10);
            }
        }
    }
    return None;
}

fn find_calibration_value(input: &[u8]) -> u32 {
    let words_to_find = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ]
    .map(|w| w.as_bytes());

    let left_digit_collection: Vec<_> = (0..input.len())
        .map(|idx| check(idx, input, words_to_find, forward_slice_selector))
        .filter(|v| *v != None)
        .map(|v| v.unwrap())
        .take(1)
        .collect();

    let right_digit_collection: Vec<_> = (0..input.len())
        .map(|idx| check(idx, input, words_to_find, backward_slice_selector))
        .filter(|v| *v != None)
        .map(|v| v.unwrap())
        .take(1)
        .collect();

    let left_digit = left_digit_collection.first().unwrap();
    let right_digit = right_digit_collection.first().unwrap();

    return (left_digit * 10 + right_digit) as u32;
}

pub fn execute(input: String) -> u32 {
    let mut split = input.split('\n');
    let mut sum: u32 = 0;
    loop {
        let line = split.next();
        match line {
            None => break,
            Some(l) => sum += find_calibration_value(l.as_bytes()),
        }
    }
    return sum;
}
