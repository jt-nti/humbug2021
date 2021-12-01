use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};

pub fn part1(input_path: &PathBuf) {
    let mut total: u32 = 0;
    let mut previous: Option<u32> = None;

    let content = std::fs::read_to_string(input_path).expect("could not read file");
    for line in content.lines() {
        let depth: u32 = line.parse().expect("Not a number!");
        if let Some(value) = previous {
            if depth > value {
                println!("{} (increased)", depth);
                total += 1
            } else if depth < value {
                println!("{} (decreased)", depth);
            } else {
                println!("{} (no change)", depth);
            }
        } else {
            println!("{} (N/A - no previous measurement)", depth);
        }
        
        previous = Some(depth);
    }

    println!("Depth increased {} times", total);
}

pub fn part2(input_path: &PathBuf) {
    let mut total: u32 = 0;
    let mut previous: Option<u32> = None;

    let file = File::open(input_path).expect("could not open file");
    let depths = read_input(file).expect("could not read file");

    let window_size: usize = 3;
    let depths_size = depths.len();

    let mut window: usize = 0;
    while window <= depths_size - window_size {
        let sum: u32 = depths[window..window + window_size].iter().sum();
        if let Some(value) = previous {
            if sum > value {
                println!("{} (increased)", sum);
                total += 1
            } else if sum < value {
                println!("{} (decreased)", sum);
            } else {
                println!("{} (no change)", sum);
            }
        } else {
            println!("{} (N/A - no previous measurement)", sum);
        }
        previous = Some(sum);
        window += 1;
    }

    println!("Depth increased {} times", total);
}

fn read_input<R: Read>(io: R) -> Result<Vec<u32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}
