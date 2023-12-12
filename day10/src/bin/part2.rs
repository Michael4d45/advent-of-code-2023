fn main() {
    let input = include_str!("./input.txt");
    let output = process(input, Direction::Up);
    dbg!(output);
}

/**
--- Part Two ---

You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........

The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop.
Here is the same loop again with those regions marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....

In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed!
Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........

In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...

The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO

In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk
pipe lying around that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L

In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?

*/
#[derive(Debug)]
enum RelationDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
enum PipeType {
    Empty,
    Start,
    EastWest,
    NorthEast,
    SouthEast,
    NorthWest,
    SouthWest,
    NorthSouth,
}

#[derive(Debug, Clone, Copy)]
struct Pipe {
    x: isize,
    y: isize,
    pipe_type: PipeType,
    is_inside: bool,
    bound: Option<Direction>,
    is_checked: bool,
}

fn can_connect(pipe: &Pipe, prev_pipe: &Pipe) -> bool {
    use PipeType::*;

    match prev_pipe.pipe_type {
        Empty => panic!("How did we get here???"),
        Start => match pipe.pipe_type {
            Empty => false,
            Start => todo!(),
            EastWest => pipe.y == prev_pipe.y,
            NorthEast => {
                let is_on_left = pipe.y == prev_pipe.y && pipe.x == prev_pipe.x - 1;
                let is_below = pipe.x == prev_pipe.x && pipe.y == prev_pipe.y + 1;
                is_below || is_on_left
            }
            SouthEast => {
                let is_on_left = pipe.y == prev_pipe.y && pipe.x == prev_pipe.x - 1;
                let is_above = pipe.x == prev_pipe.x && pipe.y == prev_pipe.y - 1;
                is_above || is_on_left
            },
            NorthWest => {
                let is_on_right = pipe.y == prev_pipe.y && pipe.x == prev_pipe.x + 1;
                let is_below = pipe.x == prev_pipe.x && pipe.y == prev_pipe.y + 1;
                is_below || is_on_right
            }
            SouthWest => {
                let is_on_right = pipe.y == prev_pipe.y && pipe.x == prev_pipe.x + 1;
                let is_above = pipe.x == prev_pipe.x && pipe.y == prev_pipe.y - 1;
                is_above || is_on_right
            }
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

fn is_inside(pipe: Pipe, direction: RelationDirection) -> bool {
    // dbg!(pipe, &direction);
    if let Some(bound) = pipe.bound {
        match pipe.pipe_type {
            PipeType::EastWest => match direction {
                RelationDirection::Up => match bound {
                    Direction::Up => todo!(),   // false,
                    Direction::Down => todo!(), // false,
                },
                RelationDirection::Down => match bound {
                    Direction::Up => todo!(),   // true,
                    Direction::Down => todo!(), // false,
                },
                _ => panic!("NOOOOO2"),
            },
            PipeType::NorthEast => match bound {
                Direction::Up => false,
                Direction::Down => true,
            },
            PipeType::SouthEast => match bound {
                Direction::Up => false,
                Direction::Down => true,
            },
            PipeType::NorthWest => match bound {
                Direction::Up => true,
                Direction::Down => false,
            },
            PipeType::SouthWest => match bound {
                Direction::Up => true,
                Direction::Down => false,
            },
            PipeType::NorthSouth => match direction {
                RelationDirection::Left => match bound {
                    Direction::Up => false,
                    Direction::Down => true,
                },
                RelationDirection::Right => match bound {
                    Direction::Up => true,
                    Direction::Down => false,
                },
                _ => panic!("NOOO1"),
            },
            _ => panic!("Something's gone terribly wrong"),
        }
    } else {
        pipe.is_inside
    }
}

fn get_direction(pipe: Pipe, prev_pipe: Pipe) -> Direction {
    let same = prev_pipe.bound.unwrap();
    let opposite = match same {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
    };
    match prev_pipe.pipe_type {
        PipeType::EastWest => match pipe.pipe_type {
            PipeType::EastWest => same,
            PipeType::NorthEast => opposite,
            PipeType::SouthEast => same,
            PipeType::NorthWest => same,
            PipeType::SouthWest => opposite,
            _ => panic!("ono1"),
        },
        PipeType::NorthEast => match pipe.pipe_type {
            PipeType::EastWest => opposite,
            PipeType::SouthEast => same,
            PipeType::NorthWest => opposite,
            PipeType::SouthWest => same,
            PipeType::NorthSouth => same,
            _ => {
                dbg!(pipe, prev_pipe);
                panic!("ono2")
            }
        },
        PipeType::SouthEast => match pipe.pipe_type {
            PipeType::EastWest => same,
            PipeType::NorthEast => same,
            PipeType::NorthWest => same,
            PipeType::SouthWest => opposite,
            PipeType::NorthSouth => same,
            _ => panic!("ono3"),
        },
        PipeType::NorthWest => match pipe.pipe_type {
            PipeType::EastWest => same,
            PipeType::NorthEast => opposite,
            PipeType::SouthEast => same,
            PipeType::SouthWest => same,
            PipeType::NorthSouth => same,
            _ => panic!("ono4"),
        },
        PipeType::SouthWest => match pipe.pipe_type {
            PipeType::EastWest => opposite,
            PipeType::NorthEast => same,
            PipeType::SouthEast => opposite,
            PipeType::NorthWest => same,
            PipeType::NorthSouth => same,
            _ => panic!("ono5"),
        },
        PipeType::NorthSouth => match pipe.pipe_type {
            PipeType::NorthEast => same,
            PipeType::SouthEast => same,
            PipeType::NorthWest => same,
            PipeType::SouthWest => same,
            PipeType::NorthSouth => same,
            _ => panic!("ono6"),
        },
        _ => panic!("ono0"),
    }
}

fn process(input: &str, start_direction: Direction) -> String {
    let mut grid: Vec<Vec<Pipe>> = vec![];
    let mut start = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut temp_grid: Vec<Pipe> = vec![];
        for (x, c) in line.chars().enumerate() {
            temp_grid.push(Pipe {
                x: x as isize,
                y: y as isize,
                is_inside: false,
                bound: None,
                is_checked: false,
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

    let first = winning_path.first().unwrap();
    let last = &winning_path[&winning_path.len() - 2];

    let start_type = if first.x > start.0 {
        if last.y > start.1 {
            PipeType::SouthEast
        } else if last.y < start.1 {
            PipeType::NorthEast
        } else {
            PipeType::EastWest
        }
    } else if first.x == start.0 {
        if last.x > start.0 {
            if first.y > start.0 {
                PipeType::SouthEast
            } else {
                PipeType::NorthEast
            }
        } else if last.x < start.0 {
            if first.y > start.0 {
                PipeType::SouthWest
            } else {
                PipeType::NorthWest
            }
        } else {
            PipeType::NorthSouth
        }
    } else {
        if last.y > start.1 {
            PipeType::SouthWest
        } else if last.y < start.1 {
            PipeType::NorthWest
        } else {
            PipeType::EastWest
        }
    };

    dbg!(first, last, start_type);

    let mut pairs = vec![];

    // Add the special case [last_element, first_element]
    if let Some(&last_element) = winning_path.last() {
        pairs.push(vec![last_element, winning_path[0]]);
        let set_pipe = &mut grid[last_element.y as usize][last_element.x as usize];
        set_pipe.bound = Some(start_direction);
        set_pipe.pipe_type = start_type;
        set_pipe.is_checked = true;
    }

    // Use windows(2) for the rest of the pairs
    pairs.extend(winning_path.windows(2).map(|w| w.to_vec()));

    for pipes in pairs {
        let prev_pipe_t = pipes[0];
        let prev_pipe = grid[prev_pipe_t.y as usize][prev_pipe_t.x as usize];
        let pipe = pipes[1];
        let set_pipe = &mut grid[pipe.y as usize][pipe.x as usize];
        let direction = get_direction(set_pipe.clone(), prev_pipe);
        set_pipe.bound = Some(direction);
        set_pipe.is_checked = true;
    }

    let mut to_check = vec![];
    for y in 0..grid.len() {
        let grid_line = &mut grid[y];
        for x in 0..grid_line.len() {
            let pipe = &mut grid_line[x];
            if pipe.bound.is_none() {
                if y == 0
                    || x == 0
                    || y == (grid_size.1 - 1) as usize
                    || x == (grid_size.0 - 1) as usize
                {
                    pipe.is_checked = true;
                } else {
                    to_check.push((x, y));
                }
            }
        }
    }

    let mut count = 0;
    for y in 0..grid.len() {
        let grid_line = &mut grid[y];
        for x in 0..grid_line.len() {
            let pipe = &mut grid_line[x];
            if pipe.is_checked {
                count += 1;
            }
        }
    }

    dbg!(count);

    dbg!(to_check.len());

    let mut to_check2 = vec![];
    while !to_check.is_empty() {
        to_check2.clear();
        // dbg!(to_check.len());
        while let Some((x, y)) = to_check.pop() {
            let left_pipe = grid[y][x - 1];
            if left_pipe.is_checked {
                // dbg!(x, y);
                let set_pipe = &mut grid[y][x];
                set_pipe.is_inside = is_inside(left_pipe, RelationDirection::Right);
                set_pipe.is_checked = true;
                continue;
            }

            let right_pipe = grid[y][x + 1];
            if right_pipe.is_checked {
                let set_pipe = &mut grid[y][x];
                set_pipe.is_inside = is_inside(right_pipe, RelationDirection::Left);
                // dbg!(x, y, set_pipe.is_inside);
                set_pipe.is_checked = true;
                continue;
            }

            let down_pipe = grid[y - 1][x];
            if down_pipe.is_checked {
                // dbg!(x, y);
                let set_pipe = &mut grid[y][x];
                set_pipe.is_inside = is_inside(down_pipe, RelationDirection::Down);
                set_pipe.is_checked = true;
                continue;
            }

            let up_pipe = grid[y + 1][x];
            if up_pipe.is_checked {
                // dbg!(x, y);
                let set_pipe = &mut grid[y][x];
                set_pipe.is_inside = is_inside(up_pipe, RelationDirection::Up);
                set_pipe.is_checked = true;
                continue;
            }

            to_check2.push((x, y));
        }
        to_check = to_check2.clone();
    }

    let mut count = 0;
    for y in 0..grid.len() {
        let grid_line = &mut grid[y];
        for x in 0..grid_line.len() {
            let pipe = &mut grid_line[x];
            if pipe.is_inside {
                count += 1;
                print!("I ");
            } else {
                if let Some(bound) = &grid_line[x].bound {
                    match &grid_line[x].pipe_type {
                        PipeType::EastWest => print!("─"),
                        PipeType::NorthEast => print!("└"),
                        PipeType::SouthEast => print!("┌"),
                        PipeType::NorthWest => print!("┘"),
                        PipeType::SouthWest => print!("┐"),
                        PipeType::NorthSouth => print!("│"),
                        _ => unreachable!("WHATTT!!!"),
                    }
                    let b = match bound {
                        Direction::Up => 'u',
                        Direction::Down => 'd',
                    };
                    print!("{}", b);
                } else {
                    print!(". ");
                }
            }
            // print!(" ({},{})  ", x, y);
        }
        print!("\n");
    }

    return count.to_string();
}

#[cfg(test)]
mod tests {
    use crate::{process, Direction};

    #[test]
    fn it_works() {
        let result = process(
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........",
            Direction::Up,
        );
        assert_eq!(result, "4".to_string());

        let result = process(
            ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...",
            Direction::Down,
        );
        assert_eq!(result, "8".to_string());

        let result = process(
            "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L",
            Direction::Down,
        );
        assert_eq!(result, "10".to_string());
    }
}
