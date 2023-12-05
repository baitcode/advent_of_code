use std::{
    cmp::{max, min, Ordering},
    ops::Range,
    str::FromStr,
};

use regex::Regex;

struct RangeMap {
    items: Vec<(Range<i64>, i64)>,
}

impl RangeMap {
    fn new(input: Vec<(i64, i64, i64)>) -> Self {
        let mut sorted_input = input.clone();
        sorted_input.sort_by_key(|i| i.1);

        let items = sorted_input
            .iter()
            .map(|(target_s, src_s, c)| (*src_s..src_s + c, target_s - src_s))
            .collect();

        return Self { items };
    }

    fn transform(&self, input: &i64) -> i64 {
        let found = self.items.binary_search_by(|i| {
            let range = &i.0;

            if range.contains(input) {
                return Ordering::Equal;
            }

            if range.start > *input {
                return Ordering::Greater;
            }

            return Ordering::Less;
        });

        match found {
            Ok(res) => input + self.items[res].1,
            Err(_) => return *input,
        }
    }
}

fn get_line<'a>(input: &'a str, line_starts: &Vec<usize>, idx: usize) -> &'a str {
    let start = line_starts[idx - 1] + 1;
    let end = max(line_starts[idx], start);
    return &input[start..end];
}

fn read_map(input: &str, line_starts: &Vec<usize>, line_number: usize) -> (RangeMap, usize) {
    let numbers = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();
    let mut cursor = line_number;
    let mut mappings: Vec<(i64, i64, i64)> = vec![];
    loop {
        let line = get_line(input, line_starts, cursor);
        if line.is_empty() {
            break;
        }
        for (_, [target_s, src_s, c]) in numbers.captures_iter(line).map(|c| c.extract()) {
            mappings.push((
                i64::from_str(target_s).unwrap(),
                i64::from_str(src_s).unwrap(),
                i64::from_str(c).unwrap(),
            ))
        }
        cursor += 1;
    }
    return (RangeMap::new(mappings), cursor + 1);
}

pub fn execute(input: &mut String) -> i64 {
    input.insert_str(0, "\n");
    input.push_str("\n\n");

    let line_starts: Vec<usize> = input.match_indices('\n').map(|i| i.0).collect();

    let (seed_to_soil, next_idx) = read_map(&input, &line_starts, 3);
    let (soil_to_fert, next_idx) = read_map(&input, &line_starts, next_idx);
    let (fert_to_water, next_idx) = read_map(&input, &line_starts, next_idx);
    let (water_to_light, next_idx) = read_map(&input, &line_starts, next_idx);
    let (light_to_temp, next_idx) = read_map(&input, &line_starts, next_idx);
    let (temp_to_hum, next_idx) = read_map(&input, &line_starts, next_idx);
    let (hum_to_loc, _) = read_map(&input, &line_starts, next_idx);

    let pipeline = vec![
        seed_to_soil,
        soil_to_fert,
        fert_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_loc,
    ];

    let seeds: Vec<i64> = get_line(&input, &line_starts, 1)
        .split(" ")
        .skip(1)
        .map(|s| i64::from_str_radix(s, 10).unwrap())
        .collect();

    let mut found = i64::MAX;
    for seed in seeds {
        let mut new_seed = seed.clone();

        for pipe in pipeline.iter() {
            new_seed = pipe.transform(&new_seed);
        }

        found = min(found, new_seed);
    }

    return found;
}
