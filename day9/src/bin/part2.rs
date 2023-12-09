fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Part Two ---

Of course, it would be nice to have even more history included in your report.
Surely it's safe to just extrapolate backwards as well, right?

For each history, repeat the process of finding differences until the sequence of
differences is entirely zero. Then, rather than adding a zero to the end and filling
in the next values of each previous sequence, you should instead add a zero to the
beginning of your sequence of zeroes, then fill in new first values for each previous sequence.

In particular, here is what the third example history looks like when extrapolating back in time:

5  10  13  16  21  30  45
  5   3   3   5   9  15
   -2   0   2   4   6
      2   2   2   2
        0   0   0

Adding the new values on the left side of each sequence from bottom to top eventually
reveals the new left-most history value: 5.

Doing this for the remaining example data above results in previous values of -3 for the
first history and 0 for the second history. Adding all three new values together produces 2.

Analyze your OASIS report again, this time extrapolating the previous value for each history.
 What is the sum of these extrapolated values?

*/
fn get_next_number(numbers: Vec<i32>) -> i32 {
    let mut first_nums = vec![];

    let mut temp_nums_processing = numbers.clone();
    let mut temp_nums_storing = vec![];
    let mut all_zero = false;
    let mut storing;
    while !all_zero {
        all_zero = true;
        first_nums.push(*temp_nums_processing.first().unwrap());

        for i in 0..temp_nums_processing.len() - 1 {
            storing = temp_nums_processing[i + 1] - temp_nums_processing[i];
            if storing != 0 {
                all_zero = false;
            }
            temp_nums_storing.push(storing);
        }
        temp_nums_processing = temp_nums_storing.clone();
        temp_nums_storing.clear();
    }

    return first_nums.iter().rev().fold(0, |acc, x| x - acc);
}

fn get_numbers(line: &str) -> Vec<i32> {
    line.split(" ")
        .map(|num| num.parse::<i32>().unwrap())
        .inspect(|x| {
            dbg!(x);
        })
        .collect::<Vec<i32>>()
}

fn process(input: &str) -> String {
    let result: i32 = input
        .lines()
        .map(get_numbers)
        .map(get_next_number)
        .inspect(|next_num| {
            dbg!(next_num);
        })
        .sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45",
        );
        assert_eq!(result, "2".to_string());
    }
}
