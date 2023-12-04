use std::collections::HashSet;
use std::fs;
use std::env;
use std::str::FromStr;

struct Card {
    winning: HashSet<u32>,
    have: HashSet<u32>
}

impl Card {
    const TWO: usize = 2;
    fn points(&self) -> usize {
        let count = self.winning.intersection(&self.have).count();
        if count == 0 {
            0
        } else {
            Self::TWO.pow((count - 1).try_into().unwrap())
        }
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = line.split(":").last().unwrap().split("|").collect();
        if sections.len() != 2 {
            Err(format!("Expected 2 lists, found {}", sections.len()))
        } else {
            let winning = sections[0].split(" ").filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
            let have = sections[1].split(" ").filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
            Ok(Self {
                winning,
                have
            })
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let cards: Vec<Card> = text.lines().map(|s| s.parse().unwrap()).collect();
        let total: usize = cards.iter().map(Card::points).sum();
        println!("Total points: {}", total)
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
