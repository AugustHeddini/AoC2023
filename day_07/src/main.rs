use std::{collections::HashSet, fs, time::Instant};

#[derive(PartialOrd, PartialEq, Eq, Hash, Debug)]
enum CARD {
    J = 1, // Part 2
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    T = 10,
    // J=11,    // Part 1
    Q = 12,
    K = 13,
    A = 14,
}

#[derive(PartialOrd, PartialEq, Eq, Debug)]
enum TYPE {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    Pair = 2,
    HighCard = 1,
}

impl From<char> for CARD {
    fn from(value: char) -> Self {
        match value {
            'T' => CARD::T,
            'J' => CARD::J,
            'Q' => CARD::Q,
            'K' => CARD::K,
            'A' => CARD::A,
            '2' => CARD::TWO,
            '3' => CARD::THREE,
            '4' => CARD::FOUR,
            '5' => CARD::FIVE,
            '6' => CARD::SIX,
            '7' => CARD::SEVEN,
            '8' => CARD::EIGHT,
            '9' => CARD::NINE,
            _ => panic!("Illegal card value"),
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, Debug)]
struct HAND {
    cards: Vec<CARD>,
    pub evaluation: TYPE,
}

impl From<&str> for HAND {
    fn from(value: &str) -> Self {
        let cards: Vec<CARD> = value.chars().map(|ch| CARD::from(ch)).collect();
        HAND {
            // evaluation: { evaluate_hand(&cards) },      // Part 1
            evaluation: { evaluate_hand_part2(&cards) }, // Part 2
            cards,
        }
    }
}

impl Ord for HAND {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.evaluation == other.evaluation {
            for (a, b) in self.cards.iter().zip(&other.cards) {
                if a == b {
                    continue;
                }
                return a.partial_cmp(&b).unwrap();
            }
        }
        return self.evaluation.partial_cmp(&other.evaluation).unwrap();
    }
}

fn evaluate_hand(cards: &Vec<CARD>) -> TYPE {
    let mut uniques = HashSet::new();
    for card in cards {
        uniques.insert(card);
    }

    return match uniques.len() {
        1 => TYPE::FiveOfAKind,
        2 => {
            let count = cards.iter().filter(|card| **card == cards[0]).count();
            if count == 4 || count == 1 {
                TYPE::FourOfAKind
            } else {
                TYPE::FullHouse
            }
        }
        3 => {
            let max = cards
                .iter()
                .map(|card| cards.iter().filter(|card2| *card2 == card).count())
                .max()
                .unwrap();
            if max == 3 {
                TYPE::ThreeOfAKind
            } else {
                TYPE::TwoPair
            }
        }
        4 => TYPE::Pair,
        5 => TYPE::HighCard,
        _ => panic!("Invalid number of cards in hand!"),
    };
}

fn evaluate_hand_part2(cards: &Vec<CARD>) -> TYPE {
    let mut uniques = HashSet::new();
    let cards_sans_joker: Vec<&CARD> = cards.iter().filter(|c| **c != CARD::J).collect();
    let jokers = cards.iter().filter(|card| **card == CARD::J).count();

    if cards_sans_joker.is_empty() {
        return TYPE::FiveOfAKind;
    }

    for card in &cards_sans_joker {
        uniques.insert(card);
    }

    let max = cards_sans_joker
        .iter()
        .map(|card| {
            cards_sans_joker
                .iter()
                .filter(|card2| *card2 == card)
                .count()
        })
        .max()
        .unwrap();

    if max + jokers == 5 {
        return TYPE::FiveOfAKind;
    }
    if max + jokers == 4 {
        return TYPE::FourOfAKind;
    }
    if max + jokers == 3 {
        if uniques.len() == 2 {
            return TYPE::FullHouse;
        }
        return TYPE::ThreeOfAKind;
    }
    if max + jokers == 2 {
        if uniques.len() == 3 {
            return TYPE::TwoPair;
        }
        return TYPE::Pair;
    }
    return TYPE::HighCard;
}

fn part1() {
    let input = fs::read_to_string("input").unwrap();

    let mut hands_scores: Vec<(HAND, i32)> = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        hands_scores.push((
            HAND::from(split.next().unwrap()),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    hands_scores.sort_by(|(a, _), (b, _)| a.cmp(&b));

    let mut res: i64 = 0;
    for (i, (_, score)) in hands_scores.iter().enumerate() {
        res += (i as i64 + 1) * *score as i64;
    }

    println!("Total Winnings: {:?}", res);
}

fn part2() {
    let input = fs::read_to_string("input").unwrap();

    let mut hands_scores: Vec<(HAND, i32)> = vec![];
    for line in input.lines() {
        let mut split = line.split_whitespace();
        hands_scores.push((
            HAND::from(split.next().unwrap()),
            split.next().unwrap().parse().unwrap(),
        ));
    }

    hands_scores.sort_by(|(a, _), (b, _)| a.cmp(&b));

    let mut res: i64 = 0;
    for (i, (_, score)) in hands_scores.iter().enumerate() {
        res += (i as i64 + 1) * *score as i64;
    }

    println!("Total Winnings: {:?}", res);
}

fn main() {
    let start = Instant::now();

    // part1();
    part2();

    println!("Elapsed: {:?}", start.elapsed());
}
