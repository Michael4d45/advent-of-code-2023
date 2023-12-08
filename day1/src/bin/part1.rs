use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

/**
 * Something is wrong with global snow production, and you've been selected to take a look.
 * The Elves have even given you a map; on it, they've used stars to mark the top fifty locations that are likely to be having problems.
 *
 * You've been doing this long enough to know that to restore snow operations, you need to check all fifty stars by December 25th.
 *
 * Collect stars by solving puzzles.
 *  Two puzzles will be made available on each day in the Advent calendar;
 *  the second puzzle is unlocked when you complete the first.
 * Each puzzle grants one star. Good luck!
 *
 * You try to ask why they can't just use a weather machine ("not powerful enough")
 *  and where they're even sending you ("the sky") and why your map looks mostly blank
 *  ("you sure ask a lot of questions") and hang on did you just say the sky ("of course, where do you think snow comes from")
 *  when you realize that the Elves are already loading you into a trebuchet ("please hold still, we need to strap you in").
 *
 * As they're making the final adjustments, they discover that their calibration document (your puzzle input) has been amended by
 *  a very young Elf who was apparently just excited to show off her art skills. Consequently,
 *  the Elves are having trouble reading the values on the document.
 *
 * The newly-improved calibration document consists of lines of text;
 *  each line originally contained a specific calibration value that the
 *  Elves now need to recover. On each line, the calibration value can be
 *  found by combining the first digit and the last digit (in that order)
 *  to form a single two-digit number.
 */
fn part1(input: &str) -> String {
    let str_numbers = str_get_with_digits(input);

    println!("numbers: {}", str_numbers.len());

    let mut sum = 0;
    for num_string in str_numbers {
        let mut first = 0;
        let mut set_first = false;
        let mut last = 0;
        for check in num_string.chars() {
            let digit = match check.to_digit(10) {
                Some(num) => num,
                None => continue,
            };

            if digit < 10 {
                last = digit;
                if !set_first {
                    first = digit;
                    set_first = true;
                }
            }
        }

        sum += first * 10 + last;
    }

    return sum.to_string();
}

fn str_get_with_digits(s: &str) -> Vec<&str> {
    let re = Regex::new(r"[^\d]*(.*)[^\d]*").unwrap();
    // iterate over all matches
    let mut results = vec![];
    for (_, [digits_in_string]) in re.captures_iter(s).map(|c| c.extract()) {
        results.push(digits_in_string);
    }

    return results;
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet",
        );
        assert_eq!(result, "142");
    }
}
