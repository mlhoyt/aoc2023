fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Vec<String>, String> {
    Ok(input.split(',').map(|v| v.to_string()).collect::<Vec<_>>())
}

fn part1(data: &[String]) -> u32 {
    data.iter()
        .map(|v| {
            v.chars()
                .map(|v| v as u8)
                .fold(0, |a, v| ((a + v as u32) * 17) % 256)
        })
        .sum()
}
