fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 12: Hot Springs ---

You finally reach the hot springs! You can see steam rising from secluded
areas attached to the primary, ornate building.

As you turn to enter, the researcher stops you. "Wait - I thought you were
looking for the hot springs, weren't you?" You indicate that this definitely looks like hot springs to you.

"Oh, sorry, common mistake! This is actually the onsen! The hot springs are next door."

You look in the direction the researcher is pointing and suddenly notice the
massive metal helixes towering overhead. "This way!"

It only takes you a few more steps to reach the main gate of the massive fenced-off
area containing the springs. You go through the gate and into a small administrative building.

"Hello! What brings you to the hot springs today? Sorry they're not very hot right now;
we're having a lava shortage at the moment." You ask about the missing machine parts for Desert Island.

"Oh, all of Gear Island is currently offline! Nothing is being manufactured at the moment,
not until we get more lava to heat our forges. And our springs. The springs aren't very springy unless they're hot!"

"Say, could you go up and see why the lava stopped flowing? The springs are too cold
for normal operation, but we should be able to find one springy enough to launch you up there!"

There's just one problem - many of the springs have fallen into disrepair, so they're
not actually sure which springs would even be safe to use! Worse yet, their condition records
of which springs are damaged (your puzzle input) are also damaged! You'll need to help them repair the damaged records.

In the giant field just outside, the springs are arranged into rows. For each row, the
condition records show every spring and whether it is operational (.) or damaged (#).
This is the part of the condition records that is itself damaged; for some springs, it is simply unknown (?) whether the spring is operational or damaged.

However, the engineer that produced the condition records also duplicated some of this
information in a different format! After the list of springs for a given row, the size
of each contiguous group of damaged springs is listed in the order those groups appear
in the row. This list always accounts for every damaged spring, and each number is the
entire size of its contiguous group (that is, groups are always separated by at least
    one operational spring: #### would always be 4, never 2,2).

So, condition records with no unknown spring conditions might look like this:

#.#.### 1,1,3
.#...#....###. 1,1,3
.#.###.#.###### 1,3,1,6
####.#...#... 4,1,1
#....######..#####. 1,6,5
.###.##....# 3,2,1

However, the condition records are partially damaged; some of the springs' conditions are
actually unknown (?). For example:

???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1

Equipped with this information, it is your job to figure out how many different arrangements of
operational and broken springs fit the given criteria in each row.

In the first line (???.### 1,1,3), there is exactly one way separate groups of one, one, and
three broken springs (in that order) can appear in that row: the first three unknown springs must be broken, then operational, then broken (#.#), making the whole row #.#.###.

The second line is more interesting: .??..??...?##. 1,1,3 could be a total of four different
arrangements. The last ? must always be broken (to satisfy the final contiguous group of three broken springs),
and each ?? must hide exactly one of the two broken springs. (Neither ?? could be both broken springs or they
    would form a single contiguous group of two; if that were true, the numbers afterward would have been 2,3 instead.)
    Since each ?? can either be #. or .#, there are four possible arrangements of springs.

The last line is actually consistent with ten different arrangements! Because the first number is 3,
the first and second ? must both be . (if either were #, the first number would have to be 4 or higher).
However, the remaining run of unknown spring conditions have many different ways they could hold groups of two and one broken springs:

?###???????? 3,2,1
.###.##.#...
.###.##..#..
.###.##...#.
.###.##....#
.###..##.#..
.###..##..#.
.###..##...#
.###...##.#.
.###...##..#
.###....##.#

In this example, the number of possible arrangements for each row is:

    ???.### 1,1,3 - 1 arrangement
    .??..??...?##. 1,1,3 - 4 arrangements
    ?#?#?#?#?#?#?#? 1,3,1,6 - 1 arrangement
    ????.#...#... 4,1,1 - 1 arrangement
    ????.######..#####. 1,6,5 - 4 arrangements
    ?###???????? 3,2,1 - 10 arrangements

Adding all of the possible arrangement counts together produces a total of 21 arrangements.

For each row, count all of the different arrangements of operational and broken springs that meet the given criteria.
What is the sum of those counts?

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
        springs,
        broken_groups,
    }
}

fn process(input: &str) -> String {
    let rows = input.lines().map(get_row);

    // dbg!(rows.map(get_count).collect::<Vec<_>>());

    return rows.map(get_count).sum::<usize>().to_string();
}

fn get_spring_groups(springs: &Vec<Spring>, check_spring: Spring) -> Vec<Vec<Spring>> {
    let mut cur_groups: Vec<Vec<Spring>> = vec![];
    let mut temp_group = vec![];
    for &spring in springs {
        if spring == check_spring {
            if let Some(&last) = temp_group.last() {
                if spring == last {
                    temp_group.push(spring);
                } else {
                    cur_groups.push(temp_group.clone());
                    temp_group.clear();
                    temp_group.push(spring);
                }
            } else {
                temp_group.push(spring);
            }
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

fn satisfies(check_springs: &Vec<Spring>, broken_groups: &Vec<usize>) -> bool {
    let springs: Vec<Vec<Spring>> = get_spring_groups(check_springs, Spring::Broken);
    // dbg!(springs.len(), broken_groups.len());
    if springs.len() != broken_groups.len() {
        return false;
    }

    // dbg!(&springs);

    springs
        .iter()
        .zip(broken_groups)
        .all(|(spring_group, &count)| spring_group.len() == count)
}

fn traverse(springs: &Vec<Spring>, groups: &Vec<usize>, count: usize) -> usize {
    // dbg!(springs);

    if satisfies(springs, groups) {
        count + 1
    } else {
        for (i, &spring) in springs.iter().enumerate() {
            if spring == Spring::Unknown {
                let mut operational_springs = springs.clone();
                operational_springs[i] = Spring::Operational;
                let mut broken_springs = springs.clone();
                broken_springs[i] = Spring::Broken;

                let count_operational = traverse(&operational_springs, groups, 0);
                let count_broken = traverse(&broken_springs, groups, 0);
                return count + count_operational + count_broken;
            }
        }
        count
    }
}

fn get_count(row: Row) -> usize {
    traverse(&row.springs, &row.broken_groups, 0)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    #[test]
    fn get_count_works(#[case] input: &str, #[case] output: usize) {
        let result = get_count(get_row(input));
        assert_eq!(result, output);
    }

        #[test]
        fn it_works() {
            let result = process(
                "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1",
            );
            assert_eq!(result, "21".to_string());
        }
}
