use std::collections::HashSet;
use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let map: Vec<Vec<_>> = input.lines().map(|line| {
        line.chars().map(|c| if c == '#' { 1 } else { 0 }).collect()
    }).collect();
    assert_eq!(map.len(), 5);
    for row in &map {
        assert_eq!(row.len(), 5);
    }

    let lookup_table: Vec<_> = (0..0b100000).map(|state| {
        let bits = (0..5).filter(|i| state >> i & 0b1 == 1).count();
        if bits == 2 || bits == 1 && state & 0b100 == 0 { 1 } else { 0 }
    }).collect();

    let mut map = (0..5).flat_map(|line| (0..5).map(move |col| (line, col))).fold(0u32, |acc, (line, col)| {
        acc | map[line][col] << line * 5 + col
    });

    let mut seen = HashSet::new();
    seen.insert(map);

    loop {
        map = (0..5).flat_map(|line| (0..5).map(move |col| (line, col))).fold(0u32, |acc, (row, col)| {
            let key = map << 5 >> row * 5 + col & 0b1 |
                      ((map >> row * 5 & 0b11111) << 1 >> col & 0b111) << 1 |
                      (map >> (row + 1) * 5 + col & 0b1) << 4;

            acc | lookup_table[key as usize] << row * 5 + col
        });

        if seen.contains(&map) {
            println!("{}", map);
            break;
        }

        seen.insert(map);
    }
}
