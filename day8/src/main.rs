use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Debug)]
enum Direction {
    Left,
    Right
}

impl Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Self::Left,
            'R' => Self::Right,
            _ => panic!("Invalid direction")
        }
    }
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String
}

#[derive(Debug)]
struct Map(HashMap<String, Node>);

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        for line in text.lines() {
            //AAA = (BBB, CCC)
            map.insert(line[0..3].to_string(), Node { left: line[7..10].to_string(), right: line[12..15].to_string()});
        }
        Ok(Self(map))
    }
}

impl Map {
    fn steps(&self, from: &str, to: &str, directions: &Vec<Direction>) -> usize {
        let mut i = 0;
        let mut current = from;
        while current != to {
            let node = self.0.get(current).unwrap();
            current = match directions[i % directions.len()] {
                Direction::Left => &node.left,
                Direction::Right => &node.right
            };
            i += 1;
        }
        i
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let sections: Vec<&str> = text.split("\r\n\r\n").collect();
        let directions: Vec<Direction> = sections[0].chars().map(|c| Direction::from(c)).collect();
        let map: Map = sections[1].parse().unwrap();
        // println!("Directions: {:?}", directions);
        // println!("Map: {:?}", map);
        println!("AAA-ZZZ in {} steps", map.steps("AAA", "ZZZ", &directions))
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
