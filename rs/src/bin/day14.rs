use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::IResult;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");

    // let result2 = part2(&data);
    // println!("part2: {result2}");
}

fn parse(input: &str) -> Result<Data, String> {
    let result = all_consuming(Data::parse)(input);

    match result {
        Err(e) => Err(format!("{e}")),
        Ok((_, v)) => Ok(v),
    }
}

#[derive(Debug, Clone)]
struct Data(Vec<Row>);

impl Data {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(line_ending, Row::parse), Self)(i)
    }

    fn transpose(&self) -> Self {
        let height = self.0.len();
        let width = if height > 0 {
            self.0.first().unwrap().0.len()
        } else {
            0
        };

        (0..width)
            .map(|ci| {
                (0..height)
                    .map(|ri| self.0.get(ri).unwrap().0.get(ci).unwrap().clone())
                    .collect::<Vec<_>>()
                    .into() // Row
            })
            .collect::<Vec<_>>()
            .into() // Self
    }

    fn mirror_y(&self) -> Self {
        (self.0)
            .iter()
            .map(|v| (v.0).iter().rev().cloned().collect::<Vec<_>>().into())
            .collect::<Vec<_>>()
            .into() // Self
    }

    fn roll_left(&self) -> Self {
        (self.0)
            .iter()
            .map(|v| {
                (v.0)
                    .split(|v| matches!(v, RockType::Cube))
                    .map(|v| {
                        // count RockType::Round
                        let n_round = v.iter().filter(|v| matches!(v, RockType::Round)).count();

                        // produce new slice of (round * round_count)
                        std::iter::repeat(RockType::Round)
                            .take(n_round)
                            // produce new slice of (empty * (len - round_count))
                            .chain(std::iter::repeat(RockType::Empty).take(v.len() - n_round))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
                    .join(&RockType::Cube)
                    .into() // Row
            })
            .collect::<Vec<_>>()
            .into() // Self
    }

    fn tilt(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => self.transpose().roll_left().transpose(),
            Direction::East => self.mirror_y().roll_left().mirror_y(),
            Direction::South => self
                .transpose()
                .mirror_y()
                .roll_left()
                .mirror_y()
                .transpose(),
            Direction::West => self.roll_left(),
        }
    }

    fn north_beam_load(&self) -> usize {
        let n_rows = (self.0).len();
        (self.0)
            .iter()
            .enumerate()
            .map(|(ri, r)| {
                let multiplier = n_rows - ri;
                let n_round = (r.0)
                    .iter()
                    .filter(|v| matches!(v, RockType::Round))
                    .count();

                n_round * multiplier
            })
            .sum()
    }

    fn spin(&self) -> Self {
        self.tilt(Direction::North)
            .tilt(Direction::West)
            .tilt(Direction::South)
            .tilt(Direction::East)
    }
}

impl std::convert::From<Vec<Row>> for Data {
    fn from(item: Vec<Row>) -> Self {
        Self(item)
    }
}

impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

#[derive(Debug, Clone)]
struct Row(Vec<RockType>);

impl Row {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(many1(RockType::parse), Self)(i)
    }
}

impl std::convert::From<Vec<RockType>> for Row {
    fn from(item: Vec<RockType>) -> Self {
        Self(item)
    }
}

impl std::fmt::Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("")
        )
    }
}

#[derive(Debug, Clone)]
enum RockType {
    Cube,
    Round,
    Empty,
}

impl RockType {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(alt((tag("#"), tag("O"), tag("."))), |c| match c {
            "#" => Self::Cube,
            "O" => Self::Round,
            _ => Self::Empty,
        })(i)
    }
}

impl std::fmt::Display for RockType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cube => write!(f, "#"),
            Self::Round => write!(f, "O"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn part1(data: &Data) -> usize {
    data.tilt(Direction::North).north_beam_load()
}

fn part2(data: &Data) -> usize {
    let mut current = data.clone();
    // for _ in 0..1_000_000_000 {
    //     current = current.spin();
    // }

    current.north_beam_load()
}
