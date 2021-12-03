use std::path::PathBuf;

// TODO need a constant for arrays but the length depends on the data!
// const DIAGNOSTIC_BITS: usize = 5;
const DIAGNOSTIC_BITS: usize = 12;

pub fn part1(input_path: &PathBuf) {

    let mut gamma_rate = String::new();
    let mut epsilon_rate = String::new();
    let mut ones: [u32; DIAGNOSTIC_BITS] = [0; DIAGNOSTIC_BITS];
    let mut zeros: [u32; DIAGNOSTIC_BITS] = [0; DIAGNOSTIC_BITS];

    let content = std::fs::read_to_string(input_path).expect("could not read file");
    for line in content.lines() {
        if line.len() > DIAGNOSTIC_BITS {
            panic!("Invalid diagnostic value: {}!", line);
        }

        for (i, bit) in line.chars().enumerate() {
            match bit {
                '0' => zeros[i] += 1,
                '1' => ones[i] += 1,
                _ => panic!("Invalid diagnostic bit {}!", bit),
            }
        }
    }

    // Could just calulate gamma and flip the bits for epsilon!
    for i in 0..DIAGNOSTIC_BITS {
        if ones[i] > zeros[i] {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        } else if zeros[i] > ones[i] {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            // TODO what if there are the same number?!
            panic!("TBC!");
        }
    }

    println!("Gamma rate = {}", gamma_rate);
    println!("Epsilon rate = {}", epsilon_rate);

    let decimal_gamma_rate = isize::from_str_radix(&gamma_rate, 2).unwrap();
    let decimal_epsilon_rate = isize::from_str_radix(&epsilon_rate, 2).unwrap();

    println!("Gamma rate = {}", decimal_gamma_rate);
    println!("Epsilon rate = {}", decimal_epsilon_rate);

    println!("Power consumption = {}", decimal_gamma_rate * decimal_epsilon_rate);
}

pub fn part2(_input_path: &PathBuf) {
    println!("TBC");
}
