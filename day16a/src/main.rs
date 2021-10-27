fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut values: Vec<_> = input.trim().chars().map(|c| c.to_digit(10).unwrap() as i16).collect();

    for _ in 0..100 {
        values = (1..=values.len()).map(|n| {
            values.iter().zip(
                [0, 1, 0, -1].iter().flat_map(move |num| std::iter::repeat(num).take(n)).cycle().skip(1)
            ).map(|(a, b)| a * b).sum::<i16>().abs() % 10
        }).collect();
    }

    println!(
        "{}",
        values.into_iter().take(8).map(|num| std::char::from_digit(num as u32, 10).unwrap()).collect::<String>(),
    );
}
