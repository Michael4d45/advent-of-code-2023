fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 13: Point of Incidence ---

With your help, the hot springs team locates an appropriate spring which
launches you neatly and precisely up to the edge of Lava Island.

There's just one problem: you don't see any lava.

You do see a lot of ash and igneous rock; there are even what look like
gray mountains scattered around. After a while, you make your way to a
nearby cluster of mountains only to discover that the valley between them
is completely full of large mirrors. Most of the mirrors seem to be aligned
in a consistent way; perhaps you should head in that direction?

As you move through the valley of mirrors, you find that several of them have
fallen from the large metal frames keeping them in place. The mirrors are
extremely flat and shiny, and many of the fallen mirrors have lodged into
the ash at strange angles. Because the terrain is all one color, it's hard
to tell where it's safe to walk or where you're about to run into a mirror.

You note down the patterns of ash (.) and rocks (#) that you see as you walk
(your puzzle input); perhaps by carefully analyzing these patterns,
you can figure out where the mirrors are!

For example:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

To find the reflection in each pattern, you need to find a perfect reflection
across either a horizontal line between two rows or across a
vertical line between two columns.

In the first pattern, the reflection is across a vertical line between two
columns; arrows on each of the two columns point at the line between the columns:

123456789
    ><
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    ><
123456789

In this pattern, the line of reflection is the vertical line between
columns 5 and 6. Because the vertical line is not perfectly in the
middle of the pattern, part of the pattern (column 1) has nowhere to
reflect onto and can be ignored; every other column has a reflected
column within the pattern and must match exactly: column 2 matches
column 9, column 3 matches 8, 4 matches 7, and 5 matches 6.

The second pattern reflects across a horizontal line instead:

1 #...##..# 1
2 #....#..# 2
3 ..##..### 3
4v#####.##.v4
5^#####.##.^5
6 ..##..### 6
7 #....#..# 7

This pattern reflects across the horizontal line between rows 4 and 5.
Row 1 would reflect with a hypothetical row 8, but since that's not in
the pattern, row 1 doesn't need to match anything. The remaining rows
match: row 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.

To summarize your pattern notes, add up the number of columns to the
left of each vertical line of reflection; to that, also add 100 multiplied
by the number of rows above each horizontal line of reflection.
In the above example, the first pattern's vertical line has 5 columns
to its left and the second pattern's horizontal line has 4 rows
above it, a total of 405.

Find the line of reflection in each of the patterns in your notes.
What number do you get after summarizing all of your notes?
*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Terrain {
    Rock,
    Ash,
}

#[derive(Debug, Clone, Copy)]
enum ReflectionType {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy)]
struct Reflection {
    reflection_type: ReflectionType,
    between: (usize, usize),
}

impl Reflection {
    fn new(rows: &Vec<Vec<Terrain>>, columns: &Vec<Vec<Terrain>>) -> Reflection {
        let (is_horizontal, between) = Reflection::get_reflection_point(rows);
        if is_horizontal {
            return Reflection {
                reflection_type: ReflectionType::Horizontal,
                between,
            };
        }
        let (is_vertical, between) = Reflection::get_reflection_point(columns);
        if is_vertical {
            return Reflection {
                reflection_type: ReflectionType::Vertical,
                between,
            };
        }

        dbg!(rows, columns);
        panic!("Should have reflection")
    }

    fn get_reflection_point(terrains: &Vec<Vec<Terrain>>) -> (bool, (usize, usize)) {
        let mut has_reflection = false;
        let mut first = 0;
        let mut second = 0;
        for (i, terrain) in terrains.windows(2).enumerate() {
            match &terrain {
                &[t1, t2] => {
                    if t1 == t2 {
                        first = i;
                        second = i + 1;
                        has_reflection = Reflection::full_test(terrains, (first, second));
                        if has_reflection {
                            return (has_reflection, (first, second));
                        }
                    }
                }
                _ => panic!("not good"),
            }
        }

        (has_reflection, (first, second))
    }

    fn full_test(terrains: &Vec<Vec<Terrain>>, (first, second): (usize, usize)) -> bool {
        let first_len = first;
        let second_len = terrains.len() - second - 1;
        let min = first_len.min(second_len);
        if min == 0 {
            return true;
        }
        // dbg!(first, second, first_len, second_len, min);
        for i in 0..min {
            let first_terrains = &terrains[first - (i + 1)];
            let second_terrains = &terrains[second + i + 1];
            // dbg!(first_terrains, second_terrains);
            if first_terrains != second_terrains {
                return false;
            }
        }

        true
    }
}

#[derive(Debug, Clone)]
struct Area {
    rows: Vec<Vec<Terrain>>,
    columns: Vec<Vec<Terrain>>,
    reflection: Reflection,
}

impl Area {
    fn new(lines: &str) -> Area {
        let rows: Vec<Vec<_>> = lines
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Terrain::Ash,
                        '#' => Terrain::Rock,
                        _ => unreachable!("Bad input {c}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let mut columns = vec![];

        for (_y, row) in rows.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                if columns.len() == x {
                    columns.push(vec![]);
                }
                columns[x].push(c);
            }
        }

        let reflection = Reflection::new(&rows, &columns);

        // dbg!(reflection);

        Area {
            rows,
            columns,
            reflection,
        }
    }

    /**
     * To summarize your pattern notes, add up the number of columns to the
     *  left of each vertical line of reflection; to that, also add 100 multiplied
     *  by the number of rows above each horizontal line of reflection.
     *  In the above example, the first pattern's vertical line has 5 columns
     *  to its left and the second pattern's horizontal line has 4 rows
     *  above it, a total of 405.
     */
    fn get_score(&self) -> usize {
        let (_first, second) = self.reflection.between;
        let score = second;

        match self.reflection.reflection_type {
            ReflectionType::Horizontal => score * 100,
            ReflectionType::Vertical => score,
        }
    }
}

fn process(input: &str) -> String {
    let scores = input
        .split("\n\n")
        .map(|lines| {
            let area = Area::new(lines);
            area.get_score()
        })
        .sum::<usize>();

    return scores.to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#",
        );
        assert_eq!(result, "405".to_string());
    }
}
