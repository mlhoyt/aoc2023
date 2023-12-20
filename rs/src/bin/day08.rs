use aoc;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::line_ending;
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot read input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Data, String> {
    let result = all_consuming(map(
        tuple((
            terminated(many1(Direction::parse), tuple((line_ending, line_ending))),
            separated_list1(
                line_ending,
                separated_pair(
                    NodeLabel::parse,
                    tag(" = "),
                    delimited(
                        tag("("),
                        separated_pair(NodeLabel::parse, tag(", "), NodeLabel::parse),
                        tag(")"),
                    ),
                ),
            ),
        )),
        |(directions, nodes)| Data {
            directions,
            nodes: nodes.into_iter().collect(),
        },
    ))(input);

    match result {
        Err(e) => Err(format!("{e:#?}")),
        Ok((_, v)) => Ok(v),
    }
}

#[derive(Debug)]
struct Data {
    directions: Vec<Direction>,
    nodes: std::collections::HashMap<NodeLabel, (NodeLabel, NodeLabel)>,
}

#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl Direction {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(alt((tag("L"), tag("R"))), |c| match c {
            "L" => Self::L,
            _ => Self::R,
        })(i)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct NodeLabel(String);

impl NodeLabel {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(alpha1, |v: &str| Self(v.into()))(i)
    }
}

fn part1(data: &Data) -> usize {
    let mut cs = State::new();
    let mut n = 0;
    while cs.node != NodeLabel("ZZZ".into()) {
        cs = cs.next(data);
        n += 1;
    }

    n
}

#[derive(Debug, Clone)]
struct State {
    node: NodeLabel,
    direction_index: usize,
}

impl State {
    fn new() -> Self {
        State {
            node: NodeLabel("AAA".into()),
            direction_index: 0,
        }
    }

    fn next(&self, data: &Data) -> Self {
        let nodes = data.nodes.get(&self.node);
        let next_node = data
            .directions
            .get(self.direction_index)
            .map(|d| match d {
                Direction::L => nodes.map(|(n, _)| n),
                Direction::R => nodes.map(|(_, n)| n),
            })
            .unwrap()
            .unwrap();
        let next_direction_index = (self.direction_index + 1) % data.directions.len();

        Self {
            node: next_node.to_owned(),
            direction_index: next_direction_index,
        }
    }
}
