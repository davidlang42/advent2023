use std::fs;
use std::env;
use std::str::FromStr;

struct Schematic(Vec<Vec<Character>>);

#[derive(PartialEq)]
enum Character {
    None,
    Digit(u8),
    Symbol(char)
}

struct Number {
    row: usize,
    from_column: usize,
    to_column: usize,
    value: u32
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
            Self::Symbol(c)
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

    fn find_numbers(&self) -> Vec<Number> {
        let mut numbers = Vec::new();
        for r in 0..self.rows() {
            let mut start = None;
            let mut value: u32 = 0;
            for c in 0..self.columns() {
                if let Character::Digit(digit) = self.0[r][c] {
                    if start.is_none() {
                        start = Some(c);
                        value = digit as u32;
                    } else {
                        value = value * 10 + digit as u32;
                    }
                } else {
                    if let Some(s) = start {
                        numbers.push(Number {
                            value,
                            row: r,
                            from_column: s,
                            to_column: c - 1
                        });
                    }
                    start = None;
                }
            }
            if let Some(s) = start {
                numbers.push(Number {
                    value,
                    row: r,
                    from_column: s,
                    to_column: self.columns() - 1
                });
            }
        }
        numbers
    }

    fn contains_symbol(&self, min_r: usize, max_r: usize, min_c: usize, max_c: usize) -> bool {
        for r in min_r..(max_r + 1) {
            for c in min_c..(max_c + 1) {
                if let Character::Symbol(_) = self.0[r][c] {
                    return true;
                }
            }
        }
        false
    }

    fn find_gear_ratios(&self, parts: &Vec<Number>) -> Vec<u32> {
        let mut gears = Vec::new();
        for r in 0..self.rows() {
            for c in 0..self.columns() {
                if let Character::Symbol(symbol) = self.0[r][c] {
                    if symbol == '*' {
                        let adjacent: Vec<&Number> = parts.iter().filter(|p| p.is_adjacent_to(r, c)).collect();
                        if adjacent.len() == 2 {
                            let ratio: u32 = adjacent.iter().map(|p| p.value).product();
                            gears.push(ratio);
                        }
                    }
                }
            }
        }
        gears
    }
}

impl Number {
    fn is_part_of(&self, schematic: &Schematic) -> bool {
        let min_r = if self.row == 0 { self.row } else { self.row - 1 };
        let max_r = if self.row == schematic.rows() - 1 { self.row } else { self.row + 1 };
        let min_c = if self.from_column == 0 { self.from_column } else { self.from_column - 1 };
        let max_c = if self.to_column == schematic.columns() - 1 { self.to_column } else { self.to_column + 1 };
        schematic.contains_symbol(min_r, max_r, min_c, max_c)
    }

    fn is_adjacent_to(&self, row: usize, column: usize) -> bool {
        row.abs_diff(self.row) <= 1 && column as isize >= self.from_column as isize - 1 && column <= self.to_column + 1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let schematic: Schematic = text.parse().unwrap();
        println!("Dimensions: {}x{}", schematic.rows(), schematic.columns());
        let numbers = schematic.find_numbers();
        println!("Numbers: {}", numbers.len());
        let parts: Vec<Number> = numbers.into_iter().filter(|n| n.is_part_of(&schematic)).collect();
        println!("Parts: {}", parts.len());
        let parts_sum: u32 = parts.iter().map(|p| p.value).sum();
        println!("Parts sum: {}", parts_sum);
        let gears = schematic.find_gear_ratios(&parts);
        println!("Gears: {:?}", gears);
        let gears_sum: u32 = gears.iter().sum();
        println!("Gears sum: {}", gears_sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
