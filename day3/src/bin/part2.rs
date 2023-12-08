use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

/**
--- Part Two ---

The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other.
 You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..

In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right;
 its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?

 */
fn part2(input: &str) -> String {
    let mut x: i32;
    let mut y: i32 = 0;
    let mut numbers = HashMap::new();
    let mut symbols = HashMap::new();
    let mut is_number: bool;
    let mut num: &str;
    let mut temp_num: String;
    for line in input.lines() {
        dbg!(line);
        is_number = false;
        num = "";
        x = 0;
        for c in line.chars() {
            if c != '.' {
                if c.is_digit(10) {
                    is_number = true;
                    temp_num = format!("{}{}", &num, &c);
                    num = temp_num.as_str();
                } else if is_number {
                    is_number = false;
                    numbers.insert((x - num.len() as i32, y), num.parse::<i32>().unwrap());
                    num = "";
                    symbols.insert((x, y), c);
                } else {
                    symbols.insert((x, y), c);
                }
            } else if is_number {
                is_number = false;
                numbers.insert((x - num.len() as i32, y), num.parse::<i32>().unwrap());
                num = "";
            }
            x += 1;
        }
        if is_number {
            numbers.insert((x - num.len() as i32, y), num.parse::<i32>().unwrap());
        }
        y += 1;
    }

    dbg!(numbers.len());
    dbg!(symbols.len());

    let mut gears: HashMap<(i32, i32), (i32, i32)> = HashMap::new();

    for ((x, y), value) in &numbers {
        let len = value.to_string().len() as i32;
        let top_check = y - 1;
        let bottom_check = y + 1;
        let pos = (x - 1, *y);
        let symbol = symbols.get(&pos);
        if symbol.unwrap_or(&'.') == &'*' {
            let cur_gear = gears.get(&pos);
            if cur_gear == None {
                gears.insert(pos, (1, *value));
            } else {
                let gear = cur_gear.unwrap();
                gears.insert(pos, (gear.0 + 1, gear.1 * value));
            }
        }
        for i in 0..len + 2 {
            let x_check = x + i - 1;
            let pos = (x_check, top_check);
            let symbol = symbols.get(&pos);
            if symbol.unwrap_or(&'.') == &'*' {
                let cur_gear = gears.get(&pos);
                if cur_gear == None {
                    gears.insert(pos, (1, *value));
                } else {
                    let gear = cur_gear.unwrap();
                    gears.insert(pos, (gear.0 + 1, gear.1 * value));
                }
            }
            let pos = (x_check, bottom_check);
            let symbol = symbols.get(&pos);
            if symbol.unwrap_or(&'.') == &'*' {
                let cur_gear = gears.get(&pos);
                if cur_gear == None {
                    gears.insert(pos, (1, *value));
                } else {
                    let gear = cur_gear.unwrap();
                    gears.insert(pos, (gear.0 + 1, gear.1 * value));
                }
            }
        }
        let pos = (x + len, *y);
        let symbol = symbols.get(&pos);
        if symbol.unwrap_or(&'.') == &'*' {
            let cur_gear = gears.get(&pos);
            if cur_gear == None {
                gears.insert(pos, (1, *value));
            } else {
                let gear = cur_gear.unwrap();
                gears.insert(pos, (gear.0 + 1, gear.1 * value));
            }
        }
    }

    fn filter_gears((_, (count, value)): (&(i32, i32), &(i32, i32))) -> Option<i32> {
        if count >= &2 {
            return Some(*value);
        }

        None
    }

    let result: i32 = gears.iter().filter_map(filter_gears).sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, "467835".to_string());
    }
}
