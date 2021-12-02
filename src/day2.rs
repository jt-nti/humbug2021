use std::path::PathBuf;

#[derive(Debug)]
enum Command {
    Up(u32),
    Down(u32),
    Forward(u32),
}

pub fn part1(input_path: &PathBuf) {
    let mut position: u32 = 0;
    let mut depth: u32 = 0;

    let content = std::fs::read_to_string(input_path).expect("could not read file");
    for line in content.lines() {
        let mut split = line.split(' ');
        let direction = split.next().expect("Expected a direction");
        let distance: u32 = split.next().expect("Expected a distance").parse().expect("Not a number!");

        let command = match direction {
            "up" => Command::Up(distance),
            "down" => Command::Down(distance),
            "forward" => Command::Forward(distance),
            _ => panic!("Invalid command: {}!", direction)
        };

        match command {
            Command::Up(d) => depth -= d,
            Command::Down(d) => depth += d,
            Command::Forward(d) => position += d,
        }
    }

    println!("Position = {}", position);
    println!("Depth = {}", depth);
    println!("Position * Depth = {}", position * depth);
}

pub fn part2(_input_path: &PathBuf) {
    println!("TBC");
}
