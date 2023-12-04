use std::collections::HashSet;

fn get_card_score(line: &str) -> usize {
    fn read_numbers<'a>(
        from: &'a str,
        starting: usize,
        count: usize,
    ) -> impl Iterator<Item = &'a str> {
        (0..count).map(move |i| &from[starting + i * 3..starting + (i + 1) * 3 - 1])
    }

    let numbers: HashSet<&str> = HashSet::from_iter(read_numbers(line, 10, 10));
    let answers: HashSet<&str> = HashSet::from_iter(read_numbers(line, 42, 25));
    let matches = numbers.intersection(&answers).count();

    return if matches > 0 {
        (2_usize).pow((matches - 1) as u32)
    } else {
        0
    };
}

pub fn execute(input: String) -> u32 {
    let mut line_start = 0;
    let mut sum = 0_usize;

    loop {
        if line_start > input.len() {
            break;
        }
        let line_end = line_start
            + (input[line_start..])
                .find("\n")
                .unwrap_or(input.len() - line_start);

        let line = &input[line_start..line_end];
        sum += get_card_score(line);
        line_start = line_end + 1;
    }

    return sum as u32;
}
