use std::fs;
use std::env;
use std::str::FromStr;

struct Raw(Vec<Vec<bool>>);

struct Image {
    galaxies: Vec<Point>,
    row_empty: Vec<bool>,
    col_empty: Vec<bool>
}

struct Point {
    r: usize,
    c: usize
}

impl FromStr for Raw {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        for line in text.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c == '#');
            }
            rows.push(row);
        }
        Ok(Self(rows))
    }
}

impl Image {
    fn from(raw: Raw) -> Self {
        let mut galaxies = Vec::new();
        let mut row_empty = Vec::new();
        let mut col_empty = Vec::new();
        for c in 0..raw.0[0].len() {
            col_empty.push(true);
        }
        for r in 0..raw.0.len() {
            let mut empty = true;
            for c in 0..raw.0[r].len() {
                if raw.0[r][c] {
                    empty = false;
                    galaxies.push(Point { r, c });
                    col_empty[c] = false;
                }
            }
            row_empty.push(empty);
        }
        Self {
            galaxies,
            row_empty,
            col_empty
        }
    }

    fn distance(&self, a: &Point, b: &Point) -> usize {
        let mut d = 0;
        if b.r > a.r {
            for r in a.r..b.r {
                if self.row_empty[r] {
                    d += 1000000;
                } else {
                    d += 1;
                }
            }
        } else {
            for r in b.r..a.r {
                if self.row_empty[r] {
                    d += 1000000;
                } else {
                    d += 1;
                }
            }
        }
        if b.c > a.c {
            for c in a.c..b.c {
                if self.col_empty[c] {
                    d += 1000000;
                } else {
                    d += 1;
                }
            }
        } else {
            for c in b.c..a.c {
                if self.col_empty[c] {
                    d += 1000000;
                } else {
                    d += 1;
                }
            }
        }
        d
    }

    fn pairs(&self) -> Vec<(&Point, &Point)> {
        let mut pairs = Vec::new();
        for i in 0..self.galaxies.len() {
            for j in (i+1)..self.galaxies.len() {
                pairs.push((&self.galaxies[i], &self.galaxies[j]));
            }
        }
        pairs
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let raw: Raw = text.parse().unwrap();
        let image = Image::from(raw);
        let mut sum = 0;
        for (a, b) in image.pairs() {
            let d = image.distance(a, b);
            //println!("Distance: {}", d);
            sum += d;
        }
        println!("Total: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
