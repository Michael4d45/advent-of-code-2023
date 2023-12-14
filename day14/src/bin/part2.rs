use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Part Two ---

The parabolic reflector dish deforms, but not in a
way that focuses the beam. To do that, you'll need
to move the rocks to the edges of the platform.
Fortunately, a button on the side of the control
panel labeled "spin cycle" attempts to do just that!

Each cycle tilts the platform four times so that the
rounded rocks roll north, then west, then south, then east.
After each tilt, the rounded rocks roll as far as they can
before the platform tilts in the next direction. After one
cycle, the platform will have finished rolling the rounded
rocks in those four directions in that order.

Here's what happens in the example above after each of the
first few cycles:

After 1 cycle:
.....#.... 1
....#...O# 2
...OO##... 3
.OO#...... 4
.....OOO#. 5
.O#...O#.# 6
....O#.... 7
......OOOO 8
#...O###.. 9
#..OO#.... 10
12345678911
         01

After 2 cycles:
.....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O

After 3 cycles:
.....#.... 1
....#...O# 2
.....##... 3
..O#...... 4
.....OOO#. 5
.O#...O#.# 6
....O#...O 7
.......OOO 8
#...O###.O 9
#.OOO#...O 10
12345678911
         01

This process should work if you leave it running long enough,
but you're still worried about the north support beams.
To make sure they'll survive for a while, you need to calculate
the total load on the north support beams after 1000000000 cycles.

In the above example, after 1000000000 cycles, the total load
on the north support beams is 64.

Run the spin cycle for 1000000000 cycles. Afterward, what is the
total load on the north support beams?


*/
#[derive(Debug, Clone, Copy)]
struct Pos {
    north_square_y: Option<usize>,
    south_square_y: Option<usize>,
    east_square_x: Option<usize>,
    west_square_x: Option<usize>,
    is_square: bool,
}

impl Pos {
    fn new(is_square: bool) -> Pos {
        Pos {
            north_square_y: None,
            south_square_y: None,
            east_square_x: None,
            west_square_x: None,
            is_square,
        }
    }
}

