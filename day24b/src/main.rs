use std::io::Read;

const GENERATIONS: usize = 200;
const MARGIN: usize = GENERATIONS / 2 + 2;
const SIZE: usize = MARGIN * 2 + 1;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut map = [[[false; 5]; 5]; SIZE];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[MARGIN][y][x] = c == '#';
        }
    }

    let mut start = MARGIN;
    let mut end = MARGIN + 1;

    let adjacent: Vec<Vec<_>> = (0isize..5).map(|y| (0isize..5).map(move |x| {
        let mut adjacent = Vec::new();

        for (dy, dx) in [
            (0, 1),
            (0, -1),
            (1, 0),
            (-1, 0),
        ] {
            let (new_y, new_x) = (y + dy, x + dx);
            if new_x < 0 || new_y < 0 || new_x >= 5 || new_y >= 5 {
                adjacent.push((-1, 2 + dy, 2 + dx));
            } else if (new_y, new_x) == (2, 2) {
                adjacent.extend((-2..3).map(|i| {
                    (1, 2 - 2 * dy + i * dx.abs(), 2 - 2 * dx + i * dy.abs())
                }));
            } else {
                adjacent.push((0, new_y, new_x));
            }
        }

        adjacent
    }).collect()).collect();

    for _ in 0..GENERATIONS {
        let iter_start = start - 1;
        let iter_end = end + 1;

        start = usize::MAX;
        end = usize::MIN;

        let mut new_map = [[[false; 5]; 5]; SIZE];

        for depth in iter_start..iter_end {
            for y in 0..5 {
                for x in 0..5 {
                    if (y, x) == (2, 2) {
                        continue;
                    }

                    let adjacent_bugs = adjacent[y][x].iter().filter(|&&(dd, other_y, other_x)| {
                        let other_depth = depth as isize + dd;
                        map[usize::try_from(other_depth).unwrap()][other_y as usize][other_x as usize]
                    }).count();

                    let was_bug_here = map[depth][y][x];
                    let bug_here = was_bug_here && adjacent_bugs == 1 ||
                                   !was_bug_here && [1, 2].contains(&adjacent_bugs);
                    if bug_here {
                        start = std::cmp::min(depth, start);
                        end = std::cmp::max(depth + 1, end);
                    }
                    new_map[depth][y][x] = bug_here;
                }
            }
        }

        map = new_map;
    }

    println!("{}", (start..end).flat_map(|depth| map[depth].iter().flat_map(|row| {
        row.iter().filter(|&&tile| tile)
    })).count());
}
