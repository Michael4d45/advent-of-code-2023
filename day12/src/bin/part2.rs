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
        springs: multiply_spring(springs),
        broken_groups: multiply_groups(broken_groups),
        // springs,
        // broken_groups,
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
    // let have_total = get_counts(&row.springs);
    // dbg!(have_total);

    let groups = get_spring_groups(&row.springs);

    // dbg!(&groups, &groups.len(), row.broken_groups.len());

    let mut all_test_groups = vec![groups.clone()];
    let mut finished_test_group = HashMap::new();
    let mut changed = true;
    while changed {
        changed = false;
        while let Some(groups) = all_test_groups.pop() {
            // dbg!(all_test_groups.len());
            let group_diff = row.broken_groups.len() - groups.len();
            if group_diff > 0 {
                for (i, springs) in groups.iter().enumerate() {
                    let (first_part_of_group, after) = groups.split_at(i);
                    let second_part_of_group = &after[1..];
                    // dbg!(first_part_of_group, second_part_of_group);
                    let count = row.broken_groups[i];
                    // dbg!(count, springs.len());
                    let poses = springs
                        .iter()
                        .enumerate()
                        .filter_map(|(pos, &(_, spring))| {
                            (spring == Spring::Unknown).then_some(pos)
                        })
                        .collect::<Vec<_>>();
                    if poses.len() > 0 && springs.len() > count {
                        let mut next_count = springs.len();
                        if i + 1 < row.broken_groups.len() {
                            // dbg!(next_count, row.broken_groups[i + 1]);
                            if row.broken_groups[i + 1] > next_count {
                                continue;
                            }
                            next_count -= row.broken_groups[i + 1];
                        }
                        for j in count..next_count {
                            if poses.contains(&j) {
                                let (before, after) = springs.split_at(j);
                                let before = before.iter().map(|&s| s).collect::<Vec<_>>();
                                let after = after[1..].iter().map(|&s| s).collect::<Vec<_>>();
                                let pushing: Vec<Vec<(usize, Spring)>> = [
                                    first_part_of_group,
                                    &[before.clone()],
                                    &[after.clone()],
                                    second_part_of_group,
                                ]
                                .concat();
                                if pushing[pushing.len() - 1].len()
                                    >= *row.broken_groups.last().unwrap()
                                {
                                    if pushing.len() == row.broken_groups.len() {
                                        let poses: Vec<Vec<usize>> = pushing
                                            .iter()
                                            .map(|v| v.iter().map(|&(i, _)| i).collect::<Vec<_>>())
                                            .collect::<Vec<_>>();
                                        // dbg!(&poses);
                                        finished_test_group.insert(poses, pushing);
                                    } else {
                                        all_test_groups.push(pushing);
                                    }
                                }
                                let (_, before_broken, _) = get_counts(&before);
                                // let (_, after_broken, _) = get_counts(&after);
                                if before_broken >= count {
                                    break;
                                }
                            }
                        }
                    }
                }
                changed = true;
            } else {
                // panic!("wat here?");
                finished_test_group.insert(vec![], groups.clone());
            }
        }
    }
    // dbg!(&finished_test_group.len());

    println!("");

    let mut all_total = 0;
    for (_, groups) in finished_test_group {
        let mut total = 1;
        for (springs, &count) in groups.iter().zip(&row.broken_groups) {
            let (_, broken, unknown) = get_counts(&springs);

            if unknown == 0 && broken == count {
                continue;
            }

            if broken > 0 {
                let mut first = 0;
                for i in 0..springs.len() {
                    if springs[i].1 == Spring::Broken {
                        first = i;
                        break;
                    }
                }
                let mut last = first;
                for i in 0..springs.len() {
                    last = springs.len() - i - 1;
                    if springs[last].1 == Spring::Broken {
                        break;
                    }
                }

                let left_size = first;
                let right_size = springs.len() - last - 1;
                let min = left_size.min(right_size);

                if min * 2 >= count {
                    // dbg!(&groups, count, last, first);
                    total *= count - (last - first);
                } else {
                    total *= 1 + min;
                }
            } else {
                total *= 1 + springs.len() - count
            }

            // dbg!(springs.len(), count, total);
        }
        // dbg!(total);
        all_total += total;
    }

    dbg!(all_total);

    // for group in groups {
    //     if group.contains(&Spring::Unknown) {
    //       // dbg!(&group);
    //         for (i, window) in group.windows(group.len() - 1).enumerate() {
    //           // dbg!(i, &window);
    //         }
    //     }
    // }

    // let count = traverse();

    // let mut other_springs = row.springs.clone();
    // other_springs.push(Spring::Unknown);
    // let count_other = traverse(0, &other_springs, &row.broken_groups, 0);

    // dbg!(count_other.pow(4) * count);

    // dbg!(count, count_other);

    all_total
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
