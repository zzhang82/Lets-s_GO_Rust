const MAXMUN: u8 = 20;//const value

//Abstruction
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let player_direction:Direction=Direction::Up;

    match player_direction{
        Direction::Up=>println!("We are up"),
        Direction::Down=>println!("We all the way down"),
        Direction::Left=>println!("We left of the left"),
        Direction::Right=>println!("We Right to the King")
    }
}