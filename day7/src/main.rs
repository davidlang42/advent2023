use std::fs;
use std::env;
use std::str::FromStr;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Set {
    hands: Vec<Hand>
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: usize
}

#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl FromStr for Set {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            hands: text.lines().map(|s| s.parse().unwrap()).collect()
        })
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = line.split(" ").collect();
        if words.len() != 2 {
            return Err(format!("Expected 2 words, found {}", words.len()));
        }
        Ok(Self {
            cards: words[0].chars().map(|c| Card::from(c)).collect::<Vec<Card>>(),
            bid: words[1].parse().unwrap()
        })
    }
}

impl Hand {
    fn group(&self) -> HashMap<Card, usize> {
        let mut map = HashMap::new();
        for card in &self.cards {
            if let Some(existing) = map.get(card) {
                map.insert(*card, existing + 1);
            } else {
                map.insert(*card, 1);
            }
        }
        map
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl Hand {
    fn hand_type(&self) -> Type {
        let g = self.group();
        match (g.len(), g.values().max().unwrap()) {
            (1, _) => Type::FiveOfAKind,
            (2, 4) => Type::FourOfAKind,
            (2, 3) => Type::FullHouse,
            (3, 3) => Type::ThreeOfAKind,
            (3, 2) => Type::TwoPair,
            (4, _) => Type::OnePair,
            (5, _) => Type::HighCard,
            _ => panic!("Type not found: {:?}", self.cards)
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        let type_cmp = self.hand_type().cmp(&other.hand_type());
        if type_cmp != Ordering::Equal {
            return Some(type_cmp);
        }
        for i in 0..self.cards.len() {
            let card_cmp = self.cards[i].cmp(&other.cards[i]);
            if card_cmp != Ordering::Equal {
                return Some(card_cmp);
            }
        }
        Some(self.bid.cmp(&other.bid))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl Card {
    fn from(c: char) -> Self {
        match c {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid card: {}", c)
        }
    }
}

impl Set {
    fn sort(&mut self) {
        self.hands.sort();
    }

    fn winnings(&self) -> usize {
        let mut total = 0;
        for i in 0..self.hands.len() {
            total += (i+1)*self.hands[i].bid;
        }
        total
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = &args[1];
        let text = fs::read_to_string(&filename)
            .expect(&format!("Error reading from {}", filename));
        let mut set: Set = text.parse().unwrap();
        set.sort();
        println!("Total winnings: {}", set.winnings());
    } else {
        println!("Please provide 1 argument: Filename");
    }
}
