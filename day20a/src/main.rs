use std::cmp::Reverse;
use std::collections::{BinaryHeap,HashMap};
use std::io::Read;

#[derive(PartialEq)]
enum Tile {
    Floor,
    Wall,
    Portal(usize, usize),
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut letters = HashMap::new();

    let mut map: Vec<Vec<_>> = input.lines().enumerate().map(|(y, line)| {
        line.chars().enumerate().map(|(x, tile)| {
            match tile {
                '#' => Tile::Wall,
                '.' => Tile::Floor,
                ' ' => Tile::Wall,

                c => {
                    letters.insert((y, x), c);
                    Tile::Wall
                }
            }
        }).collect()
    }).collect();

    let mut portals = HashMap::<_, (usize, _)>::new();

    for (&(y, x), &c) in &letters {
        if let Some(((portal_y, portal_x), name)) = if let Some(&c_below) = letters.get(&(y + 1, x)) {
            Some(((if y == 0 || map[y - 1][x] == Tile::Wall {
                y + 2
            } else {
                y - 1
            }, x), (c, c_below)))
        } else if let Some(&c_right) = letters.get(&(y, x + 1)) {
            Some(((y, if x == 0 || map[y][x - 1] == Tile::Wall {
                x + 2
            } else {
                x - 1
            }), (c, c_right)))
        } else {
            None
        } {
            if let Some(&(other_y, other_x)) = portals.get(&name) {
                map[other_y][other_x] = Tile::Portal(portal_y, portal_x);
                map[portal_y][portal_x] = Tile::Portal(other_y, other_x);
            } else {
                portals.insert(name, (portal_y, portal_x));
            }
        }
    }

    let (ent_y, ent_x) = portals[&('A', 'A')];
    let exit = portals[&('Z', 'Z')];
    let mut visited: Vec<Vec<_>> = map.iter().map(|line| line.iter().map(|_| false).collect()).collect();
    visited[ent_y][ent_x] = true;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, ent_y, ent_x));

    while let Some((_, dist, y, x)) = queue.pop() {
        if (y, x) == exit {
            println!("{}", dist);
            break;
        }

        let new_dist = dist + 1;

        for (y, x) in [
            (y - 1, x),
            (y + 1, x),
            (y, x - 1),
            (y, x + 1),
        ] {
            if map[y][x] != Tile::Wall && !visited[y][x] {
                visited[y][x] = true;
                queue.push((Reverse(new_dist), new_dist, y, x));
            }
        }

        if let Tile::Portal(y, x) = map[y][x] {
            if !visited[y][x] {
                visited[y][x] = true;
                queue.push((Reverse(new_dist), new_dist, y, x));
            }
        }
    }
}
