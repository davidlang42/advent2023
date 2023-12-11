use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(PartialEq)]
enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Start
}

struct Map {
    pipes: HashMap<Point, Pipe>
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Point {
    r: isize,
    c: isize
}

impl Point {
    fn north(&self) -> Point {
        Self { r: self.r - 1, c: self.c }
    }

    fn south(&self) -> Point {
        Self { r: self.r + 1, c: self.c }
    }

    fn east(&self) -> Point {
        Self { r: self.r, c: self.c + 1 }
    }

    fn west(&self) -> Point {
        Self { r: self.r, c: self.c - 1 }
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut pipes = HashMap::new();
        let mut r = 0;
        for line in text.lines() {
            let mut c = 0;
            for ch in line.chars() {
                if ch != '.' {
                    pipes.insert(Point { r: r as isize, c: c as isize }, Pipe::from(ch));
                }
                c += 1;
            }
            r += 1;
        }
        Ok(Self {
            pipes
        })
    }
}

impl Pipe {
    fn from(c: char) -> Self {
        match c {
            '|' => Self::NorthSouth,
            '-' => Self::EastWest,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            'S' => Self::Start,
            _ => panic!("Unknown char: {}", c)
        }
    }

    fn points(&self, p: &Point) -> Vec<Point> {
        match self {
            Self::NorthSouth => vec![p.north(), p.south()],
            Self::EastWest => vec![p.east(), p.west()],
            Self::NorthEast => vec![p.north(), p.east()],
            Self::NorthWest => vec![p.north(), p.west()],
            Self::SouthWest => vec![p.south(), p.west()],
            Self::SouthEast => vec![p.south(), p.east()],
            Self::Start => vec![p.north(), p.south(), p.east(), p.west()]
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let map: Map = text.parse().unwrap();
        let longest = map.loop_from_start();
        println!("Longest loop: {}", longest);
        println!("Farthest point: {}", (longest+1) / 2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}

impl Map {
    fn next(&self, previous: &Point, current: &Point) -> Option<Point> {
        if let Some(pipe) = self.pipes.get(current) {
            let points = pipe.points(current);
            if points.len() == 2 {
                if points[0] == *previous {
                    Some(points[1])
                } else if points[1] == *previous {
                    Some(points[0])
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn loop_from_start(&self) -> usize {
        let s = self.start();
        let mut options = Vec::new();
        for p in Pipe::Start.points(&s) {
            //println!("Start: {:?}, Next: {:?}", s, p);
            let longest = self.longest(&s, &p);
            //println!("Longest: {:?}", longest);
            if let Some(option) = longest {
                options.push(option);
            }
        }
        if options.len() == 0 {
            panic!("No options found");
        }
        options.into_iter().max().unwrap()
    }

    fn start(&self) -> Point {
        for (point, pipe) in &self.pipes {
            if *pipe == Pipe::Start {
                return point.clone();
            }
        }
        panic!("No start found");
    }

    fn longest(&self, start: &Point, next: &Point) -> Option<usize> {
        let mut previous = *start;
        let mut current = *next;
        let mut steps = 1;
        while current != *start {
            if let Some(next) = self.next(&previous, &current) {
                //println!("Next: {:?}", next);
                previous = current;
                current = next;
                steps += 1;
            } else {
                return None;
            }
        }
        Some(steps)
    }
}