fn process(input: &str) -> String {
    let size_y = input.lines().count();
    let size_x = input.lines().next().unwrap().len();
    let mut matrix = BTreeSet::new();
    let mut squares: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut last_square_at_x = BTreeMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.char_indices() {
            matrix.insert((x, y));
            if c == '#' {
                squares.insert((x, y), 0);

                last_square_at_x.insert(x, y);
            };
            if c == 'O' {
                let &last_y = last_square_at_x.get(&x).expect("I set all the rocks");
                let mut square = squares.remove(&(x, last_y)).expect("Should have value.");
                square += 1;
                squares.insert((x, last_y), square);
            }
        }
    }
    let mut nearest_squares = BTreeMap::new();
    for &(x, y) in matrix.iter() {
        let mut pos = Pos::new(squares.contains_key(&(x, y)));

        // get north
        for i in 1..y + 1 {
            if squares.contains_key(&(x, y - i)) {
                pos.north_square_y = Some(y - i);
                break;
            }
        }
        for i in 1..size_y - y {
            if squares.contains_key(&(x, y + i)) {
                pos.south_square_y = Some(y + i);
                break;
            }
        }
        for i in 1..x + 1 {
            if squares.contains_key(&(x - i, y)) {
                pos.west_square_x = Some(x - i);
                break;
            }
        }
        for i in 1..size_x - x {
            if squares.contains_key(&(x + i, y)) {
                pos.east_square_x = Some(x + i);
                break;
            }
        }

        nearest_squares.insert((x, y), pos);
    }

    // north, then west, then south, then east.

    let mut zeroed_squares = BTreeMap::new();
    for &key in squares.keys() {
        zeroed_squares.insert(key, 0);
    }

    let mut temp_squares = zeroed_squares.clone();
    // west
    for (&(x, y), &count) in &squares {
        for i in 1..=count {
            let p = nearest_squares.get(&(x, y + i)).expect("Why no P?");
            let west_x = p.west_square_x.expect("No west x??");
            let t_count = temp_squares.entry((west_x, y + i)).or_insert(0);
            *t_count += 1;
        }
    }
    squares = temp_squares.clone();

    temp_squares = zeroed_squares.clone();
    // south
    for (&(x, y), &count) in &squares {
        for i in 1..=count {
            let p = nearest_squares.get(&(x + i, y)).expect("Why no P?");
            let south_y = p.south_square_y.expect("No south y??");
            let t_count = temp_squares.entry((x + i, south_y)).or_insert(0);
            *t_count += 1;
        }
    }
    squares = temp_squares.clone();

    temp_squares = zeroed_squares.clone();
    // east
    for (&(x, y), &count) in &squares {
        for i in 1..=count {
            let p = nearest_squares.get(&(x, y - i)).expect("Why no P?");
            let east_x = p.east_square_x.expect("No east x??");
            let t_count = temp_squares.entry((east_x, y - i)).or_insert(0);
            *t_count += 1;
        }
    }
    squares = temp_squares.clone();

    // dbg!(squares
    //     .iter()
    //     .filter_map(|((x, y), &value)| (value > 0).then_some((x, y, value)))
    //     .collect::<Vec<_>>());

    let mut cc = 1;
    // temp_squares = zeroed_squares.clone();

    // finished one cycle

    let mut map = BTreeMap::new();
    let mut inverted = BTreeMap::new();

    let mut holding_stones: Vec<(usize, usize, usize)> = squares
        .iter()
        .filter_map(|(&(x, y), &value)| (value > 0).then_some((x, y, value)))
        .collect::<Vec<_>>();

    while !map.contains_key(&holding_stones) {
        // for _ in 0..100 {
        map.insert(holding_stones.clone(), cc);
        inverted.insert(cc, holding_stones);
        // north, then west, then south, then east.
        // north
        let mut temp_squares = BTreeMap::new();
        for (&(x, y), &count) in &squares {
            for i in 1..=count {
                let p = nearest_squares.get(&(x - i, y)).expect("Why no P?");
                let north_y = p.north_square_y.expect("No north y??");
                let t_count = temp_squares.entry((x - i, north_y)).or_insert(0);
                *t_count += 1;
            }
        }
        squares = temp_squares.clone();

        temp_squares = zeroed_squares.clone();
        // west
        // println!("West");
        for (&(x, y), &count) in &squares {
            for i in 1..=count {
                let p = nearest_squares.get(&(x, y + i)).expect("Why no P?");
                let west_x = p.west_square_x.expect("No west x??");
                let t_count = temp_squares.entry((west_x, y + i)).or_insert(0);
                *t_count += 1;
            }
        }
        squares = temp_squares.clone();

        temp_squares = zeroed_squares.clone();
        // south
        // println!("South");
        for (&(x, y), &count) in &squares {
            for i in 1..=count {
                let p = nearest_squares.get(&(x + i, y)).expect("Why no P?");
                let south_y = p.south_square_y.expect("No south y??");
                let t_count = temp_squares.entry((x + i, south_y)).or_insert(0);
                *t_count += 1;
            }
        }
        squares = temp_squares.clone();

        temp_squares = zeroed_squares.clone();
        // east
        // println!("East");
        for (&(x, y), &count) in &squares {
            for i in 1..=count {
                let p = nearest_squares.get(&(x, y - i)).expect("Why no P?");
                let east_x = p.east_square_x.expect("No east x??");
                let t_count = temp_squares.entry((east_x, y - i)).or_insert(0);
                *t_count += 1;
            }
        }
        squares = temp_squares.clone();

        // dbg!(squares
        //     .iter()
        //     .filter_map(|((x, y), &value)| (value > 0).then_some((x, y, value)))
        //     .collect::<Vec<_>>());
        holding_stones = squares
            .iter()
            .filter_map(|(&(x, y), &value)| (value > 0).then_some((x, y, value)))
            .collect::<Vec<_>>();
        cc += 1;
    }
    let old = map.get(&holding_stones).unwrap();

    // dbg!(squares
    //     .iter()
    //     .filter_map(|((x, y), &value)| (value > 0).then_some((x, y, value)))
    //     .collect::<Vec<_>>());

    // dbg!(cc, map.len(), old, holding_stones);
    dbg!(squares.len());

    let l = cc - old;

    let target = 1_000_000_000;
    let pos = (target - old) % l + old;
    let stones = inverted.get(&pos).unwrap();
    // dbg!(target, l, pos, target % l, stones);

    // let circles = squares
    //     .iter()
    //     .filter_map(|((_, y), (_, value))| (!value.is_empty()).then_some((count - y, value)));

    let total = size_y - 1;
    let maths = stones.iter().map(|&(_x, y, count)| {
        // dbg!(total, y, count);
        (total - y) * count
    });

    // dbg!(maths.sum::<usize>(), cc);
    // for (cc, stones) in inverted {
    //     let maths = stones.iter().map(|&(_x, y, count)| {
    //         // dbg!(total, y, count);
    //         (total - y) * count
    //     });

    //     dbg!(maths.sum::<usize>(), cc);
    // }

    return maths.sum::<usize>().to_string();
    // 0.to_string()
}

#[cfg(test)]
mod tests {
    use crate::process;

    // #[test]
    // fn it_works1() {
    //     let result = process("###\n#O#\n###");
    //     assert_eq!(result, "64".to_string());
    // }

    // #[test]
    // fn it_works2() {
    //     let result = process("####\n#O #\n####");
    //     assert_eq!(result, "64".to_string());
    // }

    // #[test]
    // fn it_works3() {
    //     let result = process("####\n#  #\n#O #\n####");
    //     assert_eq!(result, "64".to_string());
    // }
    #[test]
    fn it_works() {
        let result = process(
            "############
#O....#....#
#O.OO#....##
#.....##...#
#OO.#O....O#
#.O.....O#.#
#O.#..O.#.##
#..O..#O..O#
#.......O..#
##....###..#
##OO..#....#
############",
        );
        assert_eq!(result, "64".to_string());
    }
}
