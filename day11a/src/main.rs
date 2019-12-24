enum RobotState {
    Painting,
    Moving,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut computer = intcode::IntCode::new(input.trim().split(',').map(|x| x.parse().unwrap()).collect());

    let mut state = RobotState::Painting;
    let mut position = Point { x: 0, y: 0 };
    let mut direction = Point { x: 0, y: -1 };
    let mut painted_panels = std::collections::HashMap::new();

    while computer.iterate() {
        while let Some(output) = computer.get_output() {
            match state {
                RobotState::Painting => {
                    painted_panels.insert(position, output);
                    computer.set_input(output);
                    state = RobotState::Moving;
                }

                RobotState::Moving => {
                    match output {
                        1 => direction = Point { x: -direction.y, y: direction.x },

                        _ => direction = Point { x: direction.y, y: -direction.x },
                    }
                    position += direction;
                    computer.set_input(*painted_panels.get(&position).unwrap_or(&0));
                    state = RobotState::Painting;
                }
            }
        }
    }

    println!("{}", painted_panels.len());
}
