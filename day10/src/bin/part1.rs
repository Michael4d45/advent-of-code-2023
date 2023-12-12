fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 10: Pipe Maze ---

You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island.
This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals.
However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction;
maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal.
As you stop to admire some metal grass, you notice something metallic scurry
away in your peripheral vision and jump into a big pipe! It didn't look like
any animal you've ever seen; if you want a better look, you'll need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely
packed with pipes; it was hard to tell at first because they're the same metallic
silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

    | is a vertical pipe connecting north and south.
    - is a horizontal pipe connecting east and west.
    L is a 90-degree bend connecting north and east.
    J is a 90-degree bend connecting north and west.
    7 is a 90-degree bend connecting south and west.
    F is a 90-degree bend connecting south and east.
    . is ground; there is no pipe in this tile.
    S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....

If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....

In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF

In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to,
pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will
    have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ

If you want to get out ahead of the animal, you should find the tile in the loop that is farthest
from the starting position. Because the animal is in the pipe, it doesn't make sense to measure
this by direct distance. Instead, you need to find the tile that would take the longest number
of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....

You can count the distance each tile in the loop is from the starting point like this:

.....
.012.
.1.3.
.234.
.....

In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...

Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...

Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?

*/
#[derive(Debug, Clone, Default)]
enum PipeType {
    #[default]
    Empty,
    Start,
    EastWest,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
    NorthSouth,
}

#[derive(Debug, Clone, Default)]
struct Pipe {
    x: isize,
    y: isize,
    pipe_type: PipeType,
}

fn can_connect(pipe: &Pipe, prev_pipe: &Pipe) -> bool {
    use PipeType::*;

    // dbg!(pipe, prev_pipe);
    match prev_pipe.pipe_type {
        Empty => panic!("How did we get here???"),
        Start => match pipe.pipe_type {
            Empty => false,
            Start => true,
            EastWest => pipe.y == prev_pipe.y,
            NorthEast => {
                let is_on_left = pipe.y == prev_pipe.y && pipe.x == prev_pipe.x - 1;
                let is_below = pipe.x == prev_pipe.x && pipe.y == prev_pipe.y + 1;
                is_below || is_on_left
            }
            SouthEast => todo!(),
            NorthWest => todo!(),
            SouthWest => todo!(),
            NorthSouth => pipe.x == prev_pipe.x,
        },
        EastWest => match pipe.pipe_type {
            Empty => false,
            Start => pipe.y == prev_pipe.y,
            EastWest => true,
            NorthEast => pipe.x < prev_pipe.x,
            SouthEast => pipe.x < prev_pipe.x,
            NorthWest => pipe.x > prev_pipe.x,
            SouthWest => pipe.x > prev_pipe.x,
            NorthSouth => false,
        },
        NorthEast => match pipe.pipe_type {
            Empty => false,
            Start => true,
            EastWest => pipe.y == prev_pipe.y,
            NorthEast => todo!(),
            SouthEast => prev_pipe.x == prev_pipe.x,
            NorthWest => pipe.y == prev_pipe.y,
            SouthWest => true,
            NorthSouth => prev_pipe.x == prev_pipe.x,
        },
        SouthEast => match pipe.pipe_type {
            Empty => false,
            Start => true,
            EastWest => pipe.y == prev_pipe.y,
            NorthEast => pipe.x == prev_pipe.x,
            SouthEast => todo!(),
            NorthWest => true,
            SouthWest => prev_pipe.x == prev_pipe.x,
            NorthSouth => prev_pipe.x == prev_pipe.x,
        },
        NorthWest => match pipe.pipe_type {
            Empty => false,
            Start => true,
            EastWest => pipe.y == prev_pipe.y,
            NorthEast => pipe.y == prev_pipe.y,
            SouthEast => prev_pipe.x == prev_pipe.x,
            NorthWest => false,
            SouthWest => prev_pipe.x == prev_pipe.x,
            NorthSouth => prev_pipe.x == prev_pipe.x,
        },
        SouthWest => match pipe.pipe_type {
            Empty => false,
            Start => true,
            EastWest => pipe.y == prev_pipe.y,
            NorthEast => true,
            SouthEast => pipe.y == prev_pipe.y,
            NorthWest => prev_pipe.x == prev_pipe.x,
            SouthWest => todo!(),
            NorthSouth => prev_pipe.x == prev_pipe.x,
        },
        NorthSouth => match pipe.pipe_type {
            Empty => false,
            Start => prev_pipe.x == prev_pipe.x,
            EastWest => false,
            NorthEast => prev_pipe.x == prev_pipe.x,
            SouthEast => prev_pipe.x == prev_pipe.x,
            NorthWest => prev_pipe.x == prev_pipe.x,
            SouthWest => prev_pipe.x == prev_pipe.x,
            NorthSouth => prev_pipe.x == prev_pipe.x,
        },
    }
}

