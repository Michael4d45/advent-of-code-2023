use regex::Regex;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

/**
You're launched high into the atmosphere! The apex of your trajectory just
barely reaches the surface of a large island floating in the sky. You gently
 land in a fluffy pile of leaves. It's quite cold, but you don't see much snow.
 An Elf runs over to greet you.

The Elf explains that you've arrived at Snow Island and apologizes for the lack of snow.
He'll be happy to explain the situation, but it's a bit of a walk, so you have some time.
They don't get many visitors up here; would you like to play a game in the meantime?

As you walk, the Elf shows you a small bag and some cubes which are either red, green, or blue.
 Each time you play this game, he will hide a secret number of cubes of each color in the bag,
  and your goal is to figure out information about the number of cubes.

To get information, once a bag has been loaded with cubes, the Elf will reach into the bag,
grab a handful of random cubes, show them to you, and then put them back in the bag. He'll do this a few times per game.

You play several games and record the information from each game (your puzzle input).
Each game is listed with its ID number (like the 11 in Game 11: ...) followed by a semicolon-separated
 list of subsets of cubes that were revealed from the bag (like 3 red, 5 green, 4 blue).

For example, the record of a few games might look like this:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

In game 1, three sets of cubes are revealed from the bag (and then put back again).
The first set is 3 blue cubes and 4 red cubes; the second set is 1 red cube, 2 green cubes, and 6 blue cubes;
 the third set is only 2 green cubes.

The Elf would first like to know which games would have been possible if the bag contained only
 12 red cubes, 13 green cubes, and 14 blue cubes?

In the example above, games 1, 2, and 5 would have been possible if the bag had been loaded with
 that configuration. However, game 3 would have been impossible because at one point the Elf showed
  you 20 red cubes at once; similarly, game 4 would also have been impossible because the Elf showed
   you 15 blue cubes at once. If you add up the IDs of the games that would have been possible, you get 8.

Determine which games would have been possible if the bag had been loaded with only 12 red cubes,
 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
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

    dbg!(red);
    dbg!(green);
    dbg!(blue);

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

    dbg!(str_games);

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
    let possible_game = Game {
        red: 12,
        green: 13,
        blue: 14,
    };

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

    if max_blue > possible_game.blue
        || max_green > possible_game.green
        || max_red > possible_game.red
    {
        return None;
    }

    return Some(game.id);
}

fn part1(input: &str) -> String {
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
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, "8");
    }
}
