// Enums - Set of predefined variables

enum Direction {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn moving_direction(d: Direction) {
    // Perform action depending on info
    match d {
        Direction::Up => println!("Moving UP"),
        Direction::Down => println!("Moving DOWN"),
        Direction::Right => println!("Moving RIGHT"),
        Direction::Left => println!("Moving LEFT")
    }
}

pub fn run() {
    println!("----> Enums : See CLI/enums.rs <----");

    let direction1 = Direction::Left;
    let direction2 = Direction::Up;
    let direction3 = Direction::Right;
    let direction4 = Direction::Down;

    moving_direction(direction1);
    moving_direction(direction2);
    moving_direction(direction3);
    moving_direction(direction4);
}