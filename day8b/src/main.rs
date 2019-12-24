fn main() {
    const HEIGHT: usize = 6;
    const WIDTH: usize = 25;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let pixels: Vec<char> = input.trim().chars().collect();
//     let mut image = [['2'; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
//         let mut line 
        for x in 0..WIDTH {
            for layer in 0..pixels.len() / (HEIGHT * WIDTH) {
                let pixel = pixels[layer * HEIGHT * WIDTH + y * WIDTH + x];
                if pixel == '1' {
                    print!("\u{2588}");
                    break;
                } else if pixel != '2' {
//                     image[y][x] = pixel;
                    print!(" ");
                    break;
                }
            }
        }
        println!("");
    }
}
