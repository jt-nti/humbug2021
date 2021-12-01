// Advent of Code hackaty hack in Rust(ish)
//
// Based on https://rust-cli.github.io/book/index.html
//
use structopt::StructOpt;

/// Process input for day one of the Advent of Code.
#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let mut total: u32 = 0;
    let mut previous: Option<u32> = None;

    let content = std::fs::read_to_string(&args.path).expect("could not read file");
    for line in content.lines() {
        let depth: u32 = line.parse().expect("Not a number!");
        if let Some(value) = previous {
            if depth > value {
                println!("{} (increased)", depth);
                total += 1
            } else if depth < value {
                println!("{} (decreased)", depth);
            } else {
                println!("{} (unchanged)", depth);
            }
        } else {
            println!("{} (N/A - no previous measurement)", depth);
        }
        
        previous = Some(depth);
    }

    println!("Depth increased {} times", total);
}
