use std::collections::HashMap;
use std::fs;
use std::env;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Round,
    Cube
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct Line(Vec<Tile>);

#[derive(Eq, PartialEq, Hash, Clone)]
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
            new_cols.push(col.shift_left());
        }
        Self::from_cols(new_cols)
    }

    fn tilt_west(&self) -> Self {
        let mut new_rows = Vec::new();
        for row in &self.rows {
            new_rows.push(row.shift_left());
        }
        Self::from_rows(new_rows)
    }

    fn tilt_south(&self) -> Self {
        let mut new_cols = Vec::new();
        for col in &self.cols {
            new_cols.push(col.shift_right());
        }
        Self::from_cols(new_cols)
    }

    fn tilt_east(&self) -> Self {
        let mut new_rows = Vec::new();
        for row in &self.rows {
            new_rows.push(row.shift_right());
        }
        Self::from_rows(new_rows)
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
    fn shift_left(&self) -> Self {
        let mut vec = self.0.clone();
        let mut rocks = 0;
        for i in (0..vec.len()).rev() {
            match vec[i] {
                Tile::Cube => {
                    if rocks > 0 {
                        for j in (i+1)..(i+1+rocks) {
                            vec[j] = Tile::Round;
                        }
                        rocks = 0;
                    }
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

    fn shift_right(&self) -> Self {
        let mut vec = self.0.clone();
        let mut rocks = 0;
        for i in 0..vec.len() {
            match vec[i] {
                Tile::Cube => {
                    if rocks > 0 {
                        for j in (i-rocks)..i {
                            vec[j] = Tile::Round;
                        }
                        rocks = 0;
                    }
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
        for j in (vec.len()-rocks)..vec.len() {
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
        let mut platform: Platform = text.parse().unwrap();
        let cycles = 1000000000;
        let mut cache = HashMap::new();
        let mut cycle = 0;
        while cycle < cycles {
            platform = platform.tilt_north();
            platform = platform.tilt_west();
            platform = platform.tilt_south();
            platform = platform.tilt_east();
            if let Some(previous) = cache.insert(platform.clone(), cycle) {
                println!("Found cycle from {} to {}", previous, cycle);
                let cadence = cycle - previous;
                while cycle < cycles {
                    cycle += cadence;
                }
                cycle -= cadence;
            }
            cycle += 1;
        }
        for row in &platform.rows {
            for col in &row.0 {
                print!("{}", match col {
                    Tile::Cube => '#',
                    Tile::Round => 'O',
                    Tile::Empty => '.'
                });
            }
            println!("");
        }
        println!("North load after {} cycles: {}", cycles, platform.north_load());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
