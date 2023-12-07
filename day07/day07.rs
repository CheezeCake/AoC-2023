use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io;
use std::str::FromStr;

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
enum Card {
    Joker,
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
    Ace,
}

impl Card {
    fn new(c: u8) -> Option<Self> {
        match c {
            b'A' => Some(Card::Ace),
            b'K' => Some(Card::King),
            b'Q' => Some(Card::Queen),
            b'J' => Some(Card::Jack),
            b'T' => Some(Card::Ten),
            b'9' => Some(Card::Nine),
            b'8' => Some(Card::Eight),
            b'7' => Some(Card::Seven),
            b'6' => Some(Card::Six),
            b'5' => Some(Card::Five),
            b'4' => Some(Card::Four),
            b'3' => Some(Card::Three),
            b'2' => Some(Card::Two),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOAK,
    FullHouse,
    FourOAK,
    FiveOAK,
}

#[derive(Debug, Clone, Copy)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn new(cards: &[Card; 5]) -> Self {
        let mut cloned: [Card; 5] = [Card::Ace; 5];
        cloned.clone_from_slice(cards);
        Hand { cards: cloned }
    }

    fn hand_type(&self) -> HandType {
        let mut card_counter: HashMap<Card, usize> = HashMap::new();
        for card in self.cards {
            card_counter
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if card_counter.contains_key(&Card::Joker) {
            let joker_count = *card_counter.get(&Card::Joker).unwrap();
            card_counter.remove(&Card::Joker);
            let max_count_card = *card_counter
                .iter()
                .max_by_key(|(_, count)| *count)
                .map(|(card, _)| card)
                .unwrap_or(&Card::Joker);
            card_counter
                .entry(max_count_card)
                .and_modify(|count| *count += joker_count)
                .or_insert(joker_count);
        }

        let mut counts: Vec<usize> = card_counter.iter().map(|(_, &count)| count).collect();
        counts.sort();

        match counts.as_slice() {
            [5] => HandType::FiveOAK,
            [1, 4] => HandType::FourOAK,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOAK,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct HandParseError;

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();
        if bytes.len() != 5 {
            return Err(HandParseError);
        }

        let cards: [Card; 5] = bytes
            .iter()
            .filter_map(|&b| Card::new(b))
            .collect::<Vec<Card>>()
            .try_into()
            .or(Err(HandParseError))?;
        Ok(Hand::new(&cards))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        match self.cmp(other) {
            Ordering::Equal => true,
            _ => false,
        }
    }
}

impl Eq for Hand {}

fn total_winnings(data: &mut Vec<(Hand, u64)>) -> u64 {
    data.sort_by_key(|(hand, _)| *hand);
    data.iter()
        .enumerate()
        .map(|(i, (_, bid))| (i + 1) as u64 * bid)
        .sum::<u64>()
}

fn main() {
    let mut data: Vec<(Hand, u64)> = io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let hand: Hand = line[..5].parse().expect("error parsing hand");
            let bid: u64 = line[6..].trim().parse().expect("error parsing bid");

            (hand, bid)
        })
        .collect();

    println!("part 1: {}", total_winnings(&mut data));

    for i in 0..data.len() {
        for j in 0..5 {
            if data[i].0.cards[j] == Card::Jack {
                data[i].0.cards[j] = Card::Joker;
            }
        }
    }
    println!("part 2: {}", total_winnings(&mut data));
}
