use pancurses::Window;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

fn draw_tile(window: &Window, x: i64, y: i64, tile: i64) {
    window.mvaddstr((y + 1) as i32, x as i32, match tile {
        0 => " ",

        1 => "\u{2588}",

        2 => "\u{2592}",

        3 => "\u{2550}",

        4 => "0",

        _ => "",
    });
    window.refresh();
}

fn draw_score(window: &Window, score: i64) {
    window.mvprintw(0, 0, &format!("         "));
    window.mvprintw(0, 0, &format!("{}", score));
    window.refresh();
}

fn calc_expected_paddle_x(
    mut tiles: HashMap<(i64, i64), i64>,
    mut ball_pos: (i64, i64),
    mut ball_direction: (i64, i64),
    paddle_y: i64,
) -> i64 {
    while ball_pos.1 < paddle_y - 1 || ball_direction.1 <= 0 {
        let next_x_pos = (ball_pos.0 + ball_direction.0, ball_pos.1);
        let next_y_pos = (ball_pos.0, ball_pos.1 + ball_direction.1);
        let next_xy_pos = (ball_pos.0 + ball_direction.0, ball_pos.1 + ball_direction.1);
        if let Some(&tile) = tiles.get(&next_x_pos) {
            if tile == 2 {
                tiles.remove(&next_x_pos);
            }
            ball_direction.0 *= -1;
        } else if let Some(&tile) = tiles.get(&next_y_pos) {
            if tile == 2 {
                tiles.remove(&next_y_pos);
            }
            ball_direction.1 *= -1;
        } else if let Some(&tile) = tiles.get(&next_xy_pos) {
            if tile == 2 {
                tiles.remove(&next_xy_pos);
            }
            ball_direction.0 *= -1;
            ball_direction.1 *= -1;
        } else {
            ball_pos.0 += ball_direction.0;
            ball_pos.1 += ball_direction.1;
        }
    }
    ball_pos.0
}


fn main() {
    let display_simulation = std::env::args().any(|x| x == "--display");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut computer = intcode::IntCode::new(input.trim().split(',').map(|x| x.parse().unwrap()).collect());
    computer.set_at_address(0, 2);

    let mut tiles = HashMap::new();

    let window = if display_simulation { Some(pancurses::initscr()) } else { None };
    if let Some(_) = window {
        pancurses::curs_set(0);
    }

    let mut paddle_pos = (-1, -1);
    let mut ball_pos = (-1, -1);
    let mut ball_direction = (0, 0);
    let mut expected_paddle_x = -1;
    let mut score = -1;

    while computer.iterate() {
        if expected_paddle_x >= 0 {
            computer.set_input(match expected_paddle_x.cmp(&paddle_pos.0) {
                Ordering::Greater => 1,

                Ordering::Equal => 0,

                Ordering::Less => -1,
            });
        }
        while let Some(output) = computer.get_outputs(3) {
            if (output[0], output[1]) == (-1, 0) {
                score = output[2];
                if let Some(win) = &window {
                    draw_score(win, output[2]);
                }
            } else {
                if output[2] == 3 {
                    paddle_pos = (output[0], output[1]);
                } else if output[2] == 4 {
                    if ball_pos != (-1, -1) {
                        let mut do_calc_expected_paddle_x = ball_direction == (0, 0);
                        ball_direction = (output[0] - ball_pos.0, output[1] - ball_pos.1);
                        if ball_pos.1 == paddle_pos.1 - 1 {
                            ball_direction.1 = -1;
                            do_calc_expected_paddle_x = true;
                        }
                        if do_calc_expected_paddle_x {
                            expected_paddle_x = calc_expected_paddle_x(
                                tiles.clone(), ball_pos, ball_direction, paddle_pos.1,
                            );
                        }
                    }
                    ball_pos = (output[0], output[1]);
                } else if output[2] == 0 {
                    tiles.remove(&(output[0], output[1]));
                } else {
                    tiles.insert((output[0], output[1]), output[2]);
                }

                if let Some(win) = &window {
                    draw_tile(win, output[0], output[1], output[2]);
                }

                if output[2] == 4 && display_simulation {
                    sleep(Duration::from_millis(10));
                }
            }
        }
    }

    sleep(Duration::from_millis(1000));
    pancurses::endwin();
    println!("{}", score);
}
