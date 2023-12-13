use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Ash,
    Rocks
}

#[derive(Debug)]
enum Mirror {
    AfterRow(usize),
    AfterColumn(usize)
}

impl Mirror {
    fn number(&self) -> usize {
        match self {
            Self::AfterRow(r) => r * 100,
            Self::AfterColumn(c) => *c
        }
    }
}

#[derive(Eq, PartialEq)]
struct Line(Vec<Tile>);

struct Pattern {
    rows: Vec<Line>,
    cols: Vec<Line>
}

impl FromStr for Line {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let vec: Vec<Tile> = line.chars().map(|c| Tile::from(c)).collect();
        Ok(Self(vec))
    }
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Self::Rocks,
            '.' => Self::Ash,
            _ => panic!("Invalid char")
        }
    }
}
impl FromStr for Pattern {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Line> = text.lines().map(|l| l.parse().unwrap()).collect();
        let mut cols = Vec::new();
        for c in 0..rows[0].0.len() {
            let mut column = Vec::new();
            for row in &rows {
                column.push(row.0[c]);
            }
            cols.push(Line(column));
        }
        Ok(Self {
            rows,
            cols
        })
    }
}

impl Pattern {
    fn mirror(&self) -> Option<Mirror> {
        for r in 0..(self.rows.len() - 1) {
            if Self::check_mirror(&self.rows, r, r + 1) {
                return Some(Mirror::AfterRow(r + 1));
            }
        }
        for c in 0..(self.cols.len() - 1) {
            if Self::check_mirror(&self.cols, c, c + 1) {
                return Some(Mirror::AfterColumn(c + 1));
            }
        }
        return None
    }

    fn check_mirror(lines: &Vec<Line>, mut lower: usize, mut upper: usize) -> bool {
        while upper < lines.len() {
            if lines[lower] != lines[upper] {
                return false;
            }
            if lower == 0 {
                break;
            }
            lower -= 1;
            upper += 1;
        }
        return true;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let patterns: Vec<Pattern> = text.split("\r\n\r\n").map(|p| p.parse().unwrap()).collect();
        let mut sum = 0;
        for pattern in patterns {
            let mirror = pattern.mirror().unwrap();
            //println!("Mirror: {:?}", mirror);
            sum += mirror.number();
        }
        println!("Total: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
