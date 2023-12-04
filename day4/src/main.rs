use std::collections::HashSet;
use std::fs;
use std::env;
use std::str::FromStr;
use std::collections::VecDeque;

#[derive(Clone)]
struct Card {
    number: usize,
    winning: HashSet<u32>,
    have: HashSet<u32>
}

impl Card {
    const TWO: usize = 2;
    fn points(&self) -> usize {
        let count = self.wins();
        if count == 0 {
            0
        } else {
            Self::TWO.pow((count - 1).try_into().unwrap())
        }
    }

    fn wins(&self) -> usize {
        self.winning.intersection(&self.have).count()
    }
}

impl FromStr for Card {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let sections: Vec<&str> = line.split(":").collect();
        if sections.len() != 2 {
            return Err(format!("Expected 2 sections, found {}", sections.len()));
        }
        let number = sections[0].split(" ").last().unwrap().trim().parse().unwrap();
        let lists: Vec<&str> = sections[1].split("|").collect();
        if lists.len() != 2 {
            return Err(format!("Expected 2 lists, found {}", lists.len()));
        }
        let winning = lists[0].split(" ").filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
        let have = lists[1].split(" ").filter(|s| s.len() > 0).map(|s| s.parse().unwrap()).collect();
        Ok(Self {
            number,
            winning,
            have
        })
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
        println!("Total points: {}", total);
        // part2
        let mut scratchcards = 0;
        let mut remaining: VecDeque<Card> = cards.iter().map(|c| c.clone()).collect();
        while let Some(card) = remaining.pop_front() {
            scratchcards += 1;
            for i in card.number..(card.number + card.wins()) {
                remaining.push_back(cards[i].clone());
            }
        }
        println!("Total scratchcards: {}", scratchcards);
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
