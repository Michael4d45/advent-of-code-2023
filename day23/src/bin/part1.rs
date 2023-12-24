use std::collections::{BTreeMap, BTreeSet, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 23: A Long Walk ---

The Elves resume water filtering operations!
Clean water starts flowing over the edge of Island Island.

They offer to help you go over the edge of Island Island, too!
Just hold on tight to one end of this impossibly long rope and
they'll lower you down a safe distance from the massive
waterfall you just created.

As you finally reach Snow Island, you see that the water isn't
really reaching the ground: it's being absorbed by the air itself.
It looks like you'll finally have a little downtime while the
moisture builds up to snow-producing levels. Snow Island is pretty
scenic, even without any snow; why not take a walk?

There's a map of nearby hiking trails (your puzzle input) that
indicates paths (.), forest (#), and steep slopes (^, >, v, and <).

For example:

#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#

You're currently on the single path tile in the top row; your
goal is to reach the single path tile in the bottom row. Because
of all the mist from the waterfall, the slopes are probably
quite icy; if you step onto a slope tile, your next step must
be downhill (in the direction the arrow is pointing). To make
sure you have the most scenic hike possible, never step onto
the same tile twice. What is the longest hike you can take?

In the example above, the longest hike you can take is
marked with O, and your starting position is marked S:

#S#####################
#OOOOOOO#########...###
#######O#########.#.###
###OOOOO#OOO>.###.#.###
###O#####O#O#.###.#.###
###OOOOO#O#O#.....#...#
###v###O#O#O#########.#
###...#O#O#OOOOOOO#...#
#####.#O#O#######O#.###
#.....#O#O#OOOOOOO#...#
#.#####O#O#O#########v#
#.#...#OOO#OOO###OOOOO#
#.#.#v#######O###O###O#
#...#.>.#...>OOO#O###O#
#####v#.#.###v#O#O###O#
#.....#...#...#O#O#OOO#
#.#########.###O#O#O###
#...###...#...#OOO#O###
###.###.#.###v#####O###
#...#...#.#.>.>.#.>O###
#.###.###.#.###.#.#O###
#.....###...###...#OOO#
#####################O#

This hike contains 94 steps.
(The other possible hikes you could have taken were 90, 86, 82, 82, and 74 steps long.)

Find the longest hike you can take through the hiking
trails listed on your map. How many steps long is the longest hike?

*/
enum Tile {
    Forest,
    Path,
    Slope((usize, usize)),
}

struct Maze {
    maze: Vec<Vec<Tile>>,
    x_size: usize,
    y_size: usize,
}

impl Maze {
    fn new(input: &str) -> Maze {
        let mut maze = Vec::new();
        let mut x_size = 0;
        let mut y_size = 0;

        for (y, line) in input.lines().enumerate() {
            y_size = y;
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                x_size = x;
                row.push(match c {
                    '>' => Tile::Slope((x + 1, y)),
                    '<' => Tile::Slope((x - 1, y)),
                    'v' => Tile::Slope((x, y + 1)),
                    '^' => Tile::Slope((x, y - 1)),
                    '.' => Tile::Path,
                    '#' => Tile::Forest,
                    _ => unreachable!("Bad input: {c}"),
                });
            }
            maze.push(row);
        }

        Maze {
            maze,
            x_size,
            y_size,
        }
    }

    fn walk_tile(
        &self,
        visited: BTreeSet<(usize, usize)>,
        x: usize,
        y: usize,
        result: &mut Vec<usize>,
    ) {
        if x == self.x_size - 1 && y == self.y_size {
            result.push(visited.len());

            return;
        }
        if visited.contains(&(x, y)) {
            return;
        }
        let mut visited = visited;
        visited.insert((x, y));
        match self.maze[y][x] {
            Tile::Path => {
                self.walk_tile(visited.clone(), x + 1, y, result);
                if x > 0 {
                    self.walk_tile(visited.clone(), x - 1, y, result);
                }
                self.walk_tile(visited.clone(), x, y + 1, result);
                if y > 0 {
                    self.walk_tile(visited.clone(), x, y - 1, result)
                }
            }
            Tile::Slope((to_x, to_y)) => self.walk_tile(visited, to_x, to_y, result),
            _ => (),
        }
    }

    fn walk(&self) -> Vec<usize> {
        let mut result = Vec::new();
        self.walk_tile(BTreeSet::new(), 1, 0, &mut result);
        result
    }
}

fn process(input: &str) -> String {
    let mut maze = Maze::new(input);

    let mut results = maze.walk();
    
    results.sort();

    return results.last().unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        let result = process(input);
        assert_eq!(result, "94".to_string());
    }
}
