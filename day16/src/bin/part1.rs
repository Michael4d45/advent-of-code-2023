use core::fmt;
use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 16: The Floor Will Be Lava ---

With the beam of light completely focused somewhere, the
reindeer leads you deeper still into the Lava Production
Facility. At some point, you realize that the steel facility
walls have been replaced with cave, and the doorways are
just cave, and the floor is cave, and you're pretty sure this
is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain,
you see a bright light in a cavern up ahead. There, you discover
that the beam of light you so carefully focused is emerging from
the cavern wall closest to the facility and pouring all of its
energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat,
two-dimensional square grid containing empty space (.),
mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces
around the grid, but each tile on the grid converts some of
the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input).
For example:

.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....

The beam enters in the top-left corner from the left and
heading to the right. Then, its behavior depends on what
it encounters as it moves:

    If the beam encounters empty space (.), it continues in the same direction.
    If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees
        depending on the angle of the mirror. For instance, a rightward-moving
        beam that encounters a / mirror would continue upward in the mirror's
        column, while a rightward-moving beam that encounters a \ mirror would
        continue downward from the mirror's column.
    If the beam encounters the pointy end of a splitter (| or -), the beam passes
        through the splitter as if the splitter were empty space. For instance, a
        rightward-moving beam that encounters a - splitter would continue in the
        same direction.
    If the beam encounters the flat side of a splitter (| or -), the beam is split
        into two beams going in each of the two directions the splitter's pointy
        ends are pointing. For instance, a rightward-moving beam that encounters
        a | splitter would split into two beams: one that continues upward from
        the splitter's column and one that continues downward from the splitter's
        column.

Beams do not interact with other beams; a tile can have many beams passing
through it at the same time. A tile is energized if that tile has at least
one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the contraption:

>|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v..

Beams are only shown on empty tiles; arrows indicate the direction of the beams.
If a tile contains beams moving in multiple directions, the number of distinct
directions is shown instead. Here is the same diagram but instead only showing
whether a tile is energized (#) or not (.):

######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..

Ultimately, in this example, 46 tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the contraption,
you need to start by analyzing the current situation. With the beam starting in
the top-left heading right, how many tiles end up being energized?


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

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Grid {
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
            matrix,
            energy: vec![],
            size_x,
            size_y,
        }
    }

    fn init(&mut self) {
        let space = self.matrix.get_mut(&(0, 0)).expect("Should have (0,0)");
        self.energy.push((0, 0, Direction::Right));

        // dbg!(self);
    }

    fn process(&mut self) {
        use Direction::*;

        let mut cc = 0;

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
            cc += 1;
            if cc % 5 == 0 {
                println!("{self}");
                println!("count: {cc}")
            }
        }
        println!("{self}");
        println!("count: {cc}")
    }

    fn get_energized(&self) -> usize {
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

    grid.init();
    grid.process();

    return grid.get_energized().to_string();
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
        assert_eq!(result, "46".to_string());
    }
}
