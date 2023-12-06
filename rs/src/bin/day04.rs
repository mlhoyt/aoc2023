use aoc;
use nom::{
    bytes::complete::tag,
    character::complete::space1,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};
use std::collections::HashSet;

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
            .map(|l| match all_consuming(Card::parse)(l) {
                Err(e) => Err(format!("{e}")),
                Ok((_, v)) => Ok(v),
            })
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

#[derive(Debug)]
struct Data(Vec<Card>);

#[derive(Debug)]
struct Card {
    index: u32,
    winning_numbers: HashSet<u32>,
    chosen_numbers: HashSet<u32>,
}

impl Card {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tuple((tag("Card"), space1)), nom::character::complete::u32),
                tuple((tag(":"), space1)),
                separated_pair(
                    separated_list1(space1, nom::character::complete::u32),
                    tuple((space1, tag("|"), space1)),
                    separated_list1(space1, nom::character::complete::u32),
                ),
            ),
            |(index, (winning_numbers, chosen_numbers))| Self {
                index,
                winning_numbers: winning_numbers.into_iter().collect::<HashSet<_>>(),
                chosen_numbers: chosen_numbers.into_iter().collect::<HashSet<_>>(),
            },
        )(i)
    }
}

fn part1(data: &Data) -> u32 {
    data.0
        .iter()
        .map(|c| {
            c.winning_numbers
                .iter()
                .filter(|n| c.chosen_numbers.contains(n))
                .count()
        })
        .map(|n| {
            if n > 0 {
                u32::pow(2, (n as u32) - 1)
            } else {
                0
            }
        })
        .sum()
}
