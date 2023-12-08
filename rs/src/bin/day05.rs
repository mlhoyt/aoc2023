use aoc;
use nom::branch::permutation;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::{space0, space1};
use nom::combinator::all_consuming;
use nom::combinator::map;
use nom::multi::many0;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

fn main() {
    let input = aoc::read_stdin().expect("cannot read stdin");
    // println!("{input}");

    let data = parse(&input).expect("cannot parse input");
    // println!("{data:#?}");

    let result1 = part1(&data);
    println!("part1: {result1}");
}

fn parse(input: &str) -> Result<Data, String> {
    let result = all_consuming(map(
        permutation((
            seeds_parser,
            map_parser("seed-to-soil"),
            map_parser("soil-to-fertilizer"),
            map_parser("fertilizer-to-water"),
            map_parser("water-to-light"),
            map_parser("light-to-temperature"),
            map_parser("temperature-to-humidity"),
            map_parser("humidity-to-location"),
        )),
        |(v1, v2, v3, v4, v5, v6, v7, v8)| Data {
            seeds: v1,
            seed_to_soil_map: v2,
            soil_to_fertilizer_map: v3,
            fertilizer_to_water_map: v4,
            water_to_light_map: v5,
            light_to_temperature_map: v6,
            temperature_to_humidity_map: v7,
            humidity_to_location_map: v8,
        },
    ))(input);

    match result {
        Err(e) => Err(format!("{e}")),
        Ok((_, v)) => Ok(v),
    }
}

fn seeds_parser(i: &str) -> IResult<&str, Vec<u64>> {
    map(
        tuple((
            tag("seeds:"),
            space1,
            separated_list1(space1, nom::character::complete::u64),
            many0(line_ending),
        )),
        |(_, _, ns, _)| ns,
    )(i)
}

fn map_parser<'a>(label: &'a str) -> impl FnMut(&'a str) -> IResult<&'a str, Vec<(u64, u64, u64)>> {
    map(
        tuple((
            tuple((tag(label), space1, tag("map:"), space0, line_ending)),
            separated_list1(
                line_ending,
                tuple((
                    nom::character::complete::u64,
                    space1,
                    nom::character::complete::u64,
                    space1,
                    nom::character::complete::u64,
                )),
            ),
            many0(line_ending),
        )),
        |(_, vs, _)| {
            let mut vs = vs
                .into_iter()
                .map(|(dst_start, _, src_start, _, range)| (src_start, dst_start, range))
                .collect::<Vec<_>>();
            vs.sort_by_key(|v| v.0);
            vs
        },
    )
}

#[derive(Debug)]
struct Data {
    seeds: Vec<u64>,
    seed_to_soil_map: Vec<(u64, u64, u64)>,
    soil_to_fertilizer_map: Vec<(u64, u64, u64)>,
    fertilizer_to_water_map: Vec<(u64, u64, u64)>,
    water_to_light_map: Vec<(u64, u64, u64)>,
    light_to_temperature_map: Vec<(u64, u64, u64)>,
    temperature_to_humidity_map: Vec<(u64, u64, u64)>,
    humidity_to_location_map: Vec<(u64, u64, u64)>,
}

impl Data {
    fn seed_to_location(&self, seed: u64) -> u64 {
        let v1 = translate(&self.seed_to_soil_map, seed);
        let v2 = translate(&self.soil_to_fertilizer_map, v1);
        let v3 = translate(&self.fertilizer_to_water_map, v2);
        let v4 = translate(&self.water_to_light_map, v3);
        let v5 = translate(&self.light_to_temperature_map, v4);
        let v6 = translate(&self.temperature_to_humidity_map, v5);
        translate(&self.humidity_to_location_map, v6)
    }
}

fn translate(map: &[(u64, u64, u64)], i: u64) -> u64 {
    let v = map
        .iter()
        .fold(None, |acc, (src_start, dst_start, range)| match acc {
            Some(v) => Some(v),
            None => {
                if i >= *src_start && i < (*src_start + *range) {
                    Some(*dst_start + (i - *src_start))
                } else {
                    None
                }
            }
        });

    match v {
        Some(v) => v,
        None => i,
    }
}

fn part1(data: &Data) -> u64 {
    data.seeds
        .iter()
        .map(|seed| data.seed_to_location(*seed))
        .min()
        .unwrap_or(0)
}
