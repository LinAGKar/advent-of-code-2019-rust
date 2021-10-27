fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut values: Vec<_> = std::iter::repeat(input.trim().chars().map(|c| c.to_digit(10).unwrap() as u8)).take(10000).flatten().collect();
    let offset = values.iter().take(7).fold(0, |acc, &val| acc * 10 + val as usize);
    assert!(offset > values.len() / 2);
    values = values[offset..].to_vec();
    let mut new_values = vec![0; values.len()];

    for _ in 0..100 {
        values.iter().zip(new_values.iter_mut()).rev().fold(0, |acc, (&val, new_val)| {
            let sum = acc + val as u32;
            *new_val = (sum % 10) as u8;
            sum
        });
        std::mem::swap(&mut values, &mut new_values);
    }

    println!(
        "{}",
        values.into_iter().take(8).map(|num| std::char::from_digit(num as u32, 10).unwrap()).collect::<String>(),
    );
}
