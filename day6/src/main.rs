use std::fs;
use std::env;

struct Race {
    time: usize,
    distance: usize
}

fn parse(text: &str) -> Vec<Race> {
    let lines: Vec<&str> = text.lines().collect();
    if lines.len() != 2 {
        panic!("Expected 2 lines, found {}", lines.len());
    }
    let times: Vec<usize> = lines[0].split(":").nth(1).unwrap().split(" ").filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
    let distances: Vec<usize> = lines[1].split(":").nth(1).unwrap().split(" ").filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
    if times.len() != distances.len() {
        panic!("Distance and time dont match");
    }
    let mut races = Vec::new();
    for i in 0..times.len() {
        races.push(Race {
            distance: distances[i],
            time: times[i]
        });
    }
    races
}

impl Race {
    fn min_to_beat(&self) -> usize {
        for i in 0..self.time {
            let distance = i * (self.time - i);
            if distance > self.distance {
                return i;
            }
        }
        panic!("Could not min beat");
    }

    fn max_to_beat(&self) -> usize {
        for i in (0..self.time).rev() {
            let distance = i * (self.time - i);
            if distance > self.distance {
                return i;
            }
        }
        panic!("Could not max beat");
    }

    fn ways_to_beat(&self) -> usize {
        self.max_to_beat() - self.min_to_beat() + 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let races = parse(&text);
        let product: usize = races.iter().map(|r| r.ways_to_beat()).product::<usize>();
        println!("Product: {}", product);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
