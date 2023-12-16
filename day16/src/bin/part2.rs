use core::fmt;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**--- Part Two ---

As you try to work out what might be wrong, the reindeer tugs on your
shirt and leads you to a nearby control panel. There, a collection of
buttons lets you align the contraption so that the beam enters from any
edge tile and heading away from that edge. (You can choose either of two
    directions for the beam if it starts on a corner; for instance, if
    the beam starts in the bottom-right corner, it can start heading either
    left or upward.)

So, the beam could start on any tile in the top row (heading downward),
any tile in the bottom row (heading upward), any tile in the leftmost
column (heading right), or any tile in the rightmost column (heading left).
To produce lava, you need to find the configuration that energizes as many
tiles as possible.

In the above example, this can be achieved by starting the beam in the
fourth tile from the left in the top row:

.|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v..

Using this configuration, 51 tiles are energized:

.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..

Find the initial beam configuration that energizes the largest number
of tiles; how many tiles are energized in that configuration?
*/
#[derive(Debug, Clone)]
enum SplitType {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
enum MirrorType {
    Forward,
    Back,
}

#[derive(Debug, Clone)]
enum SpaceType {
    Split(SplitType),
    Mirror(MirrorType),
}

#[derive(Debug, Clone)]
struct Space {
    space_type: Option<SpaceType>,
    to_go_up: bool,
    finished_go_up: bool,
    to_go_down: bool,
    finished_go_down: bool,
    to_go_left: bool,
    finished_go_left: bool,
    to_go_right: bool,
    finished_go_right: bool,
    is_energized: bool,
}

impl Space {
    fn new(space_type: Option<SpaceType>) -> Space {
        Space {
            space_type,
            to_go_up: false,
            finished_go_up: false,
            to_go_down: false,
            finished_go_down: false,
            to_go_left: false,
            finished_go_left: false,
            to_go_right: false,
            finished_go_right: false,
            is_energized: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Grid {
    start_matrix: BTreeMap<(usize, usize), Space>,
    matrix: BTreeMap<(usize, usize), Space>,
    energy: Vec<(usize, usize, Direction)>,
    size_x: usize,
    size_y: usize,
}

impl Grid {
    fn new(input: &str) -> Grid {
        let mut matrix = BTreeMap::new();
        let mut size_y = 0;
        let mut size_x = 0;
        for (y, line) in input.lines().enumerate() {
            size_y = size_y.max(y);
            for (x, c) in line.chars().enumerate() {
                size_x = size_x.max(x);
                matrix.insert(
                    (x, y),
                    Space::new(match c {
                        '.' => None,
                        '-' => Some(SpaceType::Split(SplitType::Horizontal)),
                        '|' => Some(SpaceType::Split(SplitType::Vertical)),
                        '/' => Some(SpaceType::Mirror(MirrorType::Forward)),
                        '\\' => Some(SpaceType::Mirror(MirrorType::Back)),
                        _ => unreachable!("Bad input {c}"),
                    }),
                );
            }
        }
        Grid {
            matrix: BTreeMap::new(),
            start_matrix: matrix.clone(),
            energy: vec![],
            size_x,
            size_y,
        }
    }

    fn process(&mut self, start: (usize, usize, Direction)) -> usize {
        use Direction::*;

        self.energy.push(start);
        self.matrix = self.start_matrix.clone();

        while let Some((x, y, direction)) = &self.energy.pop() {
            if let Some(space) = self.matrix.get_mut(&(*x, *y)) {
                space.is_energized = true;
                match direction {
                    Up => space.to_go_up = true,
                    Down => space.to_go_down = true,
                    Left => space.to_go_left = true,
                    Right => space.to_go_right = true,
                }
                match space.space_type.clone() {
                    Some(space_type) => match space_type {
                        SpaceType::Split(split_type) => match split_type {
                            SplitType::Horizontal => {
                                if space.to_go_right && !space.finished_go_right {
                                    space.finished_go_right = true;
                                    self.energy.push((x + 1, *y, Right));
                                }
                                if space.to_go_left && !space.finished_go_left {
                                    space.finished_go_left = true;
                                    if *x > 0 {
                                        self.energy.push((x - 1, *y, Left));
                                    }
                                }
                                if space.to_go_up && !space.finished_go_up {
                                    space.finished_go_up = true;
                                    space.finished_go_down = true;
                                    if *x > 0 {
                                        self.energy.push((x - 1, *y, Left));
                                    }
                                    self.energy.push((x + 1, *y, Right));
                                }
                                if space.to_go_down && !space.finished_go_down {
                                    space.finished_go_down = true;
                                    space.finished_go_up = true;
                                    if *x > 0 {
                                        self.energy.push((x - 1, *y, Left));
                                    }
                                    self.energy.push((x + 1, *y, Right));
                                }
                            }
                            SplitType::Vertical => {
                                if space.to_go_right && !space.finished_go_right {
                                    space.finished_go_right = true;
                                    space.finished_go_left = true;
                                    self.energy.push((*x, y + 1, Down));
                                    if *y > 0 {
                                        self.energy.push((*x, y - 1, Up));
                                    }
                                }
                                if space.to_go_left && !space.finished_go_left {
                                    space.finished_go_left = true;
                                    space.finished_go_right = true;
                                    self.energy.push((*x, y + 1, Down));
                                    if *y > 0 {
                                        self.energy.push((*x, y - 1, Up));
                                    }
                                }
                                if space.to_go_up && !space.finished_go_up {
                                    space.finished_go_up = true;
                                    if *y > 0 {
                                        self.energy.push((*x, y - 1, Up));
                                    }
                                }
                                if space.to_go_down && !space.finished_go_down {
                                    space.finished_go_down = true;
                                    self.energy.push((*x, y + 1, Down));
                                }
                            }
                        },
                        SpaceType::Mirror(mirror_type) => match mirror_type {
                            MirrorType::Forward => {
                                if space.to_go_right && !space.finished_go_right {
                                    space.finished_go_right = true;
                                    if *y > 0 {
                                        self.energy.push((*x, y - 1, Up));
                                    }
                                }
                                if space.to_go_left && !space.finished_go_left {
                                    space.finished_go_left = true;
                                    self.energy.push((*x, y + 1, Down));
                                }
                                if space.to_go_up && !space.finished_go_up {
                                    space.finished_go_up = true;
                                    self.energy.push((x + 1, *y, Right));
                                }
                                if space.to_go_down && !space.finished_go_down {
                                    space.finished_go_down = true;
                                    if *x > 0 {
                                        self.energy.push((x - 1, *y, Left));
                                    }
                                }
                            }
                            MirrorType::Back => {
                                if space.to_go_right && !space.finished_go_right {
                                    space.finished_go_right = true;
                                    self.energy.push((*x, y + 1, Down));
                                }
                                if space.to_go_left && !space.finished_go_left {
                                    space.finished_go_left = true;
                                    if *y > 0 {
                                        self.energy.push((*x, y - 1, Up));
                                    }
                                }
                                if space.to_go_up && !space.finished_go_up {
                                    space.finished_go_up = true;
                                    if *x > 0 {
                                        self.energy.push((x - 1, *y, Left));
                                    }
                                }
                                if space.to_go_down && !space.finished_go_down {
                                    space.finished_go_down = true;
                                    self.energy.push((x + 1, *y, Right));
                                }
                            }
                        },
                    },
                    None => {
                        if space.to_go_right && !space.finished_go_right {
                            space.finished_go_right = true;
                            self.energy.push((x + 1, *y, Right));
                        }
                        if space.to_go_left && !space.finished_go_left {
                            space.finished_go_left = true;
                            if *x > 0 {
                                self.energy.push((x - 1, *y, Left));
                            }
                        }
                        if space.to_go_up && !space.finished_go_up {
                            space.finished_go_up = true;
                            if *y > 0 {
                                self.energy.push((*x, y - 1, Up));
                            }
                        }
                        if space.to_go_down && !space.finished_go_down {
                            space.finished_go_down = true;
                            self.energy.push((*x, y + 1, Down));
                        }
                    }
                };
            }
        }

        self.matrix
            .iter()
            .filter_map(|(_, space)| space.is_energized.then_some(space))
            .count()
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for row in 0..=self.size_y {
            for col in 0..=self.size_x {
                let position = (col, row);
                let space = self.matrix.get(&position).unwrap();

                if space.is_energized {
                    output.push('#');
                } else {
                    match space.space_type {
                        Some(SpaceType::Split(SplitType::Horizontal)) => output.push('-'),
                        Some(SpaceType::Split(SplitType::Vertical)) => output.push('|'),
                        Some(SpaceType::Mirror(MirrorType::Forward)) => output.push('/'),
                        Some(SpaceType::Mirror(MirrorType::Back)) => output.push('\\'),
                        None => output.push('.'),
                    }
                }
                // output.push(' ');
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}

fn process(input: &str) -> String {
    let mut grid = Grid::new(input);

    use Direction::*;
    let mut to_process = vec![];
    for x in 0..=grid.size_x {
        to_process.push((x, 0, Down));
        to_process.push((x, grid.size_y, Up));
    }
    for y in 0..=grid.size_y {
        to_process.push((0, y, Right));
        to_process.push((grid.size_x, y, Left));
    }
    let score = to_process.iter().map(|&start| grid.process(start)).max();

    return score.unwrap().to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....",
        );
        assert_eq!(result, "51".to_string());
    }
}
