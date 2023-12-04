use std::fs;
use std::env;
use std::str::FromStr;

struct Schematic(Vec<Vec<Character>>);

#[derive(PartialEq)]
enum Character {
    None,
    Digit(u8),
    Symbol
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();
        for l in text.lines() {
            let mut row = Vec::new();
            for c in l.chars() {
                row.push(Character::from_char(c));
            }
            rows.push(row);
        }
        Ok(Self(rows))
    }
}

impl Character {
    fn from_char(c: char) -> Self {
        if c == '.' {
            Self::None
        } else if let Some(d) = c.to_digit(10) {
            Self::Digit(d.try_into().unwrap())
        } else {
            Self::Symbol
        }
    }
}

impl Schematic {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn columns(&self) -> usize {
        self.0[0].len()
    }

    fn find_part_numbers(&self) -> Vec<u32> {
        let mut parts = Vec::new();
        for r in 0..self.rows() {
            let mut start = None;
            let mut number: u32 = 0;
            for c in 0..self.columns() {
                if let Character::Digit(digit) = self.0[r][c] {
                    if start.is_none() {
                        start = Some(c);
                        number = digit as u32;
                    } else {
                        number = number * 10 + digit as u32;
                    }
                } else {
                    if let Some(s) = start {
                        let min_r = if r == 0 { r } else { r - 1 };
                        let max_r = if r == self.rows() - 1 { r } else { r + 1 };
                        let min_c = if s == 0 { s } else { s - 1 };
                        let max_c = if c == self.columns() - 1 { c } else { c + 1 };
                        if self.contains_symbol(min_r, max_r, min_c, max_c) {
                            parts.push(number);
                        }
                    }
                    start = None;
                }
            }
        }
        parts
    }

    fn contains_symbol(&self, min_r: usize, max_r: usize, min_c: usize, max_c: usize) -> bool {
        for r in min_r..(max_r + 1) {
            for c in min_c..(max_c + 1) {
                if self.0[r][c] == Character::Symbol {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let s: Schematic = text.parse().unwrap();
        println!("Dimensions: {}x{}", s.rows(), s.columns());
        let parts = s.find_part_numbers();
        println!("Parts: {:?}", parts);
        let sum: u32 = parts.iter().sum();
        println!("Sum: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
