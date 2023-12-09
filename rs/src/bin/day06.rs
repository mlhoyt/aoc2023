use aoc;
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Data, String> {
    let result = all_consuming(map(
        permutation((time_parser, distance_parser)),
        |(times, distances)| {
            Data(
                times
                    .into_iter()
                    .zip(distances.into_iter())
                    .collect::<Vec<_>>(),
            )
        },
    ))(input);

    match result {
        Err(e) => Err(format!("{e}")),
        Ok((_, v)) => Ok(v),
    }
}

fn time_parser(i: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((
            preceded(
                tuple((tag("Time:"), space1)),
                separated_list1(space1, nom::character::complete::u64),
            ),
            many0(line_ending),
        )),
        |(v, _)| v,
    )(i)
}

fn distance_parser(i: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((
            preceded(
                tuple((tag("Distance:"), space1)),
                separated_list1(space1, nom::character::complete::u64),
            ),
            many0(line_ending),
        )),
        |(v, _)| v,
    )(i)
}

#[derive(Debug)]
struct Data(Vec<(u64, u64)>);

fn part1(data: &Data) -> u64 {
    data.0
        .iter()
        .map(|(t, d)| {
            (0..=*t)
                .into_iter()
                .filter(|th| *th * (*t - *th) > *d)
                .count() as u64
        })
        .product()
}
