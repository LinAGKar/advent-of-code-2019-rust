use std::collections::BTreeSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_memory: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut computer = intcode::IntCode::new(initial_memory.clone());
    let mut map = Vec::new(); 
    let mut row = Vec::new();

    let mut pos = (0, 0);
    let mut direction = (0, 0);

    while let Some(output) = computer.run() {
        let c = output as u8;

        match c as char {
            '\n' => {
                if !row.is_empty() {
                    map.push(row);
                    row = Vec::new();
                }
            }

            '>' => {
                pos = (map.len() as i8, row.len() as i8);
                direction = (0, 1);
                row.push('#' as u8);
            }

            '<' => {
                pos = (map.len() as i8, row.len() as i8);
                direction = (0, -1);
                row.push('#' as u8);
            }

            '^' => {
                pos = (map.len() as i8, row.len() as i8);
                direction = (-1, 0);
                row.push('#' as u8);
            }

            'v' => {
                pos = (map.len() as i8, row.len() as i8);
                direction = (1, 0);
                row.push('#' as u8);
            }

            _ => row.push(c),
        }
    }

    let get_at_pos = |y, x| {
        if y < 0 || x < 0 {
            false
        } else {
            let y = y as usize;
            let x = x as usize;
            if y >= map.len() || x >= map[y].len() {
                false
            } else {
                map[y][x] as char == '#'
            }
        }
    };

    let mut forward_steps = 0;
    let mut turn = '.';
    let mut path2 = Vec::new();

    loop {
        if get_at_pos(pos.0 + direction.0, pos.1 + direction.1) {
            forward_steps += 1;
            pos = (pos.0 + direction.0, pos.1 + direction.1);
        } else if get_at_pos(pos.0 + direction.1, pos.1 - direction.0) {
            if forward_steps > 0 {
                path2.push((turn, forward_steps));
                forward_steps = 0;
            }
            turn = 'R';
            direction = (direction.1, -direction.0)
        } else if get_at_pos(pos.0 - direction.1, pos.1 + direction.0) {
            if forward_steps > 0 {
                path2.push((turn, forward_steps));
                forward_steps = 0;
            }
            turn = 'L';
            direction = (-direction.1, direction.0)
        } else {
            if forward_steps > 0 {
                path2.push((turn, forward_steps));
            }
            break;
        }
    }

    let mut calls = BTreeSet::new();
    let mut functions = Vec::new();

    for func_index in 0..3 {
        let first_hole = if calls.is_empty() {
            (0, path2.len())
        } else {
            let mut prev: Option<(usize, usize, i64)> = None;

            let first_hole = calls.iter().chain(&[(path2.len(), path2.len(), func_index)]).find_map(|&x| {
                if let Some(prev_call) = prev {
                    if x.0 > prev_call.1 {
                        Some((prev_call.1, x.0))
                    } else {
                        prev = Some(x);
                        None
                    }
                } else {
                    prev = Some(x);
                    None
                }
            }).unwrap();

            first_hole
        };

        let mut matches = Vec::new();

        for i in 2..=first_hole.1 - first_hole.0 {
            let func = &path2[first_hole.0..first_hole.0 + i];

            let mut new_matches = Vec::new();
            let mut j = first_hole.0;
            while j + i <= path2.len() {
                let subset = &path2[j..j + i];
                if subset == func && !calls.iter().any(|a| a.0 < j + i && a.1 > j) {
                    new_matches.push((j, j + i, func_index));
                    j += i;
                } else {
                    j += 1;
                }
            }

            if new_matches.len() < matches.len() {
                break;
            } else {
                matches = new_matches;
            }
        }

        let first_match = matches[0];
        functions.push(&path2[first_match.0..first_match.1]);
        calls.extend(matches.into_iter());
    }

    let mut prev: Option<(usize, usize, i64)> = None;

    assert!(calls.iter().chain(&[(path2.len(), path2.len(), 0)]).all(|&x| {
        if let Some(prev_call) = prev {
            if x.0 > prev_call.1 {
                false
            } else {
                prev = Some(x);
                true
            }
        } else {
            prev = Some(x);
            true
        }
    }), "Full path not covered");

    let mut computer = intcode::IntCode::new(initial_memory);
    computer.set_at_address(0, 2);
    for (n, (_, _, func)) in calls.iter().enumerate() {
        if n > 0 {
            computer.put_input(',' as i64);
        }
        computer.put_input('A' as i64 + func);
    }

    computer.put_input('\n' as i64);

    for func in &functions {
        for (n, (dir, dist)) in func.iter().enumerate() {
            if n > 0 {
                computer.put_input(',' as i64);
            }
            for c in format!("{},{}", dir, dist).chars() {
                computer.put_input(c as i64);
            }
        }

        computer.put_input('\n' as i64);
    }

    computer.put_input('n' as i64);
    computer.put_input('\n' as i64);

    let mut last = 0;
    while let Some(output) = computer.run() {
        last = output;
    }

    println!("{}", last);
}
