use std::fs;
use std::env;
use std::str::FromStr;
use std::time::Instant;

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
    fn possible_combinations(&mut self) -> usize {
        Self::combinations(&mut self.broken, 0, 0, &self.groups)
    }

    fn combinations(state: &mut Vec<Option<bool>>, mut index: usize, mut group: usize, mut remaining: &[usize]) -> usize {
        if remaining.len() != 0 {
            let max_remaining_broken = state.iter().skip(index).filter(|o| o.is_none() || o.unwrap()).count() + group;
            let required_broken = remaining.iter().sum::<usize>();
            if max_remaining_broken < required_broken {
                // not enough BROKEN left to complete
                return 0;
            }
            let max_remaining_space = state.len() - index + group;
            let required_space = required_broken + remaining.len() - 1;
            if max_remaining_space < required_space {
                // not enough space left to complete
                return 0;
            }
        }
        //println!("START: index [{}], group={}, remaining {:?}", index, group, remaining);
        while index < state.len() {
            if remaining.len() == 0 {
                // no more groups required
                let mut known_values = state.iter().skip(index).filter(|o| o.is_some()).map(|o| o.unwrap());
                if known_values.any(|b| b) {
                    // but some are broken, therefore not possible
                    //println!("NOPE: no remaining, but has future groups");
                    return 0;
                } else if group > 0 {
                    // but group in progress, therefore not possible
                    //println!("NOPE: no remaining, but has current group");
                    return 0;
                } else {
                    // none are broken, therefore set all to not broken and this is possible
                    // let mut new_state = state.clone();
                    // for i in index..new_state.len() {
                    //     if new_state[i].is_none() {
                    //         new_state[i] = Some(false)
                    //     }
                    // }
                    //println!("YEP: no remaining, no values -- {}", format_solution(&new_state));
                    return 1;
                }
            }
            if let Some(current) = state[index] {
                if current {
                    group += 1;
                    if group > remaining[0] {
                        // group too big, not possible
                        //println!("NOPE: current group too big");
                        return 0;
                    }
                } else if group > 0 {
                    if group != remaining[0] {
                        // group is wrong size, not possible
                        //println!("NOPE: group finished, wrong size {} != {}", group, remaining[0]);
                        return 0;
                    } else {
                        // group is correct size, keep going
                        remaining = &remaining[1..];
                        //println!("CONTINUE: group finished size {}, now remaining {:?}", group, remaining);
                        group = 0;
                    }
                }
            } else {
                // we found an unknown, try each option
                //println!("EXPAND: found unknown at [{}]", index);
                state[index] = Some(true);
                let mut combos = Self::combinations(state, index, group, remaining);
                state[index] = Some(false);
                combos += Self::combinations(state, index, group, remaining);
                state[index] = None;
                return combos;
            }
            index += 1;
        }
        // we reached the end of the state
        if remaining.len() == 0 {
            if group == 0 {
                // no group to finish
                //println!("YEP: no remaining, no current group -- {}", format_solution(&state));
                return 1;
            } else {
                // we had a group in progress when we wanted none
                //println!("NOPE: no remaining, but has current group");
                return 0;
            }
        } else if group != remaining[0] {
            // group is wrong size (or zero), not possible
            //println!("NOPE: final group, wrong size {} != {}", group, remaining[0]);
            return 0;
        } else if remaining.len() == 1 {
            // group is correct size, and its the last group
            //println!("YEP: final group correct size -- {}", format_solution(&state));
            return 1;
        } else {
            // group is correct size, BUT THERE ARE MORE GROUPS
            //println!("NOPE: final group correct size, but more groups remain");
            return 0;
        }
    }

    fn unfold(&self) -> Self {
        let mut broken = Vec::new();
        let mut groups = Vec::new();
        for i in 0..5 {
            broken.append(&mut self.broken.clone());
            if i != 4 {
                broken.push(None);
            }
            groups.append(&mut self.groups.clone());
        }
        Self {
            broken,
            groups
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut reports: Vec<Report> = text.lines().map(|s| s.parse().unwrap()).collect();
        let combos: Vec<usize> = reports.iter_mut().map(|r| r.possible_combinations()).collect();
        // for i in 0..combos.len() {
        //     println!("Set [{}]", i);
        //     for j in 0..combos[i].len() {
        //         println!("   {}", combos[i][j]);
        //     }
        //     println!("");
        // }
        let sum: usize = combos.iter().sum();
        println!("Total: {}", sum);
        let mut new_reports: Vec<Report> = reports.iter().map(|r| r.unfold()).collect();
        println!("--UNFOLD--");
        let start = Instant::now();
        let new_combos: Vec<usize> = new_reports.iter_mut().map(|r| r.possible_combinations()).collect();
        let duration = start.elapsed();
        let new_sum: usize = new_combos.iter().sum();
        println!("Total: {} (calculated in {:.2}s)", new_sum, duration.as_secs_f64());

    } else {
        println!("Please provide 1 argument: Filename");
    }
}
