use std::collections::{HashMap, HashSet};

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
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pos {
    id: u32,
    x: usize,
    y: usize,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Pair {
    pos1: Pos,
    pos2: Pos,
    empty_x: u32,
    empty_y: u32,
}

fn get_galaxies(input: &str) -> HashSet<Pair> {
    let mut count = 0;
    let mut galaxy_pairs = HashSet::new();
    let mut galaxy_set: HashSet<Pos> = HashSet::new();
    let mut has_xs = HashSet::new();
    let mut empty_ys = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        let mut is_empty = true;
        for (x, c) in line.char_indices() {
            if c == '#' {
                is_empty = false;
                count += 1;
                let pos = Pos { x, y, id: count };
                for galaxy in &galaxy_set {
                    galaxy_pairs.insert(Pair {
                        pos1: pos,
                        pos2: galaxy.clone(),
                        empty_x: 0,
                        empty_y: 0,
                    });
                }
                galaxy_set.insert(pos);
                has_xs.insert(x);
            }
        }
        if is_empty {
            empty_ys.insert(y);
        }
    }

    for x in 0..*has_xs.iter().max().unwrap() {
        dbg!(x);
    }

    galaxy_pairs
}

fn process(input: &str) -> String {
    let galaxies = get_galaxies(input);

    return 0.to_string();
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
