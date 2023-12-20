use aoc;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::separated_list1;
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
struct Data(Vec<Sequence>);

impl Data {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(line_ending, Sequence::parse), |v| Self(v))(i)
    }
}

#[derive(Debug)]
struct Sequence(Vec<i32>);

impl Sequence {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_list1(space1, nom::character::complete::i32),
            |v| Self(v),
        )(i)
    }

    fn next(&self) -> i32 {
        if self.0.iter().all(|v| *v == 0) {
            0
        } else {
            self.0.last().unwrap() + self.delta().next()
        }
    }

    fn delta(&self) -> Self {
        Self(self.0.windows(2).map(|w| w[1] - w[0]).collect())
    }
}

fn part1(data: &Data) -> i32 {
    data.0.iter().map(|s| s.next()).sum()
}
