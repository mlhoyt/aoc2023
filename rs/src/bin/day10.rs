use aoc;
use std::collections::HashMap;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
    // 6850 - too low
}

fn parse(input: &str) -> Result<Data, String> {
    let mut start = Point(0, 0);
    let mut nodes = HashMap::new();

    input.lines().enumerate().for_each(|(ri, l)| {
        l.chars().enumerate().for_each(|(ci, c)| {
            let cp = Point(ci, ri);
            match c {
                '|' => {
                    nodes.insert(
                        cp.clone(),
                        vec![cp.up(), cp.down()]
                            .into_iter()
                            .filter_map(|v| v)
                            .collect(),
                    );
                }
                '-' => {
                    nodes.insert(
                        cp.clone(),
                        vec![cp.left(), cp.right()]
                            .into_iter()
                            .filter_map(|v| v)
                            .collect(),
                    );
                }
                'L' => {
                    nodes.insert(
                        cp.clone(),
                        vec![cp.up(), cp.right()]
                            .into_iter()
                            .filter_map(|v| v)
                            .collect(),
                    );
                }
                'J' => {
                    nodes.insert(
                        cp.clone(),
                        vec![cp.up(), cp.left()]
                            .into_iter()
                            .filter_map(|v| v)
                            .collect(),
                    );
                }
                '7' => {
                    nodes.insert(
                        cp.clone(),
                        vec![cp.left(), cp.down()]
                            .into_iter()
                            .filter_map(|v| v)
                            .collect(),
                    );
                }
                'F' => {
                    nodes.insert(
                        cp.clone(),
                        vec![cp.right(), cp.down()]
                            .into_iter()
                            .filter_map(|v| v)
                            .collect(),
                    );
                }
                'S' => {
                    start = cp.clone();
                }
                _ => (),
            };
        })
    });

    Ok(Data { start, nodes })
}

#[derive(Debug)]
struct Data {
    start: Point,
    nodes: HashMap<Point, Vec<Point>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point(usize, usize);

impl Point {
    fn up(&self) -> Option<Self> {
        if self.1 == 0 {
            None
        } else {
            Some(Point(self.0, self.1 - 1))
        }
    }

    fn right(&self) -> Option<Self> {
        Some(Point(self.0 + 1, self.1))
    }

    fn down(&self) -> Option<Self> {
        Some(Point(self.0, self.1 + 1))
    }

    fn left(&self) -> Option<Self> {
        if self.0 == 0 {
            None
        } else {
            Some(Point(self.0 - 1, self.1))
        }
    }

    fn traverse(&self, previous: &Point, adjacent_points: Option<&Vec<Point>>) -> Option<Point> {
        match adjacent_points {
            None => None,
            Some(aps) => aps
                .iter()
                // Keep adjacent points we did not just visit
                .filter(|ap| **ap != *previous)
                .collect::<Vec<_>>()
                .first() // Should only be one at most
                .map(|v| (**v).clone()),
        }
    }
}

fn part1(data: &Data) -> usize {
    // Find all valid points adjacent to the starting point
    let mut paths = vec![
        data.start.up(),
        data.start.right(),
        data.start.down(),
        data.start.left(),
    ]
    .into_iter()
    .filter_map(|v| v) // drop None, unwrap Some
    .filter(|v| data.nodes.contains_key(&v) && data.nodes.get(&v).unwrap().contains(&data.start))
    .map(|v| (1 as usize, v, data.start.clone()))
    .collect::<Vec<_>>();

    while !paths_joined(&paths) && !paths_crossed(&paths) {
        paths = paths
            .into_iter()
            .filter_map(|(n, cp, pp)| {
                if let Some(np) = cp.traverse(&pp, data.nodes.get(&cp)) {
                    Some((n + 1, np, cp))
                } else {
                    None
                }
            })
            .collect();
    }

    let max_steps = paths.iter().map(|(n, _, _)| *n).max().unwrap();
    if paths_joined(&paths) {
        max_steps
    } else {
        max_steps - 1
    }
}

fn paths_joined(paths: &[(usize, Point, Point)]) -> bool {
    let points = paths.iter().fold(HashMap::new(), |mut acc, (_, p, _)| {
        acc.entry(p).and_modify(|e| *e += 1).or_insert(1);
        acc
    });

    points.iter().any(|(_, n)| *n > 1)
}

fn paths_crossed(paths: &[(usize, Point, Point)]) -> bool {
    cartesian_product(paths)
        .iter()
        .any(|((_, cp1, pp1), (_, cp2, pp2))| *cp1 == *pp2 && *cp2 == *pp1)
}

fn cartesian_product(
    paths: &[(usize, Point, Point)],
) -> Vec<((usize, Point, Point), (usize, Point, Point))> {
    let xs = paths.iter();
    let ys = paths.iter();
    ys.flat_map(|y| xs.clone().map(move |x| (x.clone(), y.clone())))
        .collect()
}
