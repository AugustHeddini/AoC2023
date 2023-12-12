use std::{
    time::Instant,
    fs
};

enum CARD {
    TWO=2,
    THREE=3,
    FOUR=4,
    FIVE=5,
    SIX=6,
    SEVEN=7,
    EIGHT=8,
    NINE=9,
    T=10,
    J=11,
    Q=12,
    K=13,
    A=14
}

enum TYPE {
    FIVE_OF_A_KIND=7,
    FOUR_OF_A_KIND=6,
    FULL_HOUSE=5,
    THREE_OF_A_KIND=4,
    TWO_PAIR=3,
    PAIR=2,
    HIGH_CARD=1
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
            _ => panic!("Illegal card value")
        }
    }
}

struct HAND {
    cards: [CARD;5],
    evaluation: TYPE
}

impl From<&str> for HAND {
    fn from(value: &str) -> Self {
        HAND{
            cards: {value.chars().map(|ch| CARD::from(ch)).take(5).collect()},
            evaluation: {}
        }
    }
}

fn main() {
    let start = Instant::now();

    let input = fs::read_to_string("input").unwrap();

    for line in input.lines() {

    }


    println!("Elapsed: {:?}", start.elapsed());
}
