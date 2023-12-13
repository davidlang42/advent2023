use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Ash,
    Rocks
}

#[derive(Debug, Eq, PartialEq)]
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

impl Line {
    fn diff(&self, other: &Self) -> usize {
        let mut diff = 0;
        for i in 0..self.0.len() {
            if self.0[i] != other.0[i] {
                diff += 1;
            }
        }
        diff
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
    fn mirror(&self, allow_smudge: bool, avoid_mirror: Option<&Mirror>) -> Option<Mirror> {
        for r in 0..(self.rows.len() - 1) {
            if Self::check_mirror(&self.rows, r, r + 1, allow_smudge) {
                let mirror = Mirror::AfterRow(r + 1);
                if let Some(avoid) = avoid_mirror {
                    if *avoid == mirror {
                        continue;
                    }
                }
                return Some(mirror);
            }
        }
        for c in 0..(self.cols.len() - 1) {
            if Self::check_mirror(&self.cols, c, c + 1, allow_smudge) {
                let mirror = Mirror::AfterColumn(c + 1);
                if let Some(avoid) = avoid_mirror {
                    if *avoid == mirror {
                        continue;
                    }
                }
                return Some(mirror);
            }
        }
        None
    }

    fn check_mirror(lines: &Vec<Line>, mut lower: usize, mut upper: usize, mut allow_smudge: bool) -> bool {
        while upper < lines.len() {
            let diff = lines[lower].diff(&lines[upper]);
            if diff == 1 && allow_smudge {
                allow_smudge = false;
            } else if diff > 0 {
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
        let mut sum1 = 0;
        let mut sum2 = 0;
        for pattern in patterns {
            let mirror1 = pattern.mirror(false, None).unwrap();
            let mirror2 = pattern.mirror(true, Some(&mirror1)).unwrap();
            sum1 += mirror1.number();
            sum2 += mirror2.number();
        }
        println!("Part 1: {}", sum1);
        println!("Part 2: {}", sum2);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
