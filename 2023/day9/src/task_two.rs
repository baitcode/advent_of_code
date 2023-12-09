use std::cmp::max;

fn calculate(data: &Vec<i32>, level: usize, idx: usize) -> i32 {
    return if level == 0 {
        data[idx]
    } else {
        calculate(data, level - 1, idx + 1) - calculate(data, level - 1, idx)
    };
}

#[inline]
fn get_line<'a>(input: &'a str, line_starts: &Vec<usize>, idx: usize) -> &'a str {
    let start = line_starts[idx - 1] + 1;
    let end = max(line_starts[idx], start);
    return &input[start..end];
}

pub fn execute(input: &mut String) -> i32 {
    input.insert(0, '\n');
    input.push('\n');
    input.push('\n');

    let line_starts: Vec<usize> = input.match_indices('\n').map(|i| i.0).collect();
    let mut idx = 1;
    let mut sum = 0__i32;
    loop {
        let line = get_line(input, &line_starts, idx);

        if line.is_empty() {
            break;
        }

        let numbers = line
            .split(" ")
            .map(|i| i32::from_str_radix(&i, 10).unwrap())
            .collect::<Vec<i32>>();

        let prediction = (0..numbers.len())
            .map(|level| calculate(&numbers, level, 0))
            .rev()
            .reduce(|a, b| b - a)
            .unwrap();

        sum += prediction;
        idx += 1;
    }

    return sum;
}
