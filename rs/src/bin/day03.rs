use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Data, String> {
    Ok(Data(
        input
            .lines()
            .enumerate()
            .map(|(ri, l)| {
                let mut es = vec![];

                let le = l.chars().enumerate().fold(None, |acc, (i, c)| {
                    if c.is_digit(10) {
                        match acc {
                            Some(Element::Number {
                                row,
                                first_index,
                                value,
                                ..
                            }) => Some(Element::Number {
                                row,
                                first_index,
                                last_index: i,
                                value: (value * 10) + c.to_digit(10).unwrap_or(0),
                            }),
                            _ => Some(Element::Number {
                                row: ri,
                                first_index: i,
                                last_index: i,
                                value: c.to_digit(10).unwrap_or(0),
                            }),
                        }
                    } else if c == '.' {
                        if acc.is_some() {
                            es.push(acc.unwrap());
                        }

                        None
                    } else {
                        if acc.is_some() {
                            es.push(acc.unwrap());
                        }

                        es.push(Element::Symbol { index: i, value: c });

                        None
                    }
                });

                if le.is_some() {
                    es.push(le.unwrap());
                }

                es
            })
            .collect::<Vec<_>>(),
    ))
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Data(Vec<Vec<Element>>);

#[derive(Debug, Hash, Eq, PartialEq)]
enum Element {
    Number {
        row: usize,
        first_index: usize,
        last_index: usize,
        value: u32,
    },
    Symbol {
        index: usize,
        value: char,
    },
}

fn part1(data: &Data) -> u32 {
    data.0
        .iter()
        .enumerate()
        .flat_map(|(i, es)| {
            es.iter()
                // Find all Element::Symbol in the current row
                .filter(|e| match e {
                    Element::Symbol { .. } => true,
                    _ => false,
                })
                // Find all Symbol::Number adjacent to Element::Symbol
                .flat_map(|se| {
                    // We know "se" is an Element::Symbol but the compiler doesn't so we need to
                    // pattern match to make the compiler happy and to extract its "index" (i.e.
                    // column position).
                    if let Element::Symbol { index: s_index, .. } = se {
                        data.0
                            .iter()
                            .enumerate()
                            // From the adjacent and current rows
                            .filter(|(n, _)| *n >= i - 1 && *n <= i + 1)
                            // Find all Element::Number items
                            .flat_map(|(_, es)| {
                                es.iter()
                                    .filter(|e| {
                                        if let Element::Number { .. } = e {
                                            true
                                        } else {
                                            false
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            })
                            // Keep adjacent Symbol::Number items
                            .filter(|ne| {
                                if let Element::Number {
                                    first_index: n_first_index,
                                    last_index: n_last_index,
                                    ..
                                } = ne
                                {
                                    (*n_first_index <= *s_index && *n_last_index >= *s_index)
                                        || (*n_first_index == *s_index + 1)
                                        || (*n_last_index + 1 == *s_index)
                                } else {
                                    false
                                }
                            })
                            .collect::<Vec<_>>()
                    } else {
                        vec![]
                    }
                })
                .map(|e| e)
                .collect::<Vec<_>>()
        })
        // We want a unique set of Element::Number
        .collect::<std::collections::HashSet<_>>()
        .iter()
        .map(|e| match e {
            Element::Number { value, .. } => *value,
            _ => 0,
        })
        .sum()
}
