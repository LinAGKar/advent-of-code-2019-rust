use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::io::Read;

#[derive(PartialEq)]
enum Tile {
    Wall,
    Floor,
    Door(usize),
    Key(usize),
}

fn get_shortest_paths(map: &Vec<Vec<Tile>>, start: (i8, i8)) -> Vec<Vec<(u16, u32)>> {
    let mut queue = BinaryHeap::new();
    let mut visited: Vec<Vec<_>> = map.iter().map(|row| {
        row.iter().map(|_| Vec::new()).collect()
    }).collect();
    let mut shortest_paths = vec![Vec::new(); 26];

    queue.push((Reverse(0), 0, 0, start));

    while let Some((_, mut needed_keys, cost, pos)) = queue.pop() {
        let (y, x) = pos;

        let visited_this = &mut visited[y as usize][x as usize];
        if visited_this.iter().any(|&old_needed_keys| needed_keys & old_needed_keys == old_needed_keys) {
            continue;
        }
        visited_this.push(needed_keys);

        let tile = &map[y as usize][x as usize];
        match *tile {
            Tile::Door(door) => {
                needed_keys |= 1 << door;
            }

            Tile::Wall => { continue; }
            Tile::Floor => {}
            Tile::Key(key) => {
                shortest_paths[key as usize].push((cost, needed_keys));
                needed_keys |= 1 << key;
            }
        }

        for (dy, dx) in &[
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ] {
            let (y, x) = (y + dy, x + dx);
            queue.push((Reverse(cost + 1), needed_keys, cost + 1, (y, x)));
        }
    }

    shortest_paths
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut entrance = (0, 0);
    let mut keys = vec![(0, 0); 26];
    let mut doors = vec![(0, 0); 26];

    let mut map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, c)| {
            match c {
                '@' => {
                    entrance = (y as i8, x as i8);
                    Tile::Floor
                }

                '.' => Tile::Floor,
                '#' => Tile::Wall,

                _ => {
                    let ord = c as usize;
                    if c.is_ascii_uppercase() {
                        let index = ord - 'A' as usize;
                        doors[index] = (y as i8, x as i8);
                        Tile::Door(index)
                    } else {
                        let index = ord - 'a' as usize;
                        keys[index] = (y as i8, x as i8);
                        Tile::Key(index)
                    }
                }
            }
        }).collect()
    }).collect();

    map[entrance.0 as usize][entrance.1 as usize] = Tile::Wall;
    map[entrance.0 as usize - 1][entrance.1 as usize] = Tile::Wall;
    map[entrance.0 as usize + 1][entrance.1 as usize] = Tile::Wall;
    map[entrance.0 as usize][entrance.1 as usize - 1] = Tile::Wall;
    map[entrance.0 as usize][entrance.1 as usize + 1] = Tile::Wall;

    keys.push((entrance.0 - 1, entrance.1 - 1));
    keys.push((entrance.0 - 1, entrance.1 + 1));
    keys.push((entrance.0 + 1, entrance.1 - 1));
    keys.push((entrance.0 + 1, entrance.1 + 1));

    let costs_from_key: Vec<_> = keys.iter().map(|&pos| {
        get_shortest_paths(&map, pos)
    }).collect();

    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, [26, 27, 28, 29], 0));

    let mut visited = HashSet::new();

    while let Some((_, cost, poses, keys)) = queue.pop() {
        if visited.contains(&(poses, keys)) {
            continue;
        }
        visited.insert((poses, keys));

        if keys == 0x3FFFFFF {
            println!("{}", cost);
            break;
        }

        queue.extend(poses.iter().enumerate().flat_map(|(n, &pos)| {
            costs_from_key[pos].iter().enumerate().filter_map(move |(
                key, possibilities,
            )| {
                possibilities.iter().find_map(|&(new_cost, needed_keys)| {
                    if key != pos && keys & 1 << key == 0 && needed_keys & keys == needed_keys {
                        let new_cost = new_cost + cost;
                        let mut new_poses = poses;
                        new_poses[n] = key;
                        Some((Reverse(new_cost), new_cost, new_poses, keys | 1 << key))
                    } else {
                        None
                    }
                })
            })
        }));
    }
}
