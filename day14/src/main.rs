use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Round,
    Cube
}

#[derive(Eq, PartialEq)]
struct Line(Vec<Tile>);

struct Platform {
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
            '#' => Self::Cube,
            '.' => Self::Empty,
            'O' => Self::Round,
            _ => panic!("Invalid tile")
        }
    }
}

impl FromStr for Platform {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Line> = text.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self::from_rows(rows))
    }
}

impl Platform {
    fn from_rows(rows: Vec<Line>) -> Self {
        let mut cols = Vec::new();
        for c in 0..rows[0].0.len() {
            let mut column = Vec::new();
            for row in &rows {
                column.push(row.0[c]);
            }
            cols.push(Line(column));
        }
        Self {
            rows,
            cols
        }
    }

    fn from_cols(cols: Vec<Line>) -> Self {
        let mut rows = Vec::new();
        for r in 0..cols[0].0.len() {
            let mut row = Vec::new();
            for col in &cols {
                row.push(col.0[r]);
            }
            rows.push(Line(row));
        }
        Self {
            rows,
            cols
        }
    }

    fn tilt_north(&self) -> Self {
        let mut new_cols = Vec::new();
        for col in &self.cols {
            new_cols.push(col.shift_up());
        }
        Self::from_cols(new_cols)
    }

    fn north_load(&self) -> usize {
        let mut row_load = 1;
        let mut total_load = 0;
        for row in self.rows.iter().rev() {
            let round_rocks = row.0.iter().filter(|r| **r == Tile::Round).count();
            total_load += round_rocks * row_load;
            row_load += 1;
        }
        total_load
    }
}

impl Line {
    fn shift_up(&self) -> Self {
        let mut vec = self.0.clone();
        let mut rocks = 0;
        for i in (0..vec.len()).rev() {
            match vec[i] {
                Tile::Cube => {
                    for j in (i+1)..(i+1+rocks) {
                        vec[j] = Tile::Round;
                    }
                    rocks = 0;
                },
                Tile::Empty => {
                    // do nothing
                },
                Tile::Round => {
                    rocks += 1;
                    vec[i] = Tile::Empty;
                }
            }
        }
        for j in 0..rocks {
            vec[j] = Tile::Round;
        }
        Self(vec)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let original: Platform = text.parse().unwrap();
        let tilted = original.tilt_north();
        println!("Norht load: {}", tilted.north_load());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
