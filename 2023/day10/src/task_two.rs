use std::{
    collections::{HashMap, HashSet},
    vec,
};

type Map<'a> = Vec<u8>;
type PossibleMoves<'a> = &'a HashMap<u8, [i32; 2]>;

fn walk_path(
    from: i32,
    towards: i32,
    map: &Map,
    allowed_moves: &PossibleMoves,
) -> (Vec<i32>, bool) {
    let mut cur = towards;
    let mut prev = from;
    let mut path = vec![];
    let mut found = false;

    loop {
        let tile = if cur < 0 || cur > map.len() as i32 {
            &('.' as u8)
        } else {
            &map[cur as usize]
        };
        if *tile == ('S' as u8) {
            found = true;
            path.push(cur);
            break;
        }

        if !allowed_moves.contains_key(tile) {
            break;
        }

        for val in allowed_moves[tile] {
            let next = val as i32 + cur;

            if next == prev {
                continue;
            }

            path.push(cur);
            prev = cur;
            cur = next;
            break;
        }
    }

    return (path, found);
}

pub fn execute(input: &mut String) -> i32 {
    let mut map = input.as_bytes().iter().map(|el| *el).collect::<Vec<u8>>();
    let width = (input.find('\n').unwrap() + 1) as i32;

    let allowed_moves = &HashMap::from([
        ('-' as u8, [-1, 1]),
        ('|' as u8, [-width, width]),
        ('7' as u8, [-1, width]),
        ('J' as u8, [-1, -width]),
        ('L' as u8, [1, -width]),
        ('F' as u8, [width, 1]),
    ]) as PossibleMoves;

    let start = input.find("S").unwrap() as i32;

    let res_path = [1, -1, width, -width]
        .iter()
        .map(|init_step| walk_path(start, start + init_step, &map, &allowed_moves))
        .filter(|(_, found)| *found)
        .map(|(p, _)| p)
        .reduce(|p1, p2| if p1.len() > p2.len() { p1 } else { p2 })
        .unwrap();

    // TODO: rewrite
    let first = res_path[0] - start;
    let last = res_path[res_path.len() - 2] - start;
    let mut fl_shifts = HashSet::new();
    fl_shifts.insert(first);
    fl_shifts.insert(last);

    for (k, value) in allowed_moves.iter() {
        let mut shifts = HashSet::new();
        shifts.insert(value[0]);
        shifts.insert(value[1]);

        if shifts == fl_shifts {
            map[start as usize] = *k;
            break;
        }
    }

    let corners = (vec![start].iter())
        .chain(res_path.iter())
        .filter(|&el| map[*el as usize] != '|' as u8)
        .filter(|&el| map[*el as usize] != '-' as u8)
        .map(|el| *el)
        .collect::<Vec<i32>>();

    let corners_rev = corners.iter().rev().map(|el| *el).collect::<Vec<i32>>();

    // Too tired to figure out the direction
    let area = vec![corners, corners_rev]
        .iter()
        .map(|corners| {
            corners
                .windows(2)
                .map(|el| {
                    // shoelace
                    let x1 = el[0] % width;
                    let y1 = el[0] / width;
                    let x2 = el[1] % width;
                    let y2 = el[1] / width;
                    return x1 * y2 - x2 * y1;
                })
                .sum::<i32>()
                / 2
        })
        .max()
        .unwrap();

    // Pick theorem
    return area - res_path.len() as i32 / 2 + 1;
}
