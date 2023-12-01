use std::{fs, env, collections::HashMap};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map = HashMap::from([
            //("zero", 0),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
            //("0", 0),
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
        ]);
        let values = text.lines().map(|s| find_calibration_value(s, &map));
        let sum: usize = values.sum();
        println!("Sum: {}", sum)
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

fn find_calibration_value(line: &str, map: &HashMap<&str, usize>) -> usize {
    let mut first_index = None;
    let mut first_value = None;
    let mut last_index = None;
    let mut last_value = None;
    for (key, value) in map.iter() {
        if let Some(i) = line.find(key) {
            if first_index.is_none() || first_index.unwrap() > i {
                first_index = Some(i);
                first_value = Some(value)
            }
            if last_index.is_none() || last_index.unwrap() < i {
                last_index = Some(i);
                last_value = Some(value);
            }
        }
    }
    first_value.unwrap() * 10 + last_value.unwrap()
}