use std::{collections::HashMap, vec};

type Map<'a> = &'a [u8];
type PossibleMoves<'a> = &'a HashMap<u8, [i32; 2]>;

fn walk_path(from: i32, towards: i32, width: i32, map: &Map) -> (Vec<i32>, bool) {
    let allowed_moves = &HashMap::from([
        ('-' as u8, [-1, 1]),
        ('|' as u8, [-width, width]),
        ('7' as u8, [-1, width]),
        ('J' as u8, [-1, -width]),
        ('L' as u8, [1, -width]),
        ('F' as u8, [width, 1]),
    ]) as PossibleMoves;

    let mut cur = towards;
    let mut prev = from;
    let mut path = vec![];
    let mut found = false;

    loop {
        let tile = &map[cur as usize];
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
    let map = input.as_bytes() as Map;
    let width = (input.find('\n').unwrap() + 1) as i32;
    let start = input.find("S").unwrap() as i32;

    let res_path = [1, -1, width, -width]
        .iter()
        .map(|init_step| walk_path(start, start + init_step, width, &map))
        .filter(|(_, found)| *found)
        .map(|(p, _)| p)
        .reduce(|p1, p2| if p1.len() > p2.len() { p1 } else { p2 })
        .unwrap();

    return (res_path.len() as f32 / 2.0).ceil() as i32;
}
