fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Part Two ---

You resume walking through the valley of mirrors and - SMACK! -
run directly into one. Hopefully nobody was watching, because that
must have been pretty embarrassing.

Upon closer inspection, you discover that every mirror has exactly
one smudge: exactly one . or # should be the opposite type.

In each pattern, you'll need to locate and fix the smudge that causes a
different reflection line to be valid. (The old reflection line won't
necessarily continue being valid after the smudge is fixed.)

Here's the above example again:

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

The first pattern's smudge is in the top-left corner. If the
top-left # were instead ., it would have a different, horizontal
line of reflection:

1 ..##..##. 1
2 ..#.##.#. 2
3v##......#v3
4^##......#^4
5 ..#.##.#. 5
6 ..##..##. 6
7 #.#.##.#. 7

With the smudge in the top-left corner repaired, a new horizontal
line of reflection between rows 3 and 4 now exists. Row 7 has no
corresponding reflected row and can be ignored, but every other
row matches exactly: row 1 matches row 6, row 2 matches row 5,
and row 3 matches row 4.

In the second pattern, the smudge can be fixed by changing the fifth
symbol on row 2 from . to #:

1v#...##..#v1
2^#...##..#^2
3 ..##..### 3
4 #####.##. 4
5 #####.##. 5
6 ..##..### 6
7 #....#..# 7

Now, the pattern has a different horizontal line of reflection between rows 1 and 2.

Summarize your notes as before, but instead use the new different reflection
lines. In this example, the first pattern's new horizontal line has 3 rows
above it and the second pattern's new horizontal line has 1 row above it,
summarizing to the value 400.

In each pattern, fix the smudge and find the different line of reflection.
What number do you get after summarizing the new reflection line in each
pattern in your notes?

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

        // dbg!(rows, columns);
        panic!("Should have reflection")
    }

    fn get_reflection_point(terrains: &Vec<Vec<Terrain>>) -> (bool, (usize, usize)) {
        let mut testing_terrains = terrains.clone();
        let mut potential_reflections = vec![];
        for i in 0..terrains.len() - 1 {
            for j in i + 1..terrains.len() {
                let mut diff = 0;
                let mut pos = 0;
                for (temp_pos, (left, right)) in terrains[i].iter().zip(&terrains[j]).enumerate() {
                    if left != right {
                        diff += 1;
                        pos = temp_pos;
                    }
                }
                // pairs.insert((i, j), diff);
                if diff == 1 && (j - i) % 2 == 1 {
                    let left = i;
                    let right = j;
                    let second = (left + right + 1) / 2;
                    let first = second - 1;
                    let temp_terrain = testing_terrains[left][pos];
                    testing_terrains[left][pos] = testing_terrains[right][pos];
                    // dbg!(pos, temp_terrain, left, right, second, first);

                    if testing_terrains[first] == testing_terrains[second]
                        && Reflection::full_test(&testing_terrains, (first, second))
                    {
                        println!("");
                        for (i, row) in testing_terrains.iter().enumerate() {
                            for terrain in row {
                                match terrain {
                                    Terrain::Rock => print!("#"),
                                    Terrain::Ash => print!("."),
                                }
                            }
                            if i == first {
                                print!("< first");
                            }
                            if i == second {
                                print!("< second");
                            }
                            if i == left {
                                print!("< left");
                            }
                            if i == right {
                                print!("< right");
                            }
                            print!("\n");
                        }
                        potential_reflections.push((first, second, pos));
                    }
                    testing_terrains[left][pos] = temp_terrain;
                }
            }
        }

        if potential_reflections.len() > 1 {
            dbg!(&potential_reflections);
            todo!();
        } else if potential_reflections.len() == 1 {
            let (left, right, _) = potential_reflections[0];
            return (true, (left, right));
        }

        println!("");

        // todo!();
        (false, (0, 0))
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
        dbg!(_first, second);

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
        assert_eq!(result, "4000".to_string());
    }
}
