use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 11: Cosmic Expansion ---

You continue following signs for "Hot Springs" and eventually come across an observatory.
The Elf within turns out to be a researcher studying cosmic expansion using the giant
telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this
research project. However, he confirms that the hot springs are the next-closest area
likely to have people; he'll even take you straight there once he's done with today's
observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single
giant image (your puzzle input). The image includes empty space (.) and galaxies (#).
For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....

The researcher is trying to figure out the sum of the lengths of the shortest path
between every pair of galaxies. However, there's a catch: the universe expanded in
the time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact,
the result is that any rows or columns that contain no galaxies should all actually
be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^

These rows and columns need to be twice as big; the result of cosmic expansion
therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......

Equipped with this expanded universe, the shortest path between every pair of
galaxies can be found. It can help to assign every galaxy a unique number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......

8 + 7 + 6 + 5 + 4 + 3 + 2 + 1
9*4
In these 9 galaxies, there are 36 pairs. Only count each pair once; order
within the pair doesn't matter. For each pair, find any shortest path between
the two galaxies using only steps that move up, down, left,
or right exactly one . or # at a time.
(The shortest path between two galaxies is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......

This path has length 9 because it takes a minimum of nine steps to get from
galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself).
Here are some other example shortest path lengths:

    Between galaxy 1 and galaxy 7: 15
    Between galaxy 3 and galaxy 6: 17
    Between galaxy 8 and galaxy 9: 5

In this example, after expanding the universe, the sum of the shortest path between
all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every pair of galaxies.
What is the sum of these lengths?

*/
#[derive(Clone, Copy, Debug)]
struct Pos {
    _id: u32,
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
struct Pair {
    pos1: Pos,
    pos2: Pos,
    empty_x: usize,
    empty_y: usize,
}

fn get_galaxies(input: &str) -> Vec<Pair> {
    let mut count = 0;
    let mut galaxy_pairs = vec![];
    let mut galaxy_set: Vec<Pos> = vec![];
    let mut has_xs = HashSet::new();
    let mut empty_ys = HashSet::new();
    let mut empty_xs = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let mut is_empty = true;
        for (x, c) in line.char_indices() {
            if c == '#' {
                is_empty = false;
                count += 1;
                let pos = Pos { x, y, _id: count };
                for galaxy in &galaxy_set {
                    galaxy_pairs.push(Pair {
                        pos1: pos,
                        pos2: galaxy.clone(),
                        empty_x: 0,
                        empty_y: 0,
                    });
                }
                galaxy_set.push(pos);
                has_xs.insert(x);
            }
        }
        if is_empty {
            empty_ys.insert(y);
        }
    }

    for x in 0..*has_xs.iter().max().unwrap() {
        if !has_xs.contains(&x) {
            empty_xs.insert(x);
        }
    }

    galaxy_pairs
        .iter()
        .map(|pair| {
            let bigger_x = pair.pos1.x.max(pair.pos2.x);
            let smaller_x = pair.pos1.x.min(pair.pos2.x);
            let empty_x = empty_xs
                .iter()
                .filter(|&x| x < &bigger_x && x > &smaller_x)
                .collect::<Vec<_>>()
                .len();
            let bigger_y = pair.pos1.y.max(pair.pos2.y);
            let smaller_y = pair.pos1.y.min(pair.pos2.y);
            let empty_y = empty_ys
                .iter()
                .filter(|&y| y < &bigger_y && y > &smaller_y)
                .collect::<Vec<_>>()
                .len();
            Pair {
                pos2: pair.pos1,
                pos1: pair.pos2,
                empty_x,
                empty_y,
            }
        })
        .collect()
}

fn get_distance(pair: Pair, expansion: usize) -> usize {
    let p1x = pair.pos1.x as f64;
    let p1y = pair.pos1.y as f64;
    let p2x = pair.pos2.x as f64;
    let p2y = pair.pos2.y as f64;

    let x_expansion = (pair.empty_x * (expansion - 1)) as f64;
    let y_expansion = (pair.empty_y * (expansion - 1)) as f64;

    let x_diff = (p1x - p2x).abs() + x_expansion;
    let y_diff = (p1y - p2y).abs() + y_expansion;

    let h = x_diff + y_diff;

    if pair.pos1._id == 5 && pair.pos2._id == 9 {
        dbg!(pair, h, x_expansion, y_expansion, x_diff, y_diff);
    }

    h as usize
}

fn process(input: &str) -> String {
    let galaxy_pairs = get_galaxies(input);

    let expansion = 2;

    dbg!(&galaxy_pairs.len());

    let result: usize = galaxy_pairs
        .iter()
        .map(|&pair| get_distance(pair, expansion))
        .sum();

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
        );
        assert_eq!(result, "374".to_string());
    }
}
