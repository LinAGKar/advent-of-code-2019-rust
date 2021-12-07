fn point_pulled(initial_memory: Vec<i64>, x: i64, y: i64) -> bool {
    let mut computer = intcode::IntCode::new(initial_memory);
    computer.put_input(x);
    computer.put_input(y);
    computer.run().unwrap() != 0
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_memory: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let (mut x, mut y) = (1..).find_map(|i| {
        (0..i).find_map(|j| {
            let x = i - j;
            let y = 1 + j;
            if point_pulled(initial_memory.clone(), x, y) {
                Some((x, y))
            } else {
                None
            }
        })
    }).unwrap();

    while !point_pulled(initial_memory.clone(), x + 99, y - 99) {
        x += 1;

        while point_pulled(initial_memory.clone(), x, y + 1) {
            y += 1;
        }
    }

    println!("{}", x * 10000 + (y - 99));
}
