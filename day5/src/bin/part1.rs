use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

/**
--- Day 5: If You Give A Seed A Fertilizer ---

You take the boat and find the gardener right where you were told he would be: managing a giant "garden" that looks more to you like a farm.

"A water source? Island Island is the water source!" You point out that Snow Island isn't receiving any water.

"Oh, we had to stop the water because we ran out of sand to filter it with! Can't make snow with dirty water.
Don't worry, I'm sure we'll get more sand soon; we only turned off the water a few days... weeks... oh no." His face sinks into a look of horrified realization.

"I've been so busy making sure everyone here has food that I completely forgot to check why we stopped getting more sand!
There's a ferry leaving soon that is headed over in that direction - it's much faster than your boat. Could you please go check it out?"

You barely have time to agree to this request when he brings up another.
"While you wait for the ferry, maybe you can help us with our food production problem.
The latest Island Island Almanac just arrived and we're having trouble making sense of it."

The almanac (your puzzle input) lists all of the seeds that need to be planted.
It also lists what type of soil to use with each kind of seed, what type of fertilizer to use with each kind of soil,
what type of water to use with each kind of fertilizer, and so on.
Every type of seed, soil, fertilizer and so on is identified with a number,
but numbers are reused by each category - that is, soil 123 and fertilizer 123 aren't necessarily related to each other.

For example:

seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

The almanac starts by listing which seeds need to be planted: seeds 79, 14, 55, and 13.

The rest of the almanac contains a list of maps which describe how to convert numbers from a source category into numbers in a destination category.
 That is, the section that starts with seed-to-soil map: describes how to convert a seed number (the source) to a soil number (the destination).
  This lets the gardener and his team know which soil to use with which seeds, which water to use with which fertilizer, and so on.

Rather than list every source number and its corresponding destination number one by one,
the maps describe entire ranges of numbers that can be converted.
Each line within a map contains three numbers: the destination range start, the source range start, and the range length.

Consider again the example seed-to-soil map:

50 98 2
52 50 48

The first line has a destination range start of 50, a source range start of 98, and a range length of 2.
This line means that the source range starts at 98 and contains two values: 98 and 99.
The destination range is the same length, but it starts at 50, so its two values are 50 and 51.
 With this information, you know that seed number 98 corresponds to soil number 50 and that seed number 99 corresponds to soil number 51.

The second line means that the source range starts at 50 and contains 48 values: 50, 51, ..., 96, 97.
This corresponds to a destination range starting at 52 and also containing 48 values: 52, 53, ..., 98, 99.
So, seed number 53 corresponds to soil number 55.

Any source numbers that aren't mapped correspond to the same destination number. So, seed number 10 corresponds to soil number 10.

So, the entire list of seed numbers and their corresponding soil numbers looks like this:

seed  soil
0     0
1     1
...   ...
48    48
49    49
50    52
51    53
...   ...
96    98
97    99
98    50
99    51

With this map, you can look up the soil number required for each initial seed number:

    Seed number 79 corresponds to soil number 81.
    Seed number 14 corresponds to soil number 14.
    Seed number 55 corresponds to soil number 57.
    Seed number 13 corresponds to soil number 13.

The gardener and his team want to get started as soon as possible,
so they'd like to know the closest location that needs a seed.
Using these maps, find the lowest location number that corresponds
to any of the initial seeds. To do this, you'll need to convert
each seed number through other categories until you can find its
corresponding location number. In this example, the corresponding types are:

    Seed 79, soil 81, fertilizer 81, water 81, light 74, temperature 78, humidity 78, location 82.
    Seed 14, soil 14, fertilizer 53, water 49, light 42, temperature 42, humidity 43, location 43.
    Seed 55, soil 57, fertilizer 57, water 53, light 46, temperature 82, humidity 82, location 86.
    Seed 13, soil 13, fertilizer 52, water 41, light 34, temperature 34, humidity 35, location 35.

So, the lowest location number in this example is 35.

What is the lowest location number that corresponds to any of the initial seed numbers?

 */
struct MappingNumbers {
    dest_start: u32,
    source_start: u32,
    range: u32,
}

struct Mapping {
    dest: String,
    // the destination range start, the source range start, and the range length.
    mappings: Vec<MappingNumbers>,
}

impl Mapping {
    fn get_dest_value(&self, source: u32) -> u32 {
        if let Some(i) = self.in_range(source) {
            let nums = &self.mappings[i];
            dbg!((source, nums.range, nums.dest_start, nums.source_start));
            return nums.dest_start + (source - nums.source_start);
        } else {
            dbg!((source));
            return source;
        }
    }

