use std::io::Read;

fn opposite_direction(dir: &str) -> &str {
    match dir {
        "north" => "south",
        "south" => "north",
        "west" => "east",
        "east" => "west",
        _ => panic!(),
    }
}

fn try_take(mut computer: intcode::IntCode, item: &str, door: &str, current_room: &str) -> bool {
    computer.put_line(&format!("take {}", item));
    computer.put_line(door);

    let mut seen_lines = Vec::new();

    let mut line = String::new();
    while let Ok(_) = computer.get_line(&mut line) {
        if line.starts_with("==") {
            let room = &line[3..line.len() - 3];
            return room != current_room;
        }

        if line != "" {
            if seen_lines.contains(&line) {
                // Stuck in a loop
                return false;
            }
            seen_lines.push(line.clone());
        }
    }

    // Dead
    false
}

#[derive(Clone, Copy)]
enum ParseState {
    None,
    Doors,
    Items,
}

fn explore(
    mut computer: intcode::IntCode,
    path_taken: &mut Vec<String>,
    item_paths: &mut Vec<(String, Vec<String>)>,
    through_checkpoint: &mut Vec<String>,
) {
    let mut parse_state = ParseState::None;
    let mut doors = Vec::<String>::new();
    let mut items = Vec::<String>::new();
    let mut room = String::new();

    let mut line = String::new();
    while let Ok(_) = computer.get_line(&mut line) {
        match (line.as_str(), parse_state) {
            ("Doors here lead:", ParseState::None) => {
                parse_state = ParseState::Doors;
            }

            ("Items here:", ParseState::None) => {
                parse_state = ParseState::Items;
            }

            ("Command?", ParseState::None) => {
                for item in &items {
                    if try_take(computer.clone(), item, &doors[0], &room) {
                        item_paths.push((item.to_string(), path_taken.clone()));
                    }
                }

                for door in &doors {
                    if door == path_taken.last().map(|x| opposite_direction(x)).unwrap_or_default() {
                        continue;
                    }

                    path_taken.push(door.to_string());
                    if room == "Security Checkpoint" {
                        *through_checkpoint = path_taken.clone();
                    } else {
                        let mut computer = computer.clone();
                        computer.put_line(door);
                        explore(computer, path_taken, item_paths, through_checkpoint);
                    }
                    path_taken.pop();
                }

                return;
            }

            ("", _) => {
                parse_state = ParseState::None;
            }

            (line, ParseState::Doors) => {
                doors.push(line[2..].to_string());
            }

            (line, ParseState::Items) => {
                items.push(line[2..].to_string());
            }

            (line, ParseState::None) => {
                if line.starts_with("==") {
                    room = line[3..line.len() - 3].to_string();
                }
            }
        }
    }

    panic!();
}

#[derive(PartialEq)]
enum CheckpointResult {
    Heavier,
    Lighter,
    Done,
}

fn try_checkpoint(computer: &mut intcode::IntCode, door: &str) -> CheckpointResult {
    computer.put_line(door);

    let mut code = 0;

    let mut line = String::new();
    while let Ok(_) = computer.get_line(&mut line) {
        for word in line.split_whitespace() {
            if let Ok(new_code) = word.parse() {
                code = new_code;
            }

            if word == "heavier" {
                while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
                return CheckpointResult::Heavier;
            }

            if word == "lighter" {
                while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
                return CheckpointResult::Lighter;
            }
        }
    }

    println!("{}", code);
    CheckpointResult::Done
}

fn pass_checkpoint(mut computer: intcode::IntCode, door: &str, mut items: Vec<&str>) {
    let mut line = String::new();

    for &item in &items {
        computer.put_line(&format!("drop {}", item));
        while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
    }

    let mut new_items = Vec::new();

    loop {
        let mut eliminated_item = false;

        new_items.clear();

        for &item in &items {
            computer.put_line(&format!("take {}", item));
            while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}

            match try_checkpoint(&mut computer, door) {
                CheckpointResult::Heavier => { new_items.push(item); },
                CheckpointResult::Lighter => { eliminated_item = true; },
                CheckpointResult::Done => { return },
            }

            computer.put_line(&format!("drop {}", item));
            while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
        }

        std::mem::swap(&mut items, &mut new_items);

        for &item in &items {
            computer.put_line(&format!("take {}", item));
            while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
        }

        new_items.clear();

        for &item in &items {
            computer.put_line(&format!("drop {}", item));
            while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}

            match try_checkpoint(&mut computer, door) {
                CheckpointResult::Heavier => { eliminated_item = true; },
                CheckpointResult::Lighter => { new_items.push(item); },
                CheckpointResult::Done => { return },
            }

            computer.put_line(&format!("take {}", item));
            while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
        }

        std::mem::swap(&mut items, &mut new_items);

        for &item in &items {
            computer.put_line(&format!("drop {}", item));
            while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
        }

        if !eliminated_item {
            // Fall back to brute force
            println!("fall back");
            break;
        }
    }

    let mut prev_i = 0;

    for i in 0..1 << items.len() {
        for (j, item) in items.iter().enumerate() {
            match (i & 1 << j != 0, prev_i & 1 << j != 0) {
                (true, false) => {
                    computer.put_line(&format!("take {}", item));
                    while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
                }

                (false, true) => {
                    computer.put_line(&format!("drop {}", item));
                    while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}
                }

                _ => {}
            }
        }

        if try_checkpoint(&mut computer, door) == CheckpointResult::Done {
            return;
        }

        prev_i = i;
    }
}

fn main() {
    let mut args = std::env::args();
    let interactive = args.nth(1).unwrap_or_default() == "--interactive";

    let mut input = String::new();

    if interactive {
        let filename = &args.next().unwrap();
        let mut file = std::fs::File::open(filename).unwrap();
        file.read_to_string(&mut input).unwrap();
    } else {
        std::io::stdin().read_to_string(&mut input).unwrap();
    };

    let initial_memory: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut computer = intcode::IntCode::new(initial_memory.clone());

    if interactive {
        let mut line = String::new();
        while let Ok(_) = computer.get_line(&mut line) {
            println!("{}", line);
            if line == "Command?" {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                for c in input.chars() {
                    computer.put_input(c as i64);
                }
            }
        }
    } else {
        let mut item_paths = Vec::new();
        let mut through_checkpoint = Vec::new();

        explore(computer.clone(), &mut Vec::new(), &mut item_paths, &mut through_checkpoint);

        let items: Vec<_> = item_paths.iter().map(|(item, _)| item.as_str()).collect();

        for (item, path) in &item_paths {
            for i in path {
                computer.put_line(i);
            }

            computer.put_line(&format!("take {}", item));

            for i in path.into_iter().rev() {
                computer.put_line(opposite_direction(&i));
            }
        }

        for i in through_checkpoint.iter().take(through_checkpoint.len() - 1) {
            computer.put_line(i);
        }

        let mut line = String::new();
        while { computer.get_line(&mut line).unwrap(); line != "== Security Checkpoint ==" } {}
        while { computer.get_line(&mut line).unwrap(); line != "Command?" } {}

        pass_checkpoint(computer, through_checkpoint.last().unwrap(), items);
    }
}
