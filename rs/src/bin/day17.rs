use aoc::grid2d::Grid2D;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
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
}

fn parse(input: &str) -> Result<Data, String> {
    let result = all_consuming(Data::parse)(input);

    match result {
        Err(e) => Err(format!("{e}")),
        Ok((_, v)) => Ok(v),
    }
}

#[derive(Debug)]
struct Data(Grid2D<u8>);

impl Data {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(separated_list1(line_ending, Row::parse), |v| {
            Self(Grid2D::new(&(v.iter().map(|v| v.0.clone()).collect::<Vec<_>>())).unwrap())
        })(i)
    }
}

#[derive(Debug)]
struct Row(Vec<u8>);

impl Row {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(many1(digit_parser), Self)(i)
    }
}

fn digit_parser(i: &str) -> IResult<&str, u8> {
    map(one_of("0123456789"), |v| (v as u8) - (b'0'))(i)
}

#[derive(Debug)]
struct Path {
    point: Point,
    direction: Direction,
    heat_loss_counter: usize,
    straight_counter: usize,
    course: Vec<Direction>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn next(&self, dir: &Direction) -> Option<Self> {
        match dir {
            Direction::North => {
                if self.y > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y - 1,
                    })
                } else {
                    None
                }
            }
            Direction::East => Some(Self {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::South => Some(Self {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::West => {
                if self.x > 0 {
                    Some(Self {
                        x: self.x,
                        y: self.y,
                    })
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Lsr {
    Left,
    Straight,
    Right,
}

// Part 1
//
// One of the main "tricks" is that spawning needs to be prevented whereever possible to keep
// the number of iterable items as small as possible.  There are some obvious ways to avoid
// spawing:
//   1. The current cell is the finish cell.
//   2. The cell to spawn to is not in the grid.
//   3. The cell to spawn to is straight ahead but the current Path straight_counter already equals 3.
//   4. The cell to spawn to has already been visited by a Path with a lower heat_loss_counter.
fn part1(data: &Data) -> usize {
    let endpoint = Point {
        x: data.0.get_width() - 1,
        y: data.0.get_height() - 1,
    };

    let mut paths = std::collections::VecDeque::from([Path {
        point: Point { x: 0, y: 0 },
        direction: Direction::East,
        heat_loss_counter: 0,
        straight_counter: 1,
        course: vec![Direction::East],
    }]);

    let mut visited = std::collections::HashMap::<(Point, Direction), usize>::new();

    let find_next_path = |visited: &std::collections::HashMap<(Point, Direction), usize>,
                          path: &Path,
                          lsr: Lsr|
     -> Option<Path> {
        let next_dir = match lsr {
            Lsr::Left => path.direction.left(),
            Lsr::Straight => path.direction.clone(),
            Lsr::Right => path.direction.right(),
        };

        if let Some(next_point) = path.point.next(&next_dir) {
            if let Some(next_point_heat_loss) = data.0.get_yx(next_point.y, next_point.x) {
                let next_heat_loss = path.heat_loss_counter + (next_point_heat_loss as usize);
                let mut next_course = path.course.clone();
                next_course.push(next_dir.clone());

                let next_path = Path {
                    point: next_point.to_owned(),
                    direction: next_dir.to_owned(),
                    heat_loss_counter: next_heat_loss,
                    straight_counter: if lsr == Lsr::Straight {
                        path.straight_counter + 1
                    } else {
                        1
                    },
                    course: next_course,
                };

                match visited.get(&(next_point.clone(), next_dir.clone())) {
                    Some(v) => {
                        if *v <= next_heat_loss {
                            // Already visited by more efficient path
                            None
                        } else {
                            Some(next_path)
                        }
                    }
                    None => Some(next_path),
                }
            } else {
                // Off-grid: too high
                None
            }
        } else {
            // Off-grid: too low
            None
        }
    };

    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();

        if let Some(v) = visited.get(&(path.point.clone(), path.direction.clone())) {
            if *v < path.heat_loss_counter {
                continue;
            }
        };

        visited
            .entry((path.point.clone(), path.direction.clone()))
            .and_modify(|v| *v = std::cmp::min(*v, path.heat_loss_counter))
            .or_insert(path.heat_loss_counter);

        if path.point == endpoint {
            println!("part1: successful path:\n{path:#?}");
            continue;
        }

        if let Some(next_path) = find_next_path(&visited, &path, Lsr::Left) {
            paths.push_back(next_path);
        }

        if path.straight_counter < 3 {
            if let Some(next_path) = find_next_path(&visited, &path, Lsr::Straight) {
                paths.push_back(next_path);
            }
        }

        if let Some(next_path) = find_next_path(&visited, &path, Lsr::Right) {
            paths.push_back(next_path);
        }
    }

    let shortest_path = [
        (endpoint.clone(), Direction::East),
        (endpoint.clone(), Direction::South),
    ]
    .iter()
    .filter_map(|v| visited.get(v))
    .min();

    if let Some(v) = shortest_path {
        *v
    } else {
        0
    }
}
