use std::io;

fn repeating(digits: &Vec<u8>) -> bool {
    let mut prev = digits[0];
    for &i in digits.iter().skip(1).take(digits.len() - 2) {
        if prev == i {
            return true;
        }
        prev = i;
    }
    false
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
                if repeating(&current) {
                    count += (end[n] - i + 1) as u32;
                } else {
                    count += 1;
                }
                break 'mainloop;
            } else if i < end[n] {
                break;
            }
        }

        if repeating(&current) {
            count += (10 - current[current.len() - 1]) as u32;
        } else {
            count += 1;
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
