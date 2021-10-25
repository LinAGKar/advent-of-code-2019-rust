use std::collections::{HashSet, VecDeque};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_memory: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut visited = HashSet::new();
    let mut next = VecDeque::new();

    visited.insert((0, 0));
    next.push_back((0i8, 0i8, 0u8, intcode::IntCode::new(initial_memory)));

    'outer: while let Some((x, y, dist, computer)) = next.pop_front() {
        for (dx, dy, dir) in [
            (1, 0, 4),
            (-1, 0, 3),
            (0, 1, 2),
            (0, -1, 1),
        ] {
            let (x, y) = (x + dx, y + dy);
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            let mut computer = computer.clone();
            computer.put_input(dir);
            if let Some(output) = computer.run() {
                match output {
                    0 => {}
                    1 => next.push_back((x, y, dist + 1, computer)),
                    2 => {
                        println!("{}", dist + 1);
                        break 'outer;
                    }
                    _ => panic!(),
                }
            } else {
                panic!();
            }
        }
    }
}
