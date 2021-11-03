fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_memory: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut computer = intcode::IntCode::new(initial_memory);
    let mut map = Vec::new(); 
    let mut row = Vec::new();

    while let Some(output) = computer.run() {
        let c = output as u8;
        if c != '\n' as u8 {
            row.push(c);
        } else if !row.is_empty() {
            map.push(row);
            row = Vec::new();
        }
    }

    println!("{}", map.iter().enumerate().skip(1).take(map.len() - 2).flat_map(|(y, row)| {
        let map = &map;

        (1..row.len() - 1).filter_map(move |x| {
            if [
                (0, 1),
                (2, 1),
                (1, 1),
                (1, 0),
                (1, 2),
            ].iter().all(|(dx, dy)| map[y + 1 - dy][x + 1 - dx] == '#' as u8) {
                Some(x * y)
            } else {
                None
            }
        })
    }).sum::<usize>());
}
