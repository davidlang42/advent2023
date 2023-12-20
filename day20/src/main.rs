use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug)]
struct Module {
    name: String,
    destinations: Vec<String>,
    module_type: ModuleType,
    flip_flop_state: bool,
    conjunction_state: HashMap<String, bool>
}

#[derive(Eq, PartialEq, Debug)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction
}

impl FromStr for Module {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = line.split(" -> ").collect();
        if sections.len() != 2 {
            return Err(format!("Expected 2 sections, found {}", sections.len()));
        }
        let mut name = sections[0].to_owned();
        let module_type = match name.chars().next().unwrap() {
            'b' => ModuleType::Broadcaster,
            '%' => ModuleType::FlipFlop,
            '&' => ModuleType::Conjunction,
            _ => panic!("Invalid module")
        };
        if module_type != ModuleType::Broadcaster {
            name = name[1..].to_owned();
        }
        let destinations = sections[1].split(", ").map(|s| s.to_owned()).collect();
        Ok(Self {
            name,
            destinations,
            module_type,
            flip_flop_state: false,
            conjunction_state: HashMap::new()
        })
    }
}

#[derive(Debug)]
struct ModuleSet {
    modules: HashMap<String, Module>,
    high_count: usize,
    low_count: usize
}

impl FromStr for ModuleSet {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let mut modules: HashMap<String, Module> = HashMap::new();
        for module in text.lines().map(|s| s.parse::<Module>().unwrap()) {
            let name = module.name.to_owned();
            modules.insert(name, module);
        }
        Ok(Self {
            modules,
            high_count: 0,
            low_count: 0
        })
    }
}

impl Module {
    fn process(&mut self, pulse: bool, from: String) -> Option<bool> {
        match self.module_type {
            ModuleType::Broadcaster => Some(pulse),
            ModuleType::FlipFlop => {
                if !pulse {
                    self.flip_flop_state = !self.flip_flop_state;
                    Some(self.flip_flop_state)
                } else {
                    None
                }
            },
            ModuleType::Conjunction => {
                self.conjunction_state.insert(from, pulse);
                Some(!self.conjunction_state.values().all(|b| *b))
            }
        }
    }
}

impl ModuleSet {
    fn press_button(&mut self) {
        let mut q: VecDeque<(String, bool, String)> = VecDeque::new();
        q.push_back(("button".to_owned(), false, "broadcaster".to_owned()));
        while let Some((from, pulse, to)) = q.pop_front() {
            if pulse {
                self.high_count += 1;
            } else {
                self.low_count += 1;
            }
            if let Some(module) = self.modules.get_mut(&to) {
                if let Some(next_pulse) = module.process(pulse, from) {
                    for dest in &module.destinations {
                        q.push_back((module.name.to_owned(), next_pulse, dest.to_owned()));
                    }
                }
            }
        }
    }

    fn reset(&mut self) {
        let mut conjunctions: HashMap<String, Vec<String>> = HashMap::new();
        for (name, module) in &self.modules {
            if module.module_type == ModuleType::Conjunction {
                let mut vec = Vec::new();
                for (sub_name, sub_module) in &self.modules {
                    if sub_module.destinations.iter().any(|d| d == name) {
                        vec.push(sub_name.to_owned());
                    }
                }
                conjunctions.insert(name.to_owned(), vec);
            }
        }
        for (name, sub_names) in conjunctions.into_iter() {
            for sub_name in sub_names {
                self.modules.get_mut(&name).unwrap().conjunction_state.insert(sub_name, false);
            }
        }
        self.high_count = 0;
        self.low_count = 0;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut modules: ModuleSet = text.parse().unwrap();
        modules.reset();
        for _ in 0..1000 {
            modules.press_button();
        }
        println!("Low count: {}", modules.low_count);
        println!("High count: {}", modules.high_count);
        println!("Multiplied: {}", modules.low_count * modules.high_count);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
