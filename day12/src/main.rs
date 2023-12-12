use std::fs;
use std::env;
use std::str::FromStr;

struct Report {
    broken: Vec<Option<bool>>,
    groups: Vec<usize>
}

impl FromStr for Report {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // ???.### 1,1,3
        let sections: Vec<&str> = line.split(" ").collect();
        if sections.len() != 2 {
            return Err(format!("Expected 2 sections, found {}", sections.len()));
        }
        let broken = sections[0].chars().map(|c| match c {
            '.' => Some(false),
            '#' => Some(true),
            '?' => None,
            _ => panic!("invalid char: {}", c)
        }).collect();
        let groups = sections[1].split(",").map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            broken,
            groups
        })
    }
}

impl Report {
    fn possible_combinations(&self) -> usize {
        Self::combinations(&self.broken, 0, &self.groups)
    }

    fn combinations(state: &Vec<Option<bool>>, mut index: usize, remaining: &[usize]) -> usize {
        if (state.len() as isize - index as isize) < remaining.iter().sum::<usize>() as isize + (remaining.len() as isize) - 1 {
            // not possible from here
            return 0;
        }
        if remaining.len() == 0 {
            // no more groups required
            let mut known_values = state.iter().skip(index).filter(|o| o.is_some()).map(|o| o.unwrap());
            if known_values.any(|b| b) {
                // but some are broken, therefore not possible
                return 0;
            } else {
                // none are broken, therefore set all to not broken and this is possible
                return 1;
            }
        }
        let mut group = 0;
        let mut group_start_index = index;
        while index < state.len() {
            if let Some(current) = state[index] {
                if current {
                    if group == 0 {
                        group_start_index = index;
                    }
                    group += 1;
                    if group > remaining[0] {
                        // group too big, not possible
                        return 0;
                    }
                } else if group > 0 {
                    if group != remaining[0] {
                        // group is wrong size, not possible
                        return 0;
                    } else {
                        // group is correct size, keep going
                        return Self::combinations(state, index, &remaining[1..]);
                    }
                }
            } else {
                // we found an unknown, try each option
                let mut new_state = state.clone();
                new_state[index] = Some(true);
                let mut combos = Self::combinations(&new_state, group_start_index, remaining);
                new_state[index] = Some(false);
                combos += Self::combinations(&new_state, group_start_index, remaining);
                return combos;
            }
            index += 1;
        }
        if group != remaining[0] {
            // group is wrong size (or zero), not possible
            return 0;
        } else {
            // group is correct size, this is valid
            return 1;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let records: Vec<Report> = text.lines().map(|s| s.parse().unwrap()).collect();
        let combos: Vec<usize> = records.iter().map(|r| r.possible_combinations()).collect();
        println!("Combos: {:?}", combos);
        let sum: usize = combos.iter().sum();
        println!("Total: {}", sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
