use aoc::grid2d::Grid2D;
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
    // println!("{data:?}");

    let result1 = part1(&data);
    println!("part1: {result1}");

    let result2 = part2(&data);
    println!("part2: {result2}");
}

fn parse(input: &str) -> Result<Data, String> {
    let result = all_consuming(Data::parse)(input);

    match result {
        Err(e) => Err(format!("{e}")),
        Ok((_, v)) => Ok(v),
    }
}

#[derive(Debug, Clone)]
struct Data(Grid2D<Tile>);

impl Data {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(line_ending, Row::parse), |v| {
            Self(Grid2D::new(&(v.iter().map(|v| v.0.clone()).collect::<Vec<_>>())).unwrap())
        })(i)
    }
}

#[derive(Debug, Clone)]
struct Row(Vec<Tile>);

impl Row {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(many1(Tile::parse), Self)(i)
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Space,
    AngleLeft,
    AngleRight,
    Vertical,
    Horizontal,
}

impl Tile {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            alt((tag("/"), tag("\\"), tag("|"), tag("-"), tag("."))),
            |c| match c {
                "/" => Self::AngleRight,
                "\\" => Self::AngleLeft,
                "|" => Self::Vertical,
                "-" => Self::Horizontal,
                _ => Self::Space,
            },
        )(i)
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AngleRight => write!(f, "/"),
            Self::AngleLeft => write!(f, "\\"),
            Self::Vertical => write!(f, "|"),
            Self::Horizontal => write!(f, "-"),
            Self::Space => write!(f, "."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next(&self, dir: Direction) -> Option<Self> {
        match dir {
            Direction::North => {
                if self.y > 0 {
                    Some(Self::new(self.x, self.y - 1))
                } else {
                    None
                }
            }
            Direction::East => Some(Self::new(self.x + 1, self.y)),
            Direction::South => Some(Self::new(self.x, self.y + 1)),
            Direction::West => {
                if self.x > 0 {
                    Some(Self::new(self.x - 1, self.y))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    point: Point,
    dir: Direction,
}

impl Beam {
    fn new(point: Point, dir: Direction) -> Self {
        Self { point, dir }
    }

    fn next(&self, tile: &Tile) -> Vec<Self> {
        match (&self.dir, tile) {
            (Direction::North, Tile::AngleLeft) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::West) {
                    next.push(Self::new(point, Direction::West));
                }
                next
            }
            (Direction::North, Tile::AngleRight) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::East) {
                    next.push(Self::new(point, Direction::East));
                }
                next
            }
            (Direction::North, Tile::Vertical) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::North) {
                    next.push(Self::new(point, Direction::North));
                }
                next
            }
            (Direction::North, Tile::Horizontal) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::West) {
                    next.push(Self::new(point, Direction::West));
                }
                if let Some(point) = self.point.next(Direction::East) {
                    next.push(Self::new(point, Direction::East));
                }
                next
            }
            (Direction::North, Tile::Space) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::North) {
                    next.push(Self::new(point, Direction::North));
                }
                next
            }
            (Direction::East, Tile::AngleLeft) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::South) {
                    next.push(Self::new(point, Direction::South));
                }
                next
            }
            (Direction::East, Tile::AngleRight) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::North) {
                    next.push(Self::new(point, Direction::North));
                }
                next
            }
            (Direction::East, Tile::Vertical) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::North) {
                    next.push(Self::new(point, Direction::North));
                }
                if let Some(point) = self.point.next(Direction::South) {
                    next.push(Self::new(point, Direction::South));
                }
                next
            }
            (Direction::East, Tile::Horizontal) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::East) {
                    next.push(Self::new(point, Direction::East));
                }
                next
            }
            (Direction::East, Tile::Space) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::East) {
                    next.push(Self::new(point, Direction::East));
                }
                next
            }
            (Direction::South, Tile::AngleLeft) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::East) {
                    next.push(Self::new(point, Direction::East));
                }
                next
            }
            (Direction::South, Tile::AngleRight) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::West) {
                    next.push(Self::new(point, Direction::West));
                }
                next
            }
            (Direction::South, Tile::Vertical) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::South) {
                    next.push(Self::new(point, Direction::South));
                }
                next
            }
            (Direction::South, Tile::Horizontal) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::West) {
                    next.push(Self::new(point, Direction::West));
                }
                if let Some(point) = self.point.next(Direction::East) {
                    next.push(Self::new(point, Direction::East));
                }
                next
            }
            (Direction::South, Tile::Space) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::South) {
                    next.push(Self::new(point, Direction::South));
                }
                next
            }
            (Direction::West, Tile::AngleLeft) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::North) {
                    next.push(Self::new(point, Direction::North));
                }
                next
            }
            (Direction::West, Tile::AngleRight) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::South) {
                    next.push(Self::new(point, Direction::South));
                }
                next
            }
            (Direction::West, Tile::Vertical) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::North) {
                    next.push(Self::new(point, Direction::North));
                }
                if let Some(point) = self.point.next(Direction::South) {
                    next.push(Self::new(point, Direction::South));
                }
                next
            }
            (Direction::West, Tile::Horizontal) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::West) {
                    next.push(Self::new(point, Direction::West));
                }
                next
            }
            (Direction::West, Tile::Space) => {
                let mut next = vec![];
                if let Some(point) = self.point.next(Direction::West) {
                    next.push(Self::new(point, Direction::West));
                }
                next
            }
        }
    }
}

fn simulate(data: &Data, starting_beam: &Beam) -> usize {
    let mut beams = std::collections::VecDeque::from([starting_beam.clone()]);
    let mut beam_history = std::collections::HashSet::<Beam>::new();

    while !beams.is_empty() {
        // println!("beams: {beams:?}");
        let beam = beams.pop_front().unwrap();
        beam_history.insert(beam.clone());

        let tile = data.0.get_yx(beam.point.y, beam.point.x).unwrap();
        beam.next(&tile)
            .into_iter()
            .filter(|b| data.0.get_yx(b.point.y, b.point.x).is_some())
            .filter(|b| !beam_history.contains(b))
            .for_each(|b| beams.push_back(b));
    }

    beam_history
        .into_iter()
        .map(|b| b.point)
        .collect::<std::collections::HashSet<Point>>()
        .len()
}

fn part1(data: &Data) -> usize {
    let starting_beam = Beam::new(Point::new(0, 0), Direction::East);
    simulate(data, &starting_beam)
}

fn part2(data: &Data) -> usize {
    let max_x = data.0.get_width() - 1;
    let max_y = data.0.get_height() - 1;

    let starting_beams = (0..=max_x)
        .map(|x| Beam::new(Point::new(x, 0), Direction::South))
        .chain((0..=max_y).map(|y| Beam::new(Point::new(max_x, y), Direction::West)))
        .chain((0..=max_x).map(|x| Beam::new(Point::new(x, max_y), Direction::North)))
        .chain((0..=max_y).map(|y| Beam::new(Point::new(0, y), Direction::East)))
        .collect::<Vec<_>>();

    let result = starting_beams
        .iter()
        .map(|sb| (sb, simulate(data, sb)))
        .max_by(|a, b| a.1.cmp(&(b.1)))
        .unwrap();
    // println!("part2: result: {result:?}");

    result.1
}
