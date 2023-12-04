use aoc;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::space1,
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot read input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Data, String> {
    Ok(Data(
        input
            .lines()
            .map(|l| match all_consuming(Game::parse)(l) {
                Err(e) => Err(format!("{e}")),
                Ok((_, v)) => Ok(v),
            })
            .collect::<Result<Vec<_>, _>>()?,
    ))
}

#[derive(Debug)]
struct Data(Vec<Game>);

#[derive(Debug)]
struct Game(u32, Vec<Round>);

impl Game {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                preceded(tag("Game "), nom::character::complete::u32),
                tag(": "),
                separated_list1(tag("; "), Round::parse),
            ),
            |(index, rounds)| Self(index, rounds),
        )(i)
    }
}

#[derive(Debug)]
struct Round(std::collections::HashMap<CubeColor, u32>);

impl Round {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                tag(", "),
                separated_pair(nom::character::complete::u32, space1, CubeColor::parse),
            ),
            |vs| {
                Self(
                    vs.into_iter()
                        .map(|(n, c)| (c, n))
                        .collect::<std::collections::HashMap<_, _>>(),
                )
            },
        )(i)
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum CubeColor {
    Red,
    Green,
    Blue,
    Other,
}

impl CubeColor {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(alt((tag("red"), tag("green"), tag("blue"))), |v| match v {
            "red" => Self::Red,
            "green" => Self::Green,
            "blue" => Self::Blue,
            _ => Self::Other,
        })(i)
    }
}

fn part1(data: &Data) -> u32 {
    data.0
        .iter()
        .filter_map(|Game(index, rounds)| {
            if rounds.iter().all(|round| {
                round.0.iter().all(|(c, n)| match c {
                    CubeColor::Red => *n <= 12,
                    CubeColor::Green => *n <= 13,
                    CubeColor::Blue => *n <= 14,
                    _ => true,
                })
            }) {
                Some(*index)
            } else {
                None
            }
        })
        .sum()
}
