fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_memory: Vec<_> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();

    let mut computers: Vec<_> = (0..50).map(|i| {
        let mut computer = intcode::IntCode::new(initial_memory.clone());
        computer.set_default_input(-1);
        computer.put_input(i);
        computer
    }).collect();

    let mut packets = Vec::new();

    'outer: loop {
        for computer in &mut computers {
            computer.iterate();
            if let Some(packet) = computer.get_outputs(3) {
                if packet[0] == 255 {
                    println!("{}", packet[2]);
                    break 'outer;
                }

                packets.push(packet);
            }
        }

        for packet in &packets {
            for &i in &packet[1..] {
                computers[packet[0] as usize].put_input(i);
            }
        }

        packets.clear();
    }
}
