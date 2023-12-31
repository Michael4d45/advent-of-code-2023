use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    // dbg!(output);
}

/**
--- Part Two ---

As you look out at the field of springs, you feel like there are way more springs than the condition records list.
 When you examine the records, you discover that they were actually folded up this whole time!

To unfold the records, on each row, replace the list of spring conditions with five copies of itself (separated by ?)
and replace the list of contiguous groups of damaged springs with five copies of itself (separated by ,).

So, this row:

.# 1

Would become:

.#?.#?.#?.#?.# 1,1,1,1,1

The first line of the above example would become:

???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3

In the above example, after unfolding, the number of possible arrangements for some rows is now much larger:

    ???.### 1,1,3 - 1 arrangement
    .??..??...?##. 1,1,3 - 16384 arrangements
    ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    ????.#...#... 4,1,1 - 16 arrangements
    ????.######..#####. 1,6,5 - 2500 arrangements
    ?###???????? 3,2,1 - 506250 arrangements

After unfolding, adding all of the possible arrangement counts together produces 525152.

Unfold your condition records; what is the new sum of possible arrangement counts?

*/
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    broken_groups: Vec<usize>,
}

fn get_spring(c: char) -> Spring {
    use Spring::*;

    match c {
        '?' => Unknown,
        '#' => Broken,
        '.' => Operational,
        _ => unreachable!("This was bad input {}", c),
    }
}

fn multiply_spring(springs: Vec<Spring>) -> Vec<Spring> {
    let spacer = Spring::Unknown;
    vec![
        springs.clone(),
        vec![spacer],
        springs.clone(),
        vec![spacer],
        springs.clone(),
        vec![spacer],
        springs.clone(),
        vec![spacer],
        springs.clone(),
    ]
    .concat()
}

fn multiply_groups(groups: Vec<usize>) -> Vec<usize> {
    vec![
        groups.clone(),
        groups.clone(),
        groups.clone(),
        groups.clone(),
        groups.clone(),
    ]
    .concat()
}

fn get_row(input: &str) -> Row {
    let temp: Vec<_> = input.split(" ").collect();
    let temp_springs = temp[0];
    let temp_groups = temp[1];

    let springs = temp_springs.chars().map(get_spring).collect::<Vec<_>>();
    let broken_groups = temp_groups
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    Row {
        // springs: multiply_spring(springs),
        // broken_groups: multiply_groups(broken_groups),
        springs,
        broken_groups,
    }
}

fn process(input: &str) -> String {
    let rows = input.lines().map(get_row);

    // dbg!(rows.map(get_count).collect::<Vec<_>>());

    return rows.map(get_count).sum::<usize>().to_string();
}

fn get_spring_groups(springs: &Vec<Spring>) -> Vec<Vec<(usize, Spring)>> {
    let mut cur_groups = vec![];
    let mut temp_group = vec![];
    for (i, &spring) in springs.iter().enumerate() {
        if spring == Spring::Broken || spring == Spring::Unknown {
            temp_group.push((i, spring));
        } else if !temp_group.is_empty() {
            cur_groups.push(temp_group.clone());
            temp_group.clear();
        }
    }
    if !temp_group.is_empty() {
        cur_groups.push(temp_group);
    }

    cur_groups
}

// fn get_broken_spring_groups(springs: &Vec<Spring>) -> Vec<Vec<Spring>> {
//     let mut cur_groups: Vec<Vec<Spring>> = vec![];
//     let mut temp_group = vec![];
//     for &spring in springs {
//         if spring == Spring::Broken {
//             temp_group.push(spring);
//         } else if !temp_group.is_empty() {
//             cur_groups.push(temp_group.clone());
//             temp_group.clear();
//         }
//     }
//     if !temp_group.is_empty() {
//         cur_groups.push(temp_group);
//     }

//     cur_groups
// }

// fn satisfies(broken_groups: &Vec<usize>, springs: Vec<Vec<Spring>>) -> bool {
//     // dbg!(springs.len(), broken_groups.len());
//     if springs.len() != broken_groups.len() {
//         return false;
//     }

//     // dbg!(&springs);

//     springs
//         .iter()
//         .zip(broken_groups)
//         .all(|(spring_group, &count)| spring_group.len() == count)
// }

fn get_counts(springs: &Vec<(usize, Spring)>) -> (usize, usize, usize) {
    springs.iter().fold(
        (0, 0, 0),
        |(operational, broken, unknown), &(_, spring)| match spring {
            Spring::Operational => (operational + 1, broken, unknown),
            Spring::Broken => (operational, broken + 1, unknown),
            Spring::Unknown => (operational, broken, unknown + 1),
        },
    )
}

fn get_count(row: Row) -> usize {
    

    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    // #[case("???.### 1,1,3", 1)]
    // #[case(".??..??...?##. 1,1,3", 4)]
    // #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    // #[case("????.#...#... 4,1,1", 1)]
    // #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[test]
    fn get_count_works(#[case] input: &str, #[case] output: usize) {
        let result = get_count(get_row(input));
        assert_eq!(result, output);
    }

    //     #[test]
    //     fn it_works() {
    //         let result = process(
    //             "???.### 1,1,3
    // .??..??...?##. 1,1,3
    // ?#?#?#?#?#?#?#? 1,3,1,6
    // ????.#...#... 4,1,1
    // ????.######..#####. 1,6,5
    // ?###???????? 3,2,1",
    //         );
    //         assert_eq!(result, "525152".to_string());
    //     }
}
