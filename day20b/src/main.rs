use std::cmp::Reverse;
use std::collections::{BinaryHeap,HashMap};
use std::io::Read;

#[derive(PartialEq)]
enum Tile {
    Floor,
    Wall,
    Portal(usize, usize, bool),
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

    let map_h = map.len();
    let map_w = map[0].len();
    let is_inner = |y, x| {
        y > 2 && x > 2 && y < map_h - 3 && x < map_w - 3
    };

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
                map[other_y][other_x] = Tile::Portal(portal_y, portal_x, is_inner(other_y, other_x));
                map[portal_y][portal_x] = Tile::Portal(other_y, other_x, is_inner(portal_y, portal_x));
            } else {
                portals.insert(name, (portal_y, portal_x));
            }
        }
    }

    let grow_visited = |visited: &mut Vec<Vec<Vec<_>>>, size| {
        while visited.len() < size as usize {
            visited.push((0..map_h).map(|_| (0..map_w).map(|_| false).collect()).collect());
        }
    };

    let (ent_y, ent_x) = portals[&('A', 'A')];
    let exit = portals[&('Z', 'Z')];
    let mut visited = Vec::new();
    grow_visited(&mut visited, 2);
    visited[0][ent_y][ent_x] = true;
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), 0, 0, ent_y, ent_x));

    while let Some((_, dist, level, y, x)) = queue.pop() {
        if (y, x) == exit && level == 0 {
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
            if map[y][x] != Tile::Wall && !visited[level as usize][y][x] {
                grow_visited(&mut visited, level + 2);
                visited[level as usize][y][x] = true;
                queue.push((Reverse(new_dist), new_dist, level, y, x));
            }
        }

        if let Tile::Portal(y, x, inner) = map[y][x] {
            let new_level = if inner { level + 1 } else { level - 1 };

            if new_level >= 0 && !visited[new_level as usize][y][x] {
                grow_visited(&mut visited, new_level + 2);
                visited[new_level as usize][y][x] = true;
                queue.push((Reverse(new_dist), new_dist, new_level, y, x));
            }
        }
    }
}
