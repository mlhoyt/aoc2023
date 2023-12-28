use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space1;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use regex::Regex;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Data, String> {
    let result = all_consuming(separated_list1(line_ending, Row::parse))(input);

    match result {
        Err(e) => Err(format!("{e}")),
        Ok((_, v)) => Ok(Data(v)),
    }
}

#[derive(Debug)]
struct Data(Vec<Row>);

#[derive(Debug)]
struct Row {
    status: Vec<SpringStatus>,
    damaged_sequence: Vec<u32>,
}

impl Row {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                many1(SpringStatus::parse),
                space1,
                separated_list1(tag(","), nom::character::complete::u32),
            ),
            |(status, damaged_sequence)| Row {
                status,
                damaged_sequence,
            },
        )(i)
    }

    fn to_regex(&self) -> Regex {
        let mut re: String = r"^\.*".to_owned();
        re.push_str(
            self.damaged_sequence
                .iter()
                .map(|n| format!("#{{{}}}", *n))
                .collect::<Vec<_>>()
                .join(r"\.+")
                .as_str(),
        );
        re.push_str(r"\.*$");

        Regex::new(re.as_str()).unwrap()
    }

    fn to_permutations(&self) -> Vec<String> {
        let nr_unknowns: u32 = self
            .status
            .iter()
            .map(|s| match s {
                SpringStatus::Unknown => 1,
                _ => 0,
            })
            .sum();
        let nr_damaged: u32 = self
            .status
            .iter()
            .map(|s| match s {
                SpringStatus::Damaged => 1,
                _ => 0,
            })
            .sum();
        let total_damaged: u32 = self.damaged_sequence.iter().sum();

        let base: u32 = 2;
        (0..=(base.pow(nr_unknowns) - 1))
            .into_iter()
            // to binary string of length nr_unknowns
            .map(|n| format!("{:0width$b}", n, width = nr_unknowns as usize))
            // to Vec<SpringStatus>: 0 -> Damaged, 1 -> Operational
            .map(|s| {
                s.chars()
                    .map(|c| match c {
                        '0' => SpringStatus::Damaged,
                        _ => SpringStatus::Operational,
                    })
                    .collect::<Vec<_>>()
            })
            // filter by correct damaged spring count
            .filter(|vs| {
                let nr_damaged_replacements: u32 = vs
                    .iter()
                    .map(|s| match s {
                        SpringStatus::Damaged => 1,
                        _ => 0,
                    })
                    .sum();

                nr_damaged + nr_damaged_replacements == total_damaged
            })
            // replace Unknown in self.status with values from Vec from binary string
            .map(|rvs| {
                let mut n = 0;
                self.status
                    .iter()
                    .map(|cv| match cv {
                        SpringStatus::Unknown => {
                            let nv = rvs.get(n).unwrap();
                            n += 1;
                            nv.clone()
                        }
                        _ => cv.clone(),
                    })
                    .collect::<Vec<_>>()
            })
            // to String
            .map(|vs| {
                vs.into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join("")
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

impl SpringStatus {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(alt((tag("."), tag("#"), tag("?"))), |v| match v {
            "." => Self::Operational,
            "#" => Self::Damaged,
            _ => Self::Unknown,
        })(i)
    }
}

impl ToString for SpringStatus {
    fn to_string(&self) -> String {
        match self {
            SpringStatus::Operational => ".".into(),
            SpringStatus::Damaged => "#".into(),
            SpringStatus::Unknown => "?".into(),
        }
    }
}

fn part1(data: &Data) -> u32 {
    data.0
        .iter()
        .map(|row| {
            let re = row.to_regex();

            row.to_permutations()
                .into_iter()
                .filter(|status| re.is_match(status.as_str()))
                .count()
        })
        .map(|v| v as u32)
        .sum()
}
