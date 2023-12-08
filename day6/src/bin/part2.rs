fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

/**
--- Part Two ---

As the race is about to start, you realize the piece of paper with race times and record distances you got earlier actually just has very bad kerning.
 There's really only one race - ignore the spaces between the numbers on each line.

So, the example from before:

Time:      7  15   30
Distance:  9  40  200

...now instead means this:

Time:      71530
Distance:  940200

Now, you have to figure out how many ways there are to win this single race. In this example, 
the race lasts for 71530 milliseconds and the record distance you need to beat is 940200 millimeters. 
You could hold the button anywhere from 14 to 71516 milliseconds and beat the record, a total of 71503 ways!

How many ways can you beat the record in this one much longer race?

 */
fn get_ranges(time: u64, distance: u64) -> u64 {
    let t = time as f64;
    let d = distance as f64;
    // t = t_1 + t_2
    // v = t_1
    // d = t_2 * v
    let best_hold_time = t / 2.;
    // let best_dist = best_hold_time * best_hold_time;

    let round_up_best_hold = best_hold_time.ceil() as u64;
    let round_down_best_hold = best_hold_time.floor() as u64;

    dbg!(round_up_best_hold);
    dbg!(round_down_best_hold);

    let p1 = t / 2.;
    let p2 = (((t * t) - (4. * d)).sqrt()) / 2.;

    let start = (p1 - p2 + 0.000001).ceil() as u64;
    let end = (p1 + p2 - 0.000001).floor() as u64;

    dbg!(p1 - p2);
    dbg!(p1 + p2);
    dbg!(start);
    dbg!(end);

    let mut sub = 0;
    if round_down_best_hold == round_up_best_hold {
        sub = 1;
    }

    return (round_down_best_hold - start + 1) + (end - round_up_best_hold + 1) - sub;
}

fn get_numbers(line: &str) -> Vec<u64> {
    let mut temp_string = "".to_string();

    let mut numbers = vec![];

    for (i, c) in line.chars().enumerate() {
        if c.is_digit(10) {
            temp_string.push(c);
        }
        if i == line.len() - 1 && !temp_string.is_empty() {
            numbers.push(temp_string.parse::<u64>().unwrap());
            temp_string.clear();
        }
    }

    return numbers;
}

fn part2(input: &str) -> String {
    let lines = input.lines().collect::<Vec<&str>>();
    let times = get_numbers(lines[0]);
    let distances = get_numbers(lines[1]);

    let product = (0..times.len())
        .map(|i| get_ranges(times[i], distances[i]))
        .inspect(|possible| println!("p: {possible}"))
        .fold(1, |acc, x| acc * x);

    return product.to_string();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "Time:      7  15   30
Distance:  9  40  200",
        );
        assert_eq!(result, "71503".to_string());
    }
}