fn process(input: &str) -> String {
    let mut grid: Vec<Vec<Pipe>> = vec![];
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut temp_grid: Vec<Pipe> = vec![];
        for (x, c) in line.chars().enumerate() {
            temp_grid.push(Pipe {
                x: x as isize,
                y: y as isize,
                pipe_type: match c {
                    '|' => PipeType::NorthSouth,
                    '-' => PipeType::EastWest,
                    'L' => PipeType::NorthEast,
                    'J' => PipeType::NorthWest,
                    '7' => PipeType::SouthWest,
                    'F' => PipeType::SouthEast,
                    '.' => PipeType::Empty,
                    'S' => PipeType::Start,
                    _ => panic!("ONO!"),
                },
            });
            if c == 'S' {
                start = (x as isize, y as isize);
            }
        }
        grid.push(temp_grid)
    }

    dbg!(start);
    // let mut stack = vec![start];

    let grid_size: (isize, isize) = (grid[0].len() as isize, grid.len() as isize);

    let (x, y) = start;
    let start_pipe = &grid[y as usize][x as usize];
    let starting_at = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
    let mut winning_path = vec![];
    let mut good_path;
    let mut path;
    let mut pipe;
    for (mut x, mut y) in starting_at {
        good_path = false;
        path = vec![];
        pipe = start_pipe;
        dbg!(x, y);
        loop {
            if x < 0 || y < 0 || x > grid_size.0 || y > grid_size.1 {
                break;
            }
            let prev_pipe = pipe;
            pipe = &grid[y as usize][x as usize];
            if !can_connect(pipe, prev_pipe) {
                break;
            }
            path.push(pipe.clone());

            match pipe.pipe_type {
                PipeType::Empty => break,
                PipeType::Start => {
                    good_path = true;
                    break;
                }
                PipeType::EastWest => {
                    if x > prev_pipe.x {
                        x = x + 1;
                    } else {
                        x = x - 1;
                    }
                }
                PipeType::NorthEast => {
                    if y > prev_pipe.y {
                        x = x + 1;
                    } else {
                        y = y - 1;
                    }
                }
                PipeType::SouthEast => {
                    if y < prev_pipe.y {
                        x = x + 1;
                    } else {
                        y = y + 1;
                    }
                }
                PipeType::NorthWest => {
                    if y > prev_pipe.y {
                        x = x - 1;
                    } else {
                        y = y - 1;
                    }
                }
                PipeType::SouthWest => {
                    if y < prev_pipe.y {
                        x = x - 1;
                    } else {
                        y = y + 1;
                    }
                }
                PipeType::NorthSouth => {
                    if y > prev_pipe.y {
                        y = y + 1;
                    } else {
                        y = y - 1;
                    }
                }
            };
        }

        if good_path {
            winning_path = path;
            dbg!(winning_path.len());
            break;
        }
    }

    if winning_path.is_empty() {
        panic!("No path found");
    }

    return (winning_path.len() / 2).to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        assert_eq!(result, "4".to_string());

        let result = process(
            "..F7.
.FJ|.
SJ.L7
|F--J
LJ...",
        );
        assert_eq!(result, "8".to_string());
    }
}
