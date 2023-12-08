use regex::Regex;

fn main() {
    let input = include_str!("./input2.txt");
    let output = part2(input);
    dbg!(output);
}

/**
The Elf says they've stopped producing snow because they aren't getting any water!
He isn't sure why the water stopped; however, he can show you how to get to the water source to check it out for yourself.
It's just up ahead!

As you continue your walk, the Elf poses a second question: in each game you played, what is the fewest number of
cubes of each color that could have been in the bag to make the game possible?

Again consider the example games from earlier:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

    In game 1, the game could have been played with as few as 4 red, 2 green, and 6 blue cubes.
    If any color had even one fewer cube, the game would have been impossible.
    Game 2 could have been played with a minimum of 1 red, 3 green, and 4 blue cubes.
    Game 3 must have been played with at least 20 red, 13 green, and 6 blue cubes.
    Game 4 required at least 14 red, 3 green, and 15 blue cubes.
    Game 5 needed no fewer than 6 red, 3 green, and 2 blue cubes in the bag.

The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively.
 Adding up these five powers produces the sum 2286.

For each game, find the minimum set of cubes that must have been present. What is the sum of the power of these sets?
 */
struct Games {
    id: i32,
    games: Vec<Game>,
}
struct Game {
    red: i32,
    green: i32,
    blue: i32,
}

fn get_game(line: &str) -> Game {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    let color_re = Regex::new(r"(\d*) (red|green|blue)").unwrap();
    for (_, [count_str, color]) in color_re.captures_iter(line).map(|c| c.extract()) {
        let count = count_str.parse::<i32>().unwrap();
        match color {
            "red" => red = count,
            "green" => green = count,
            "blue" => blue = count,
            &_ => continue,
        };
    }

    Game { red, green, blue }
}

fn process_string(line: &str) -> Option<Games> {
    let game_re = Regex::new(r"Game (?<id>\d*): (?<games>.*)$").unwrap();
    let Some(caps) = game_re.captures(line) else {
        println!("no match! {line}");
        return None;
    };

    let id = &caps["id"].parse::<i32>().unwrap();

    let str_games = &caps["games"];

    let mut games = vec![];
    for game in str_games.split(';') {
        dbg!(game);
        games.push(get_game(game));
    }

    Some(Games { id: *id, games })
}

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        return x;
    } else {
        return y;
    };
}

fn get_game_id(line: &str) -> Option<i32> {
    let game = match process_string(line) {
        Some(game) => game,
        None => return None,
    };

    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;

    for gg in game.games {
        max_blue = max(max_blue, gg.blue);
        max_green = max(max_green, gg.green);
        max_red = max(max_red, gg.red);
    }

    // The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
    let mut power_set = 1;
    if max_blue > 0 {
        power_set *= max_blue;
    }
    if max_red > 0 {
        power_set *= max_red;
    }
    if max_green > 0 {
        power_set *= max_green;
    }
    // The power of the minimum set of cubes in game 1 is 48. In games 2-5 it was 12, 1560, 630, and 36, respectively.
    //  Adding up these five powers produces the sum 2286.
    
    // For each game, find the minimum set of cubes that must have been present.

    return Some(power_set);
}

fn part2(input: &str) -> String {
    input
        .lines()
        .inspect(|line| {
            dbg!(line);
        })
        .filter_map(get_game_id)
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "2286");
    }
}