    fn in_range(&self, source: u32) -> Option<usize> {
        for i in 0..self.mappings.len() {
            if self.mappings[i].in_range(source) {
                return Some(i);
            }
        }

        return None;
    }
}

impl MappingNumbers {
    fn in_range(&self, source: u32) -> bool {
        if source < self.source_start {
            return false;
        }
        if source >= self.source_start + self.range {
            return false;
        }
        return true;
    }
}

type Maps = HashMap<String, Mapping>;

fn get_seeds(seeds_line: &str) -> Vec<u32> {
    let mut seeds = vec![];
    let mut temp_num_str = "";
    let mut temp_str;
    let mut temp_num;
    let nums = &seeds_line[6..];
    for (i, c) in nums.chars().enumerate() {
        if i == nums.len() - 1 || c == ' ' {
            if i == nums.len() - 1 {
                temp_str = format!("{temp_num_str}{c}");
                temp_num_str = temp_str.as_str();
            }
            if !temp_num_str.is_empty() {
                temp_num = temp_num_str.parse::<u32>().unwrap();
                temp_num_str = "";
                seeds.push(temp_num);
            }
        } else {
            temp_str = format!("{temp_num_str}{c}");
            temp_num_str = temp_str.as_str();
        }
    }

    dbg!(&seeds);

    return seeds;
}

fn get_source_dest(line: &str) -> (String, String) {
    let mut source = String::new();
    let mut dest = String::new();
    let mut temp_string = "".to_string();
    let string = &line[..line.len() - 5];
    dbg!(string);
    for (i, c) in string.chars().enumerate() {
        temp_string.push(c);
        if c == '-' && !temp_string.is_empty() {
            if source.is_empty() {
                source = temp_string[..temp_string.len() - 1].to_string();
            }
            temp_string.clear();
        } else if i == string.len() - 1 && !temp_string.is_empty() {
            dest = temp_string.clone();
        }
    }

    dbg!((source.clone(), dest.clone()));

    (source, dest)
}

fn get_mapping_nums(line: &str) -> MappingNumbers {
    let mut temp_string = "".to_string();

    let mut dest_start = 0;
    let mut source_start = 0;
    let mut range = 0;

    let mut set_dest_start = false;

    dbg!(line);
    for (i, c) in line.chars().enumerate() {
        temp_string.push(c);
        if c == ' ' && !temp_string.is_empty() {
            if set_dest_start {
                source_start = temp_string[..temp_string.len() - 1].parse::<u32>().unwrap();
            } else {
                set_dest_start = true;
                dest_start = temp_string[..temp_string.len() - 1].parse::<u32>().unwrap();
            }
            temp_string.clear();
        } else if i == line.len() - 1 && !temp_string.is_empty() {
            range = temp_string.parse::<u32>().unwrap();
        }
    }

    MappingNumbers {
        dest_start,
        source_start,
        range,
    }
}

fn get_maps(maps_lines: &[&str]) -> Maps {
    let mut maps = HashMap::new();

    let mut source = String::new();
    let mut map = Mapping {
        dest: String::new(),
        mappings: vec![],
    };
    for i in 0..maps_lines.len() {
        let line = maps_lines[i];
        dbg!(line);

        if map.dest.is_empty() {
            let (new_source, new_dest) = get_source_dest(line);
            map.dest = new_dest.clone();
            source = new_source.clone();
        } else if line.is_empty() {
            maps.insert(source, map);
            source = String::new();
            map = Mapping {
                dest: String::new(),
                mappings: vec![],
            };
        } else {
            map.mappings.push(get_mapping_nums(line));
        }
    }
    maps.insert(source, map);

    maps
}

fn traverse_map(source: String, maps: &Maps, source_value: u32) -> u32 {
    let mut destination_value = u32::MAX;
    let mut prev_value = source_value;
    let mut new_source = source;
    while let Some(map) = maps.get(&new_source) {
        dbg!((new_source, prev_value));
        new_source = map.dest.clone();
        destination_value = map.get_dest_value(prev_value);
        prev_value = destination_value;
    }

    return destination_value;
}

fn part1(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let seeds = get_seeds(&lines[0]);
    let maps = get_maps(&lines[2..]);

    let mut result = u32::MAX;
    let mut temp_result: u32;
    for seed in seeds {
        temp_result = traverse_map("seed".to_string(), &maps, seed);
        dbg!(temp_result);
        if temp_result < result {
            result = temp_result;
        }
    }

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4",
        );
        assert_eq!(result, "0 - 35".to_string());
    }
}
