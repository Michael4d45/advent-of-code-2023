use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

/**
--- Part Two ---

The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take significantly more steps to escape!

What if the map isn't for people - what if the map is for ghosts? Are ghosts even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in A is equal to the number ending in Z!
 If you were a ghost, you'd probably just start at every node that ends with A and follow all of the paths at the same time until they all simultaneously end up at nodes that end with Z.

For example:

LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)





Here, there are two starting nodes, 11A and 22A (because they both end with A). As you follow each left/right instruction,
use that instruction to simultaneously navigate away from both nodes you're currently on. Repeat this process until all of the
nodes you're currently on end with Z. (If only some of the nodes you're on end with Z, they act like any other node and you continue as normal.)
In this example, you would proceed as follows:

    Step 0: You are at 11A and 22A.
    Step 1: You choose all of the left paths, leading you to 11B and 22B.
    Step 2: You choose all of the right paths, leading you to 11Z and 22C.
    Step 3: You choose all of the left paths, leading you to 11B and 22Z.
    Step 4: You choose all of the right paths, leading you to 11Z and 22B.
    Step 5: You choose all of the left paths, leading you to 11B and 22C.
    Step 6: You choose all of the right paths, leading you to 11Z and 22Z.

So, in this example, you end up entirely on nodes that end in Z after 6 steps.

Simultaneously start on every node that ends with A. How many steps does it take before you're only on nodes that end with Z?


*/
#[derive(Debug, Copy, Clone)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
    is_first: bool,
    is_last: bool,
}

