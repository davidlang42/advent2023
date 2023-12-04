use std::fs;
use std::env;
use std::str::FromStr;

struct Game {
    number: usize,
    rounds: Vec<Round>
}

struct Round {
    red: usize,
    green: usize,
    blue: usize
}

impl FromStr for Game {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let sections: Vec<&str> = line.split(":").collect();
        if sections.len() != 2 {
            return Err(format!("Expected 2 sections, found {}", sections.len()));
        }
        let number = sections[0].split(" ").last().unwrap().trim().parse().unwrap();
        let rounds = sections[1].split(";").map(|s| s.trim().parse().unwrap()).collect();
        Ok(Self {
            number,
            rounds
        })
    }
}

impl FromStr for Round {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let pairs: Vec<&str> = line.split(",").collect();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for pair in pairs {
            let words: Vec<&str> = pair.trim().split(" ").collect();
            if words.len() != 2 {
                return Err(format!("Expected 2 words: {}", pair));
            }
            let number = words[0].parse().unwrap();
            match words[1] {
                "red" => red = number,
                "green" => green = number,
                "blue" => blue = number,
                invalid => return Err(format!("Invalid color: {}", invalid))
            }
        }
        Ok(Self {
            red,
            green,
            blue
        })
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let games: Vec<Game> = text.lines().map(|s| s.parse().unwrap()).collect();
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut game_number_sum = 0;
        for game in games {
            let mut possible = true;
            for round in game.rounds {
                if round.red > max_red || round.green > max_green || round.blue > max_blue {
                    possible = false;
                }
            }
            if possible {
                game_number_sum += game.number;
            }
        }
        println!("Game number sum: {}", game_number_sum);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
