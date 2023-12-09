use std::{char, cmp::max, collections::HashMap};

use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;

fn get_line<'a>(input: &'a str, line_starts: &Vec<usize>, idx: usize) -> &'a str {
    let start = line_starts[idx - 1] + 1;
    let end = max(line_starts[idx], start);
    return &input[start..end];
}

pub fn execute(input: &mut String) -> usize {
    input.insert(0, '\n');
    input.push('\n');
    input.push('\n');

    let line_starts: Vec<usize> = input.match_indices('\n').map(|i| i.0).collect();

    let mut graph = HashMap::new();
    let mut idx = 3;

    loop {
        let mapping = get_line(input, &line_starts, idx);
        if mapping.is_empty() {
            break;
        }

        graph.insert(&mapping[0..3], vec![&mapping[7..10], &mapping[12..15]]);

        idx += 1;
    }

    return get_line(input, &line_starts, 1)
        .chars()
        .cycle()
        .fold_while(("AAA", 0), |(key, idx), el| {
            let x = graph[key][(el == 'R') as usize];
            if x == "ZZZ" {
                Done((x, idx + 1))
            } else {
                Continue((x, idx + 1))
            }
        })
        .into_inner()
        .1;
}
