use std::{
    cmp::{max, min},
    collections::{BTreeSet, HashSet},
    num::Wrapping,
    ops::Bound::{Excluded, Included},
};

use itertools::{self, Itertools};

pub fn execute(input: &mut String) -> u128 {
    // http://people.csail.mit.edu/mip/papers/dyn1d/arxiv.pdf

    println!("{}\n", input);

    let mut matches = input.match_indices("\n").map(|(idx, _)| idx);
    let width = matches.clone().next().unwrap() + 1;
    let height = matches.clone().count() + 1;

    let mut galaxies = input
        .match_indices("#")
        .map(|(idx, _)| idx)
        .collect::<Vec<usize>>();

    println!("w: {} h: {}\n", width, height);

    let rows = BTreeSet::from_iter(galaxies.iter().map(|m| m / width));
    let cols = BTreeSet::from_iter(galaxies.iter().map(|m| m % width));

    fn d(p1: usize, p2: usize, width: usize, r: &BTreeSet<usize>, c: &BTreeSet<usize>) -> usize {
        let y1 = Wrapping(p1 / width);
        let x1 = Wrapping(p1 % width);

        let y2 = Wrapping(p2 / width);
        let x2 = Wrapping(p2 % width);

        let zero = Wrapping(0);

        // println!("From ({}, {}) to ({}, {})", y1, x1, y2, x2);

        let leny = if y2 == y1 { zero } else { y2 - y1 };
        let startx = min(x1, x2);
        let lenx = max(x1, x2) - startx;

        // println!("\tDiff x={}", lenx);
        // println!("\tDiff y={}", leny);

        let w = 1000000;

        let y = if y1.0 != y2.0 {
            r.range((Excluded(y1.0), Included(y2.0))).count() * (w - 1)
        } else {
            0
        };
        let x = if x1.0 != x2.0 {
            c.range((Excluded(startx.0), Included((startx + lenx).0)))
                .count()
                * (w - 1)
        } else {
            0
        };

        // println!("\tExpansion x={} - {}", lenx.0, x);
        // println!("\tExpansion y={} - {}", leny.0, y);

        return lenx.0 * w + leny.0 * w + 1 - x - y - 1;
    }

    println!("{:?}", galaxies.iter().map(|v| *v).collect::<Vec<usize>>());

    let mut sum: u128 = 0;
    for i in 0..galaxies.len() - 1 {
        for j in (i + 1)..galaxies.len() {
            let p1 = galaxies[i];
            let p2 = galaxies[j];
            let z = d(min(p1, p2), max(p1, p2), width, &rows, &cols);
            println!("From {} -> {} d {}\n\n", i, j, z);
            sum += z as u128;
        }
    }
    return sum;
}
