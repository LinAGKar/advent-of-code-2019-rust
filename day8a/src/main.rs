fn main() {
    let height = 6;
    let width = 25;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut chars = input.trim().chars().peekable();

    let mut min_zeroes = std::u32::MAX;
    let mut product = 0;

    while let Some(_) = chars.peek() {
        let mut count = 0;
        let mut zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        while count < height * width {
            if let Some(i) = chars.next() {
                match i {
                    '0' => { zeroes += 1; },

                    '1' => { ones += 1; },

                    '2' => { twos += 1; },

                    _ => {},
                }
            } else {
                break;
            }
            count += 1;
        }
        if zeroes < min_zeroes {
            min_zeroes = zeroes;
            product = ones * twos;
        }
    }
    println!("{}", product);
}
