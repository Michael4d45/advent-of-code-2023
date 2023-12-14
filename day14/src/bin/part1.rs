use std::collections::BTreeMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 14: Parabolic Reflector Dish ---

You reach the place where all of the mirrors were pointing: a massive
parabolic reflector dish attached to the side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors
themselves are roughly in the shape of a parabolic reflector dish,
each individual mirror seems to be pointing in slightly the wrong
direction. If the dish is meant to focus light, all it's doing right
now is sending it in a vague direction.

This system must be what provides the energy for the lava! If you focus
the reflector dish, maybe you can go where it's pointing and use the
light to fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected
via an elaborate system of ropes and pulleys to a large metal platform
below the dish. The platform is covered in large rocks of various shapes.
Depending on their position, the weight of the rocks deforms the platform,
and the shape of the platform controls which ropes move and ultimately the
focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even
has a control panel on the side that lets you tilt it in one of four
directions! The rounded rocks (O) will roll when the platform is tilted,
while the cube-shaped rocks (#) will stay in place. You note the positions
of all of the empty spaces (.) and rocks (your puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....

Start by tilting the lever so all of the rocks will slide north as far as they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....

You notice that the support beams along the north side of the platform are
damaged; to ensure the platform doesn't collapse, you should calculate the
total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number
of rows from the rock to the south edge of the platform, including the row
the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the
amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10 5 50
OO..#....#  9 2 18 68
OO..O##..O  8 4 32 100
O..#.OO...  7 3 21 121
........#.  6
..#....#.#  5
..O..#.O.O  4 3 12 133
..O.......  3 1 3  136
#....###..  2
#....#....  1

The total load is the sum of the load caused by all of the rounded rocks.
In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward,
what is the total load on the north support beams?

*/
fn process(input: &str) -> String {
    let count = input.lines().count() - 1;
    let mut matrix = BTreeMap::new();
    let mut squares: BTreeMap<(usize, usize), (usize, Vec<usize>)> = BTreeMap::new();
    let mut last_square_at_x = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            matrix.insert((x, y), c);
            if c == '#' {
                squares.insert((x, y), (y, vec![]));

                if let Some(last_y) = last_square_at_x.remove(&x) {
                    let last_square = squares.remove(&(x, last_y)).expect("Should have value.");
                    squares.insert((x, last_y), (y - 1, last_square.1));
                }

                last_square_at_x.insert(x, y);
            };
            if c == 'O' {
                let &last_y = last_square_at_x.get(&x).expect("I set all the rocks");
                let (_, mut circles) = squares.remove(&(x, last_y)).expect("Should have value.");
                circles.push(y);
                squares.insert((x, last_y), (y, circles));
            }
        }
    }

    let circles = squares
        .iter()
        .filter_map(|((_, y), (_, value))| (!value.is_empty()).then_some((count - y, value)));

    let maths = circles.map(|(pos, list)| {
        let mut weight = 0;
        for i in 0..list.len() {
            weight += pos - i;
        }
        weight
    });

    // dbg!(maths.sum::<usize>());

    return maths.sum::<usize>().to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "##########
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....",
        );
        assert_eq!(result, "136".to_string());
    }
}
