fn find_calibration_value(line: &str) -> u32 {
    let ldigit = line
        .chars()
        .find_map(|c| c.to_digit(10))
        .expect("No digits found in the input");

    let rdigit = line
        .chars()
        .rev()
        .find_map(|c| c.to_digit(10))
        .expect("No digits found in the input");

    return ldigit * 10 + rdigit;
}

pub fn execute(input: String) -> u32 {
    let mut split = input.split('\n');
    let mut sum: u32 = 0;
    loop {
        let line = split.next();
        match line {
            None => break,
            Some(l) => sum += find_calibration_value(l),
        }
    }
    return sum;
}
