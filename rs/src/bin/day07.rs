use aoc;
use nom::character::complete::one_of;
use nom::character::complete::space1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::count;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Data, String> {
    Ok(Data(
        input
            .lines()
            .map(Hand::parse)
            .map(|r| match r {
                Err(e) => Err(format!("{e}")),
                Ok((_, v)) => Ok(v),
            })
            .collect::<Result<_, _>>()?,
    ))
}

#[derive(Debug)]
struct Data(Vec<Hand>);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    bet: u64,
    hand_type: HandType,
}

impl Hand {
    fn new(cards: Vec<Card>, bet: u64) -> Self {
        let hand_type = HandType::new(&cards);

        Self {
            cards,
            bet,
            hand_type,
        }
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        all_consuming(map(
            separated_pair(count(Card::parse, 5), space1, nom::character::complete::u64),
            |(cards, bet)| Hand::new(cards, bet),
        ))(i)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let by_type = self.hand_type.cmp(&other.hand_type);
        if let Ordering::Equal = by_type {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .fold(Ordering::Equal, |acc, (s, o)| match acc {
                    Ordering::Equal => s.cmp(o),
                    _ => acc,
                })
        } else {
            by_type
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(cards: &[Card]) -> Self {
        let mut ordered_cards = cards.to_vec().clone();
        ordered_cards.sort();

        let mut hand_types: Vec<HandType> = vec![];
        let last = ordered_cards
            .iter()
            .fold((None, Card::N2), |acc, c| match acc {
                (None, _) => (Some(HandType::HighCard), c.clone()),
                (Some(HandType::HighCard), cp) => {
                    if *c == cp {
                        (Some(HandType::OnePair), c.clone())
                    } else {
                        hand_types.push(HandType::HighCard);
                        (Some(HandType::HighCard), c.clone())
                    }
                }
                (Some(HandType::OnePair), cp) => {
                    if *c == cp {
                        (Some(HandType::ThreeOfAKind), c.clone())
                    } else {
                        hand_types.push(HandType::OnePair);
                        (Some(HandType::HighCard), c.clone())
                    }
                }
                (Some(HandType::ThreeOfAKind), cp) => {
                    if *c == cp {
                        (Some(HandType::FourOfAKind), c.clone())
                    } else {
                        hand_types.push(HandType::ThreeOfAKind);
                        (Some(HandType::HighCard), c.clone())
                    }
                }
                (Some(HandType::FourOfAKind), cp) => {
                    if *c == cp {
                        (Some(HandType::FiveOfAKind), c.clone())
                    } else {
                        hand_types.push(HandType::FourOfAKind);
                        (Some(HandType::HighCard), c.clone())
                    }
                }
                _ => acc,
            });
        hand_types.push(last.0.unwrap());

        hand_types.sort_by(|a, b| b.cmp(a));

        hand_types
            .into_iter()
            .reduce(|acc, v| match acc {
                HandType::FiveOfAKind => acc,
                HandType::FourOfAKind => acc,
                HandType::ThreeOfAKind => {
                    if let HandType::OnePair = v {
                        HandType::FullHouse
                    } else {
                        acc
                    }
                }
                HandType::OnePair => {
                    if let HandType::OnePair = v {
                        HandType::TwoPair
                    } else {
                        acc
                    }
                }
                _ => acc,
            })
            .unwrap()
    }
}

#[test]
fn test_hand_type_new() {
    // 32T3K 765
    assert_eq!(
        HandType::OnePair,
        HandType::new(vec![Card::N3, Card::N2, Card::T, Card::N3, Card::K].as_ref())
    );
    // KK677 28
    assert_eq!(
        HandType::TwoPair,
        HandType::new(vec![Card::K, Card::K, Card::N6, Card::N7, Card::N7].as_ref())
    );
    // KTJJT 220
    assert_eq!(
        HandType::TwoPair,
        HandType::new(vec![Card::K, Card::T, Card::J, Card::J, Card::T].as_ref())
    );
    // T55J5 684
    assert_eq!(
        HandType::ThreeOfAKind,
        HandType::new(vec![Card::T, Card::N5, Card::N5, Card::J, Card::N5].as_ref())
    );
    // QQQJA 483
    assert_eq!(
        HandType::ThreeOfAKind,
        HandType::new(vec![Card::Q, Card::Q, Card::Q, Card::J, Card::A].as_ref())
    );
    // 87887 -
    assert_eq!(
        HandType::FullHouse,
        HandType::new(vec![Card::N8, Card::N7, Card::N8, Card::N8, Card::N7].as_ref())
    );
    // 87888 -
    assert_eq!(
        HandType::FourOfAKind,
        HandType::new(vec![Card::N8, Card::N7, Card::N8, Card::N8, Card::N8].as_ref())
    );
    // 77777 -
    assert_eq!(
        HandType::FiveOfAKind,
        HandType::new(vec![Card::N7, Card::N7, Card::N7, Card::N7, Card::N7].as_ref())
    );
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn new(c: char) -> Option<Self> {
        match c {
            'A' => Some(Card::A),
            'K' => Some(Card::K),
            'Q' => Some(Card::Q),
            'J' => Some(Card::J),
            'T' => Some(Card::T),
            '9' => Some(Card::N9),
            '8' => Some(Card::N8),
            '7' => Some(Card::N7),
            '6' => Some(Card::N6),
            '5' => Some(Card::N5),
            '4' => Some(Card::N4),
            '3' => Some(Card::N3),
            '2' => Some(Card::N2),
            _ => None,
        }
    }

    fn parse(i: &str) -> IResult<&str, Self> {
        map(one_of("23456789TJQKA"), |c| Card::new(c).unwrap())(i)
    }
}

fn part1(data: &Data) -> u64 {
    let mut hands = data.0.clone();
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i as u64 + 1) * hand.bet)
        .sum()
}
