use std::time::{Instant, Duration};

use regex::Regex;

fn main() {
    // Read input from file
    let input = include_str!("./input1.txt");

    // Define the number of times you want to run the benchmark
    let num_runs = 10;

    // Initialize a variable to store the total execution time
    let mut total_time = Duration::new(0, 0);

    // Run the benchmark multiple times
    for _ in 0..num_runs {
        let start_time = Instant::now();
        let output = part2(input);
        let elapsed_time = start_time.elapsed();
        
        // Accumulate the total execution time
        total_time += elapsed_time;

        // Print the result for each run if needed
        println!("Output: {:?}", output);
        println!("Execution time: {:?}", elapsed_time);
    }

    // Calculate and print the average execution time
    let average_time = total_time / num_runs as u32;
    println!("Average execution time: {:?}", average_time);
}

/**
 * Your calculation isn't quite right. It looks like some of the digits are actually spelled out with letters:
 * one, two, three, four, five, six, seven, eight, and nine also count as valid "digits".
 *
 * Equipped with this new information, you now need to find the real first and last digit on each line. For example:
 *
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen

 * In this example, the calibration values are 29, 83, 13, 24, 42, 14, and 76. Adding these together produces 281.
*/
fn part2(input: &str) -> String {
    let str_numbers = str_get_with_digits(input);

    // println!("numbers: {}", str_numbers.len());

    let mut sum = 0;
    for num_string in str_numbers {
        sum += num_string;
    }

    return sum.to_string();
}

fn str_digit_to_digit(s: &str) -> Option<usize> {
    let numbers = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for i in 0..numbers.len() {
        if s == numbers[i] {
            return Some(i + 1);
        }
    }

    return None;
}

fn translate_to_digits(s: &str) -> u32 {
    if s.len() == 0 {
        return 0;
    }
    // println!("s: {s}");

    let mut first = 0;
    let mut set_first = false;
    let mut last = 0;

    for i in 0..s.len() {
        let digit: u32 = match s.chars().nth(i) {
            Some(num) => num.to_digit(10).unwrap_or(0),
            None => 0,
        };

        if digit < 10 && digit > 0 {
            last = digit;
            if !set_first {
                first = digit;
                set_first = true;
            }
            continue;
        }

        for j in i + 1..s.len() + 1 {
            // let start = &s[0..i];
            let sub_str = &s[i..j];
            // let end = &s[j..s.len()];
            // println!("sub_str:{} {sub_str} {}", start, end);
            let digit = match str_digit_to_digit(sub_str) {
                Some(digit) => digit as u32,
                None => continue,
            };

            if digit > 0 {
                last = digit;
                if !set_first {
                    first = digit;
                    set_first = true;
                }
            }
        }
    }

    // println!("first, last:{first} {last}");
    return first * 10 + last;
}

fn str_get_with_digits(s: &str) -> Vec<u32> {
    let re = Regex::new(r"\s*(.*)\s*").unwrap();
    // iterate over all matches
    let mut results = vec![];
    for (_, [digits_in_string]) in re.captures_iter(s).map(|c| c.extract()) {
        let final_string = translate_to_digits(digits_in_string);
        // println!("final_string: {final_string}");
        results.push(final_string);
    }

    return results;
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen",
        );
        assert_eq!(result, "281");
    }
}
