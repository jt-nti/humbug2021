// Advent of Code hackaty hack in Rust(ish)
//
// Based on https://rust-cli.github.io/book/index.html
//
mod day1;

use structopt::StructOpt;

const DAY: u8 = 1;

/// Process input for day one of the Advent of Code.
#[derive(Debug,StructOpt)]
struct Cli {
    /// The day
    day: u8,
    /// The part
    part: u8,
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    if (args.day < 1) || (args.day > DAY) {
        panic!("Invalid day! Day {} must be between 1 and {}", args.day, DAY);
    }

    if args.part == 1 {
        day1::part1(&args.path);
    } else if args.part == 2 {
        day1::part2(&args.path);
    } else {
        panic!("Invalid part! Part {} must be 1 or 2", args.part);
    }
    
}