fn get_mapping(line: &str) -> Node {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let name = parts[0];
    let left = &parts[2][1..parts[2].len() - 1];
    let right = &parts[3][0..parts[3].len() - 1];

    let last_letter = name.chars().last().unwrap();
    return Node {
        name,
        left,
        right,
        is_first: last_letter == 'A',
        is_last: last_letter == 'Z',
    };
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn calculate_lcm(cycle_lengths: &[usize]) -> usize {
    if cycle_lengths.is_empty() {
        0
    } else {
        cycle_lengths.iter().fold(1, |acc, &x| lcm(acc, x))
    }
}

/** from chatGPT
If the cycles not only have different lengths but also start at different positions and have different start times, determining when they will all be in a specific relative position (e.g., aligned) can be more complex.
In this case, you need to consider both the cycle lengths and the initial offsets.

The general approach is to find the least common multiple (LCM) of the cycle lengths, as previously mentioned. However, you also need to take into account the initial offsets.

Let L1,L2,L3,…,LnL1​,L2​,L3​,…,Ln​ be the cycle lengths, and O1,O2,O3,…,OnO1​,O2​,O3​,…,On​ be the initial offsets (the positions at the start time).

The time it takes for all cycles to be in the same relative position is given by the following formula:

LCM(L1,L2,L3,…,Ln)−max(O1,O2,O3,…,On)LCM(L1​,L2​,L3​,…,Ln​)−max(O1​,O2​,O3​,…,On​)

This accounts for both the cycle lengths and the initial offsets. The subtraction ensures that you consider the relative position after taking into account the initial offsets.

To summarize, find the LCM of the cycle lengths, and subtract the maximum initial offset. This will give you the time at which all cycles will be in the specified relative position.

*/
fn find_alignment_time(
    cycle_starts: &[usize],
    all_z_offsets: &[usize],
    cycle_lens: &[usize],
) -> usize {
    // Adjust for the cycle starts
    let cycle_start_offset = cycle_starts.iter().cloned().max().unwrap_or(0);

    // Find the maximum and minimum initial offsets
    let max_offset = *all_z_offsets
        .iter()
        .max()
        .unwrap_or(&0);
    // let min_offset = *all_z_offsets
    //     .iter()
    //     .min()
    //     .unwrap_or(&0);

    // Calculate the least common multiple of cycle lengths
    let lcm_cycle_lengths = cycle_lens.iter().fold(1, |acc, &len| lcm(acc, len));

    // Calculate the time when all cycles will be in the same relative position
    let alignment_time = lcm_cycle_lengths - max_offset;

    dbg!(
        max_offset,
        cycle_start_offset,
        lcm_cycle_lengths
    );

    // turns out the offsets here cancel out... I can clean this up more, but I want to make sure 
    alignment_time + cycle_start_offset
}

fn part2(input: &str) -> String {
    let mut lines = input.lines();
    let route = lines.next().expect("should have first line");
    lines.next(); // skip empty line

    let nodes = lines.fold(HashMap::new(), |mut acc, line| {
        let node = get_mapping(line);
        if node.is_last || node.is_first {
            dbg!(&node);
        }
        acc.insert(node.name, node);
        acc
    });

    let route_chars = route.chars().collect::<Vec<char>>();

    let traversing_nodes = nodes
        .values()
        .filter_map(|node| node.is_first.then_some(node))
        .collect::<Vec<&Node>>();
    let mut count;
    let mut node;

    let mut counts = vec![];
    let mut visited = HashSet::new();
    let mut cycles_on;
    let mut visiting;
    let mut counts2 = vec![];
    let mut all_z_offsets = vec![];
    let mut z_offsets = vec![];
    let mut c_start;
    for i in 0..traversing_nodes.len() {
        z_offsets.clear();
        visited.clear();
        dbg!(i);
        node = traversing_nodes[i];
        // dbg!(node.name);
        count = 0;
        visiting = format!("{}{}", node.name, count % route_chars.len());
        while visited.insert(visiting.clone()) {
            node = nodes
                .get(match route_chars[count % route_chars.len()] {
                    'L' => node.left,
                    'R' => node.right,
                    _ => panic!(
                        "should only be L or R, got {}",
                        route_chars[count % route_chars.len()]
                    ),
                })
                .expect("should get node");
            // dbg!(node.name);
            count += 1;
            visiting = format!("{}{}", node.name, count % route_chars.len());
        }
        cycles_on = visiting.clone();
        // dbg!(visiting.clone());
        counts.push(count);

        c_start = count;

        node = nodes
            .get(match route_chars[count % route_chars.len()] {
                'L' => node.left,
                'R' => node.right,
                _ => panic!(
                    "should only be L or R, got {}",
                    route_chars[count % route_chars.len()]
                ),
            })
            .expect("should get node");
        count += 1;
        visiting = format!("{}{}", node.name, count % route_chars.len());
        // dbg!(node.name);

        while visiting != cycles_on {
            if node.is_last {
                println!("hit last: {}, count: {}", node.name, count);
                z_offsets.push(count - c_start);
            }
            node = nodes
                .get(match route_chars[count % route_chars.len()] {
                    'L' => node.left,
                    'R' => node.right,
                    _ => panic!(
                        "should only be L or R, got {}",
                        route_chars[count % route_chars.len()]
                    ),
                })
                .expect("should get node");
            // dbg!(node.name);
            count += 1;
            visiting = format!("{}{}", node.name, count % route_chars.len());
        }
        counts2.push(count);
        all_z_offsets.push(z_offsets.clone());
    }

    dbg!(&counts);
    dbg!(&counts2);
    let cycle_lens: Vec<usize> = counts
        .iter()
        .zip(counts2)
        .map(|(x1, x2)| x2 - x1)
        .collect::<Vec<_>>();
    let new_z_pos = all_z_offsets
        .iter()
        .map(|list| *list.iter().max().unwrap())
        .zip(&cycle_lens)
        .map(|(x1, x2)| x2 - x1)
        .collect::<Vec<_>>();
    dbg!(&new_z_pos);
    let cycle_starts = cycle_lens
        .iter()
        .zip(counts)
        .map(|(x1, x2)| x2 - x1)
        .collect::<Vec<_>>();
    dbg!(&cycle_starts);
    dbg!(&all_z_offsets);
    dbg!(&cycle_lens);
    let lcms = calculate_lcm(&cycle_lens);
    dbg!(lcms);
    let result = find_alignment_time(&cycle_starts, &new_z_pos, &cycle_lens);
    dbg!(result);

    return result.to_string();
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn it_works() {
        let result = part2(
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)",
        );
        assert_eq!(result, "6".to_string());
    }
}
