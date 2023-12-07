use itertools::Itertools;
use std::cmp::{max, Ordering};

fn get_line<'a>(input: &'a str, line_starts: &Vec<usize>, idx: usize) -> &'a str {
    let start = line_starts[idx - 1] + 1;
    let end = max(line_starts[idx], start);
    return &input[start..end];
}

trait Zzz {
    fn if_empty_append(&self, val: &str) -> String;
}

impl Zzz for String {
    fn if_empty_append(&self, val: &str) -> String {
        if self.is_empty() {
            return val.to_string();
        }
        return self.to_string();
    }
}

pub fn execute(input: &mut String) -> usize {
    let order = "AKQT98765432J".chars().rev().collect::<String>();

    let line_starts: Vec<usize> = input.match_indices('\n').map(|i| i.0).collect();

    return (1..line_starts.len())
        .map(|idx| get_line(input, &line_starts, idx))
        .map(|line| {
            (
                &line[..5],
                usize::from_str_radix(&line[5..].trim(), 10).unwrap(),
            )
        }) // END of Parsing. Now we have input in form (str, usize)
        .map(|(cards, bid)| {
            (
                cards,
                bid,
                cards
                    .chars()
                    .filter(|c| *c != 'J')
                    .sorted()
                    .group_by(|c| *c)
                    .into_iter()
                    .map(|(_, g)| g.count())
                    .sorted_by(|a, b| Ord::cmp(&b, &a))
                    .enumerate()
                    .map(|(idx, v)| {
                        if idx > 0 {
                            v
                        } else {
                            v + cards.chars().filter(|c| *c == 'J').count()
                        }
                    })
                    .map(|c| (c).to_string())
                    .collect::<String>()
                    .if_empty_append("5"),
            )
        })
        .sorted_by(|(cards1, _, group1), (cards2, _, group2)| {
            return match str::cmp(group1, group2) {
                Ordering::Equal => Iterator::zip(
                    cards1.chars().map(|c| order.find(c).unwrap()),
                    cards2.chars().map(|c| order.find(c).unwrap()),
                )
                .map(|(c1, c2)| usize::cmp(&c1, &c2))
                .filter(|i| *i != Ordering::Equal)
                .next()
                .unwrap_or(Ordering::Equal),

                other => other,
            };
        })
        .enumerate()
        // .map(|c| {
        // if c.1 .0.contains('J') {
        // println!("{} {} {} {}", c.0, c.1 .1, c.1 .2, c.1 .0);
        // }
        // return c;
        // })
        .map(|(rank, (_, bid, _))| (rank + 1) * bid)
        .sum();
}
