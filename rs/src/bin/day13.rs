use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
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
    let result = all_consuming(Data::parse)(input);

    match result {
        Err(e) => Err(format!("{e}")),
        Ok((_, v)) => Ok(v),
    }
}

#[derive(Debug)]
struct Data(Vec<Pattern>);

impl Data {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_list1(tuple((line_ending, line_ending)), Pattern::parse),
            Self,
        )(i)
    }
}

#[derive(Debug)]
struct Pattern(Vec<Row>);

impl Pattern {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(line_ending, Row::parse), Self)(i)
    }

    fn reflection_point(&self) -> Option<usize> {
        self.0
            .windows(2)
            .enumerate()
            .filter(|(_, w)| w[0] == w[1])
            .filter(|(i, _)| {
                (1..=usize::min(*i, self.0.len() - (i + 2)))
                    .all(|o| self.0[i - o] == self.0[i + 1 + o])
            })
            .map(|(i, _)| i)
            .collect::<Vec<_>>()
            .first()
            .copied()
    }

    fn transpose(&self) -> Self {
        let height = self.0.len();
        let width = if height > 0 {
            self.0.first().unwrap().0.len()
        } else {
            0
        };

        Self(
            (0..width)
                .map(|ci| {
                    Row((0..height)
                        .map(|ri| self.0.get(ri).unwrap().0.get(ci).unwrap().clone())
                        .collect::<Vec<_>>())
                })
                .collect::<Vec<_>>(),
        )
    }
}

#[derive(Debug, PartialEq)]
struct Row(Vec<Symbol>);

impl Row {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(many1(Symbol::parse), Self)(i)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Symbol {
    Ash,
    Rocks,
}

impl Symbol {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(alt((tag("."), tag("#"))), |c| match c {
            "." => Self::Ash,
            _ => Self::Rocks,
        })(i)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ash => write!(f, "."),
            Self::Rocks => write!(f, "#"),
        }
    }
}

fn part1(data: &Data) -> usize {
    data.0
        .iter()
        .map(|p| {
            if let Some(h) = p.reflection_point() {
                return 100 * (h + 1);
            }

            if let Some(v) = p.transpose().reflection_point() {
                return v + 1;
            }

            0
        })
        .sum()
}
