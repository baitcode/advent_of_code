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

    return numbers.intersection(&answers).count();
}

pub fn execute(input: String) -> u32 {
    let line_count = input.match_indices('\n').count() + 1;
    let mut card_count = vec![1_usize; line_count];

    fn get_line<'a>(input: &'a str, idx: usize) -> &'a str {
        return &input[117 * idx..117 * (idx + 1) - 1];
    }

    fn add_copies(cards: &mut [usize], count: usize) {
        for card in cards {
            *card = *card + count;
        }
    }

    for i in 0..line_count {
        let score = get_card_score(get_line(&input, i));
        let count = card_count[i];
        if score > 0 {
            add_copies(&mut card_count[i + 1..i + score + 1], count);
        }
    }
    let res: usize = card_count.iter().sum();
    return res as u32;
}
