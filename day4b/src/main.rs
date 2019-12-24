use std::io;

enum Reps {
    NoRep,
    InternalRep,
    RepAtEnd,
    Triple,
}

fn repeating(digits: &Vec<u8>) -> Reps {
    let mut prev = digits[0];
    let mut streak = 0;
    for &i in digits.iter().skip(1).take(digits.len() - 2) {
        if prev == i {
            streak += 1;
        } else {
            if streak == 1 {
                return Reps::InternalRep;
            }
            streak = 0;
        }
        prev = i;
    }
    if streak == 1 {
        Reps::RepAtEnd
    } else if streak == 0 {
        Reps::NoRep
    } else {
        Reps::Triple
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let mut limits = input.trim().split('-').map(|x| x.chars().map(|y| y.to_digit(10).unwrap() as u8).collect());
    let mut current: Vec<u8> = limits.next().unwrap();
    for i in 1..current.len() {
        if current[i] < current[i - 1] {
            current[i] = current[i - 1];
        }
    }
    let end: Vec<u8> = limits.next().unwrap();
    let mut count: u32 = 0;

    'mainloop: loop {
        for (n, &i) in current.iter().enumerate() {
            if i > end[n] {
                break 'mainloop;
            } else if n == current.len() - 1 {
                match repeating(&current) {
                    Reps::NoRep => count += 1,

                    Reps::InternalRep => count += (end[n] - i + 1) as u32,

                    Reps::Triple => {}
                    
                    Reps::RepAtEnd => count += (end[n] - i) as u32,
                }
                break 'mainloop;
            } else if i < end[n] {
                break;
            }
        }

        match repeating(&current) {
            Reps::NoRep => count += 1,

            Reps::InternalRep => count += (10 - current[current.len() - 1]) as u32,

            Reps::Triple => {}
            
            Reps::RepAtEnd => count += (9 - current[current.len() - 1]) as u32,
        }

        let last = current.last_mut().unwrap();
        *last = 9;
        for i in current.iter_mut().rev() {
            *i += 1;
            if *i > 9 {
                *i = 0;
            } else {
                break;
            }
        }

        for i in 1..current.len() {
            if current[i] < current[i - 1] {
                current[i] = current[i - 1];
            }
        }
    }

    println!("{}", count);
}
