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

    let mut count = 1;

    let (mut x, mut bottom_y) = (1..).find_map(|i| {
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

    let mut top_y = bottom_y;

    loop {
        count += bottom_y + 1 - top_y;

        x += 1;
        if x >= 50 {
            break;
        }
        while top_y < 50 && !point_pulled(initial_memory.clone(), x, top_y) {
            top_y += 1;
        }
        while bottom_y < 49 && point_pulled(initial_memory.clone(), x, bottom_y + 1) {
            bottom_y += 1;
        }
    }

    println!("{}", count);
}
