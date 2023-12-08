use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

/**
--- Part Two ---

Everyone will starve if you only plant such a small number of seeds.
 Re-reading the almanac, it looks like the seeds: line actually describes ranges of seed numbers.

The values on the initial seeds: line come in pairs. Within each pair,
the first value is the start of the range and the second value is the length of the range.
So, in the first line of the example above:

seeds: 79 14 55 13

This line describes two ranges of seed numbers to be planted in the garden.
The first range starts with seed number 79 and contains 14 values: 79, 80, ..., 91, 92.
The second range starts with seed number 55 and contains 13 values: 55, 56, ..., 66, 67.

Now, rather than considering four seed numbers, you need to consider a total of 27 seed numbers.

In the above example, the lowest location number can be obtained from seed number 82, which
 corresponds to soil 84, fertilizer 84, water 84, light 77, temperature 45, humidity 46, and
 location 46. So, the lowest location number is 46.

Consider all of the initial seed numbers listed in the ranges on the first line of the almanac.
What is the lowest location number that corresponds to any of the initial seed numbers?

 */
struct MappingNumbers {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

struct Mapping {
    dest: String,
    // the destination range start, the source range start, and the range length.
    mappings: Vec<MappingNumbers>,
}

impl Mapping {
    fn get_dest_value_range(&self, ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut dest_ranges: Vec<(u64, u64)> = vec![];
        let mut cut_ranges: Vec<(u64, u64)> = vec![];
        let mut source_check: u64;
        let mut range_check: u64;

        let mut ranges_check = ranges;

        while !ranges_check.is_empty() {
            for (source, range) in &ranges_check {
                source_check = *source;
                range_check = *range;
                for i in 0..self.mappings.len() {
                    if self.mappings[i].in_range(source_check, range_check) {
                        let (
                            (dest_start, dest_range),
                            (cut_start, cut_range),
                            (end_start, end_range),
                        ) = self.mappings[i].get_dest_range(source_check, range_check);
                        dest_ranges.push((dest_start, dest_range));
                        source_check = cut_start;
                        range_check = cut_range;
                        if end_range > 0 {
                            cut_ranges.push((end_start, end_range));
                        }
                    }
                    if range_check == 0 {
                        break;
                    }
                }
                if range_check > 0 {
                    dest_ranges.push((source_check, range_check));
                }
            }
            ranges_check = cut_ranges.clone();
            cut_ranges.clear();
        }

        dest_ranges
    }
}

impl MappingNumbers {
    fn in_range(&self, source: u64, range: u64) -> bool {
        if source + range < self.source_start {
            return false;
        }
        if source >= self.source_start + self.range {
            return false;
        }

        true
    }

    fn get_dest_value(&self, source: u64) -> u64 {
        self.dest_start + (source - self.source_start)
    }

    fn get_dest_range(&self, source: u64, range: u64) -> ((u64, u64), (u64, u64), (u64, u64)) {
        let mut cut_start = 0;
        let mut cut_range = 0;
        let mut end_start = 0;
        let mut end_range = 0;
        let mut dest_start = self.dest_start;
        let mut dest_end = self.dest_start + self.range;
        let mut set_start = false;
        let mut set_end = false;
        if source >= self.source_start && source <= self.source_start + self.range {
            dest_start = self.get_dest_value(source);
            set_start = true;
        }
        if source + range >= self.source_start && source + range <= self.source_start + self.range {
            dest_end = self.get_dest_value(source + range);
            set_end = true;
        }
        let new_range = dest_end - dest_start;
        println!("{{s{},d{},r{}}}", self.source_start, self.dest_start, self.range);
        println!("\t{{{}-{},{}}}", source, source + range - 1, range);

        if new_range < range {
            if set_start {
                cut_start = source + new_range;
                cut_range = range - new_range;
            } else if set_end {
                cut_start = source;
                cut_range = range - new_range;
            } else {
                cut_start = source;
                cut_range = self.source_start - source - 1;
                end_start = source + new_range;
                end_range = range - cut_range - new_range;
                println!("\t\t{{cs{}-ce{}, {}}}", cut_start, cut_start + cut_range - 1, cut_range);
                println!("\t\t{{es{}-ee{}, {}}}", end_start, end_start + end_range - 1, end_range);
            }
        }

        (
            (dest_start, new_range),
            (cut_start, cut_range),
            (end_start, end_range),
        )
    }
}

type Maps = HashMap<String, Mapping>;

fn get_seeds(seeds_line: &str) -> Vec<(u64, u64)> {
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
                temp_num = temp_num_str.parse::<u64>().unwrap();
                temp_num_str = "";
                seeds.push(temp_num);
            }
        } else {
            temp_str = format!("{temp_num_str}{c}");
            temp_num_str = temp_str.as_str();
        }
    }

    dbg!(&seeds);

    let mut seed_pairs: Vec<(u64, u64)> = vec![];
    for i in 0..(seeds.len() / 2) {
        let pos = i * 2;
        seed_pairs.push((seeds[pos], seeds[pos + 1]));
    }

    return seed_pairs;
}

fn get_source_dest(line: &str) -> (String, String) {
    let mut source = String::new();
    let mut dest = String::new();
    let mut temp_string = "".to_string();
    let string = &line[..line.len() - 5];
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

    (source, dest)
}

fn get_mapping_nums(line: &str) -> MappingNumbers {
    let mut temp_string = "".to_string();

    let mut dest_start = 0;
    let mut source_start = 0;
    let mut range = 0;

    let mut set_dest_start = false;

    for (i, c) in line.chars().enumerate() {
        temp_string.push(c);
        if c == ' ' && !temp_string.is_empty() {
            if set_dest_start {
                source_start = temp_string[..temp_string.len() - 1].parse::<u64>().unwrap();
            } else {
                set_dest_start = true;
                dest_start = temp_string[..temp_string.len() - 1].parse::<u64>().unwrap();
            }
            temp_string.clear();
        } else if i == line.len() - 1 && !temp_string.is_empty() {
            range = temp_string.parse::<u64>().unwrap();
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

fn traverse_map(source: String, maps: &Maps, source_value: u64, range: u64) -> u64 {
    println!("");
    println!("{source}: {source_value}-{} ({range})", source_value + range - 1);
    let mut destination_value = vec![(source_value, range)];
    let mut new_source = source;
    while let Some(map) = maps.get(&new_source) {
        println!("{new_source}-{}", map.dest);
        destination_value = map.get_dest_value_range(destination_value);
        println!("");
        let mut c = 0;
        for d in &destination_value {
            print!("{}-{} ({}), ", d.0, d.0 + d.1 - 1, d.1);
            c += d.1;
        }
        if c != range {
            panic!("Should equal");
        }
        print!("\n");
        new_source = map.dest.clone();
    }

    return destination_value
        .iter()
        .fold(u64::MAX, |acc, (x, _)| if x < &acc { *x } else { acc });
}

fn part2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let seeds = get_seeds(&lines[0]);
    let maps = get_maps(&lines[2..]);

    let mut result = u64::MAX;
    let mut temp_result: u64;
    for (start, range) in seeds {
        temp_result = traverse_map("seed".to_string(), &maps, start, range);
        if temp_result < result {
            result = temp_result;
        }
    }

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(result, "46".to_string());
    }
}
