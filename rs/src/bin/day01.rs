use aoc;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");

    let data = parse(&input);
    // println!("data: {data:#?}");

    let result1: u32 = data.iter().sum();
    println!("result: {result1}");
}

fn parse(input: &str) -> Vec<u32> {
    let number_words: Vec<(String, String)> = vec![
        ("one".into(), "1".into()),
        ("two".into(), "2".into()),
        ("three".into(), "3".into()),
        ("four".into(), "4".into()),
        ("five".into(), "5".into()),
        ("six".into(), "6".into()),
        ("seven".into(), "7".into()),
        ("eight".into(), "8".into()),
        ("nine".into(), "9".into()),
    ];

    enum NextPos {
        First,
        Last,
    }

    let next_number = |s: &str, pos: NextPos| -> u32 {
        let mut vs: Vec<_> = number_words
            .iter()
            .flat_map(|(fs, ts)| {
                s.match_indices(fs)
                    .map(|(n, _)| (n, fs.len(), ts.to_owned()))
            })
            .collect();

        vs.sort_by_key(|v| v.0);

        let mut s = s.to_string();
        match pos {
            NextPos::First => {
                if let Some((n, l, ts)) = vs.first() {
                    s.replace_range(*n..(*n + *l), &ts);
                }
            }
            NextPos::Last => {
                if let Some((n, l, ts)) = vs.last() {
                    s.replace_range(*n..(*n + *l), &ts);
                }
            }
        };

        let ns: Vec<_> = s.chars().filter_map(|c| c.to_digit(10)).collect();

        match pos {
            NextPos::First => ns.first().map(|v| v.clone()).unwrap_or(0),
            NextPos::Last => ns.last().map(|v| v.clone()).unwrap_or(0),
        }
    };

    input
        .lines()
        .map(|l| {
            let nl = next_number(l, NextPos::First);
            let nr = next_number(l, NextPos::Last);
            (nl * 10) + nr
        })
        .collect()
}
