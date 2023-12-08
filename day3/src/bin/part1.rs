use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

/**
 * --- Day 3: Gear Ratios ---

You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand,
 but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

4 6 7 . . 1 1 4 . .
. . . * . . . . . .
. . 3 5 . . 6 3 3 .
. . . . . . # . . .
6 1 7 * . . . . . .
. . . . . + . 5 8 .
. . 5 9 2 . . . . .
. . . . . . 7 5 5 .
. . . $ . * . . . .
. 6 6 4 . 5 9 8 . .

In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

 */
fn part1(input: &str) -> String {
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

    let mut result = 0;

    for ((x, y), value) in &numbers {
        let len = value.to_string().len() as i32;
        let top_check = y - 1;
        let bottom_check = y + 1;
        let mut has_symbol = false;
        let symbol = symbols.get(&(x - 1, *y));
        if symbol != None {
            has_symbol = true;
        }
        for i in 0..len + 2 {
            let x_check = x + i - 1;
            let symbol = symbols.get(&(x_check, top_check));
            if symbol != None {
                has_symbol = true;
                break;
            }
            let symbol = symbols.get(&(x_check, bottom_check));
            if symbol != None {
                has_symbol = true;
                break;
            }
        }
        let symbol = symbols.get(&(x + len, *y));
        if symbol != None {
            has_symbol = true;
        }

        if has_symbol {
            result += value;
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
        assert_eq!(result, "4361".to_string());
    }
}
