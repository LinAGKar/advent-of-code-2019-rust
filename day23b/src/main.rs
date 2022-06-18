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
    let mut last_zero_y = 0;
    let mut nat_mem = [0, 0];

    'outer: loop {
        for computer in &mut computers {
            computer.iterate();
            if let Some(packet) = computer.get_outputs(3) {
                if packet[0] == 255 {
                    nat_mem = [packet[1], packet[2]];
                } else {
                    packets.push(packet);
                }
            }
        }

        for packet in &packets {
            for &i in &packet[1..] {
                computers[packet[0] as usize].put_input(i);
            }
        }

        packets.clear();

        if computers.iter().all(|computer| computer.waiting_for_input()) {
            if nat_mem[1] == last_zero_y {
                println!("{}", last_zero_y);
                break 'outer;
            }

            last_zero_y = nat_mem[1];

            for &i in &nat_mem {
                computers[0].put_input(i);
            }
        }
    }
}
