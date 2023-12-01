use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let values = text.lines().map(find_calibration_value);
        let sum: usize = values.sum();
        println!("Sum: {}", sum)
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn find_calibration_value(line: &str) -> usize {
    let mut first = None;
    let mut last = None;
    for c in line.chars() {
        if c.is_digit(10) {
            if first.is_none() {
                first = Some(c)
            }
            last = Some(c)
        }
    }
    let number = format!("{}{}", first.unwrap(), last.unwrap());
    number.parse().unwrap()
}