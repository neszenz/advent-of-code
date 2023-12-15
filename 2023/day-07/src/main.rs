use std::collections::HashMap;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    Joker = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Queen = 11,
    King = 12,
    Ace = 13,
}

impl Card {
    fn parse(value: char) -> Option<Card> {
        match value {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            'J' => Some(Card::Joker),
            _ => None
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Debug)]
struct Hand ([Card;5]);

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type() == other.hand_type() && self.0 == other.0
    }
}

impl Eq for Hand { }

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.hand_type() != other.hand_type() {
            self.hand_type().partial_cmp(&other.hand_type())
        }
        else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type() != other.hand_type() {
            self.hand_type().cmp(&other.hand_type())
        }
        else {
            self.0.cmp(&other.0)
        }
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let card_map: HashMap<Card, i32> = {
            let mut tmp = HashMap::new();

            self.0
                .iter()
                .for_each(|card| {
                    if tmp.contains_key(card) {
                        *tmp.get_mut(card).unwrap() += 1;
                    }
                    else {
                        tmp.insert(*card, 1);
                    }
                });

            tmp
        };

        assert!(!card_map.is_empty());
        assert_eq!(card_map.values().sum::<i32>(), 5);

        let n_joker = card_map.get(&Card::Joker).unwrap_or(&0);
        let jokerless_card_map: HashMap<&Card, &i32> = card_map.iter().filter(|(k,_)| **k != Card::Joker).collect();

        let is_five_of_a_kind = {
            let without_joker = card_map.values().any(|v| *v == 5);
            let with_joker = jokerless_card_map.values().any(|v| **v == (5 - n_joker));

            without_joker || with_joker
        };

        let is_four_of_a_kind = {
            let without_joker = card_map.values().any(|v| *v == 4);
            let with_joker = jokerless_card_map.values().any(|v| **v == (4 - n_joker));

            !is_five_of_a_kind && (without_joker || with_joker)
        };

        let is_full_house = {
            let without_joker = card_map.values().any(|v| *v == 3) && card_map.values().any(|v| *v == 2);
            let with_joker = jokerless_card_map.len() == 2 && jokerless_card_map.values().all(|v| **v == 2);

            !is_four_of_a_kind && (without_joker || with_joker)
        };

        let is_three_of_a_kind = {
            let without_joker = card_map.values().any(|v| *v == 3);
            let with_joker = jokerless_card_map.values().any(|v| **v == (3 - n_joker));

            !is_full_house && (without_joker || with_joker)
        };

        let is_two_pair = {
            let without_joker = card_map.values().filter(|v| **v == 2).count() == 2;
            let with_joker = false; // impossible

            !is_three_of_a_kind && (without_joker || with_joker)
        };

        let is_one_pair = {
            let without_joker = card_map.values().filter(|v| **v == 2).count() == 1;
            let with_joker = n_joker >= &1;

            !is_two_pair && (without_joker || with_joker)
        };

        if  is_five_of_a_kind {
            HandType::FiveOfAKind
        }
        else if is_four_of_a_kind {
            HandType::FourOfAKind
        }
        else if is_full_house {
            HandType::FullHouse
        }
        else if is_three_of_a_kind {
            HandType::ThreeOfAKind
        }
        else if is_two_pair {
            HandType::TwoPair
        }
        else if is_one_pair {
            HandType::OnePair
        }
        else {
            HandType::HighCard
        }
    }
}

#[derive(Debug)]
struct Bid (usize);

struct HandAndBid {
    hand: Hand,
    bid: Bid,
}

impl HandAndBid {
    fn parse(input: &str) -> HandAndBid {
        let tmp: Vec<&str> = input.split(' ').collect();
        assert_eq!(tmp.len(), 2);

        let hand = Hand(
            tmp[0]
                .chars()
                .map(|c| Card::parse(c).unwrap())
                .collect::<Vec<Card>>()
                .try_into()
                .unwrap()
        );

        let bid = Bid(tmp[1].parse().unwrap());

        HandAndBid { hand, bid }
    }
}

#[test]
fn example() {
    static EXAMPLE_INPUT: &str = include_str!("../res/example");
    static EXAMPLE_ANSWER: usize = 5905;

    let mut hands_with_bids: Vec<HandAndBid> = EXAMPLE_INPUT
        .lines()
        .map(|l| HandAndBid::parse(l))
        .collect();

    hands_with_bids.sort_by(|lhs, rhs| lhs.hand.cmp(&rhs.hand));

    let result: usize = hands_with_bids
        .iter()
        .enumerate()
        .map(|(i, hb)| (i+1) * hb.bid.0)
        .sum();

    assert_eq!(result, EXAMPLE_ANSWER);
}

fn main() {
    static INPUT: &str = include_str!("../res/input");

    let mut hands_with_bids: Vec<HandAndBid> = INPUT
        .lines()
        .map(|l| HandAndBid::parse(l))
        .collect();

    hands_with_bids.sort_by(|lhs, rhs| lhs.hand.cmp(&rhs.hand));

    let result: usize = hands_with_bids
        .iter()
        .enumerate()
        .map(|(i, hb)| (i+1) * hb.bid.0)
        .sum();

    println!("result={result}");
}
