fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");

    let result2 = part2(&data);
    println!("part2: {result2}");
}

fn parse(input: &str) -> Result<Data, String> {
    Ok(Data(
        input
            .lines()
            .enumerate()
            .flat_map(move |(yi, l)| {
                l.chars().enumerate().filter_map(move |(xi, c)| match c {
                    '#' => Some(Point(xi, yi)),
                    _ => None,
                })
            })
            .collect::<Vec<_>>(),
    ))
}

#[derive(Debug, Clone)]
struct Data(Vec<Point>);

impl Data {
    fn expand(&self, factor: usize) -> Self {
        self.expand_horizontal(factor).expand_vertical(factor)
    }

    fn expand_horizontal(&self, factor: usize) -> Self {
        let max_x = self.0.iter().map(|Point(x, _)| *x).max().unwrap();

        // find empty columns (largest to smallest)
        let empty: Vec<_> = (0..max_x)
            .rev()
            .filter(|xi| !self.0.iter().any(|Point(x, _)| *x == *xi))
            .collect();

        // fold over empty columns shifting right each entry located right of the current column
        empty.into_iter().fold(self.clone(), |acc, xi| {
            Self(
                acc.0
                    .iter()
                    .map(|Point(x, y)| {
                        if *x > xi {
                            Point(*x + (factor - 1), *y)
                        } else {
                            Point(*x, *y)
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
    }

    fn expand_vertical(&self, factor: usize) -> Self {
        let max_y = self.0.iter().map(|Point(_, y)| *y).max().unwrap();

        // find empty rows (largest to smallest)
        let empty: Vec<_> = (0..max_y)
            .rev()
            .filter(|yi| !self.0.iter().any(|Point(_, y)| *y == *yi))
            .collect();

        // fold over empty rows shifting down each entry located below the current row
        empty.into_iter().fold(self.clone(), |acc, yi| {
            Self(
                acc.0
                    .iter()
                    .map(|Point(x, y)| {
                        if *y > yi {
                            Point(*x, *y + (factor - 1))
                        } else {
                            Point(*x, *y)
                        }
                    })
                    .collect::<Vec<_>>(),
            )
        })
    }
}

#[derive(Debug, Clone)]
struct Point(usize, usize);

impl Point {
    fn distance(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

fn part1(data: &Data) -> usize {
    let data = data.expand(2);

    // find all pairs of points
    let pairs = (0..(data.0.len() - 1))
        .flat_map(|i1| ((i1 + 1)..data.0.len()).map(move |i2| (i1, i2)))
        .map(|(i1, i2)| {
            (
                data.0.get(i1).unwrap().clone(),
                data.0.get(i2).unwrap().clone(),
            )
        })
        .collect::<Vec<_>>();

    pairs.into_iter().map(|(p1, p2)| p1.distance(&p2)).sum()
}

fn part2(data: &Data) -> usize {
    let data = data.expand(1_000_000);

    // find all pairs of points
    let pairs = (0..(data.0.len() - 1))
        .flat_map(|i1| ((i1 + 1)..data.0.len()).map(move |i2| (i1, i2)))
        .map(|(i1, i2)| {
            (
                data.0.get(i1).unwrap().clone(),
                data.0.get(i2).unwrap().clone(),
            )
        })
        .collect::<Vec<_>>();

    pairs.into_iter().map(|(p1, p2)| p1.distance(&p2)).sum()
}
