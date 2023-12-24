use std::collections::{BTreeMap, BTreeSet, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Part Two ---

As you reach the trailhead, you realize that the
ground isn't as slippery as you expected; you'll
have no problem climbing up the steep slopes.

Now, treat all slopes as if they were normal paths (.).
You still want to make sure you have the most scenic hike possible,
so continue to ensure that you never step onto the same tile twice.
What is the longest hike you can take?

In the example above, this increases the longest hike to 154 steps:

#S#####################
#OOOOOOO#########OOO###
#######O#########O#O###
###OOOOO#.>OOO###O#O###
###O#####.#O#O###O#O###
###O>...#.#O#OOOOO#OOO#
###O###.#.#O#########O#
###OOO#.#.#OOOOOOO#OOO#
#####O#.#.#######O#O###
#OOOOO#.#.#OOOOOOO#OOO#
#O#####.#.#O#########O#
#O#OOO#...#OOO###...>O#
#O#O#O#######O###.###O#
#OOO#O>.#...>O>.#.###O#
#####O#.#.###O#.#.###O#
#OOOOO#...#OOO#.#.#OOO#
#O#########O###.#.#O###
#OOO###OOO#OOO#...#O###
###O###O#O###O#####O###
#OOO#OOO#O#OOO>.#.>O###
#O###O###O#O###.#.#O###
#OOOOO###OOO###...#OOO#
#####################O#

Find the longest hike you can take through the surprisingly dry hiking
trails listed on your map. How many steps long is the longest hike?

*/
#[derive(PartialEq, Eq)]
enum Tile {
    Forest,
    Path,
}
struct Maze {
    maze: Vec<Vec<Tile>>,
    x_size: usize,
    y_size: usize,
    cache: BTreeMap<(usize, usize), BTreeSet<(usize, usize)>>,
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
                    '>' => Tile::Path,
                    '<' => Tile::Path,
                    'v' => Tile::Path,
                    '^' => Tile::Path,
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
            cache: BTreeMap::new(),
        }
    }

    fn walk_tile(
        &mut self,
        x: usize,
        y: usize,
        traversed: BTreeSet<(usize, usize)>,
    ) -> Option<BTreeSet<(usize, usize)>> {
        if x == self.x_size - 1 && y == self.y_size {
            dbg!(traversed.len(), &self.cache.len());
            return Some(BTreeSet::new());
        }
        if self.maze[y][x] != Tile::Path {
            return None;
        }
        if traversed.contains(&(x, y)) {
            return None;
        }
        if let Some(&ref cache) = self.cache.get(&(x, y)) {
            if cache.len() > traversed.len() {
                return None;
            }
            // return cache.clone();
        }
        let mut traversed = traversed;
        traversed.insert((x, y));
        let mut max = None;
        if let Some(mut back_track) = self.walk_tile(x + 1, y, traversed.clone()) {
            if back_track.insert((x + 1, y)) {
                max = Some(back_track);
            }
        }
        if let Some(mut back_track) = self.walk_tile(x, y + 1, traversed.clone()) {
            if back_track.insert((x, y + 1)) {
                if let Some(cur_max) = &max {
                    if back_track.len() > cur_max.len() {
                        max = Some(back_track);
                    }
                } else {
                    max = Some(back_track);
                }
            }
        }
        if x > 0 {
            if let Some(mut back_track) = self.walk_tile(x - 1, y, traversed.clone()) {
                if back_track.insert((x - 1, y)) {
                    if let Some(cur_max) = &max {
                        if back_track.len() > cur_max.len() {
                            max = Some(back_track);
                        }
                    } else {
                        max = Some(back_track);
                    }
                }
            }
        }
        if y > 0 {
            if let Some(mut back_track) = self.walk_tile(x, y - 1, traversed.clone()) {
                if back_track.insert((x, y - 1)) {
                    if let Some(cur_max) = &max {
                        if back_track.len() > cur_max.len() {
                            max = Some(back_track);
                        }
                    } else {
                        max = Some(back_track);
                    }
                }
            }
        }
        // if let Some(cache) = &max {
        //     dbg!(cache.len());
        //     for py in 0..=self.y_size {
        //         for px in 0..=self.x_size {
        //             match self.maze[py][px] {
        //                 Tile::Forest => print!("█"),
        //                 Tile::Path => {
        //                     if px == x && py == y {
        //                         print!("O");
        //                     } else if cache.contains(&(px, py)) {
        //                         print!(".");
        //                     } else if traversed.contains(&(px, py)) {
        //                         print!(",")
        //                     } else {
        //                         print!(" ")
        //                     }
        //                 }
        //             }
        //         }
        //         print!("\n");
        //     }
            // self.cache.insert((x, y), traversed.clone());
        // }

        max
    }

    fn walk(&mut self) -> usize {
        self.cache.clear();
        let walk = self.walk_tile(1, 0, BTreeSet::new());

        // for ((x, y), c) in &self.cache {
        if let Some(cache) = walk {
            dbg!(cache.len());
            for y in 0..=self.y_size {
                for x in 0..=self.x_size {
                    match self.maze[y][x] {
                        Tile::Forest => print!("█"),
                        Tile::Path => {
                            if cache.contains(&(x, y)) {
                                print!(".");
                            } else {
                                print!(" ")
                            }
                        }
                    }
                }
                print!("\n");
            }
        }
        // }

        0
    }
}

fn process(input: &str) -> String {
    let mut maze = Maze::new(input);

    return maze.walk().to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    //     #[test]
    //     fn it_works_small() {
    //         let input = "#.####
    // #....#
    // #....#
    // #....#
    // ####.#";
    //         let result = process(input);
    //         assert_eq!(result, "_".to_string());
    //     }
//     #[test]
//     fn it_works_small() {
//         let input = "#.#####
// #.....#
// #.....#
// #####.#";
//         let result = process(input);
//         assert_eq!(result, "_".to_string());
//     }

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
            assert_eq!(result, "_".to_string());
        }
}
