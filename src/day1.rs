use std::path::PathBuf;

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
    let window_size: usize = 3;
    let mut total: u32 = 0;
    let mut previous_window = Vec::with_capacity(window_size);
    let mut current_window = Vec::with_capacity(window_size);

    let content = std::fs::read_to_string(input_path).expect("could not read file");
    for line in content.lines() {
        let depth: u32 = line.parse().expect("Not a number!");

        current_window.push(depth);

        if current_window.len() == window_size {
            let current_sum: u32 = current_window.iter().sum();
            let mut previous_sum: Option<u32> = None;
            if previous_window.iter().len() == window_size {
                previous_sum = Some(previous_window.iter().sum());
            }

            if let Some(value) = previous_sum {
                if current_sum > value {
                    println!("{} (increased)", current_sum);
                    total += 1
                } else if current_sum < value {
                    println!("{} (decreased)", current_sum);
                } else {
                    println!("{} (no change)", current_sum);
                }
            } else {
                println!("{} (N/A - no previous sum)", current_sum);
            }

            previous_window = current_window.clone();
            current_window.remove(0);
        }

        
    }

    println!("Depth increased {} times", total);
}
