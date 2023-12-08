use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

/**
--- Day 8: Haunted Wasteland ---

You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf,
she disappears before your eyes! To be fair, she had just finished warning you about ghosts a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert.
At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of network of labeled nodes.

It seems like you're meant to use the left/right instructions to navigate the network.
Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: AAA and ZZZ.
You feel like AAA is where you are now, and you have to follow the left/right instructions until you reach ZZZ.

This format defines each node of the network individually. For example:

RL
      ( L ,  R )
AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)

Starting with AAA, you need to look up the next element based on the next left/right instruction in your input.
In this example, start with AAA and go right (R) by choosing the right element of AAA, CCC. Then, L means to choose the left element of CCC, ZZZ. By following the left/right instructions, you reach ZZZ in 2 steps.

Of course, you might not find ZZZ right away. If you run out of left/right instructions, repeat the whole
sequence of instructions as necessary: RL really means RLRLRLRLRLRLRLRL... and so on. For example, here is a situation that takes 6 steps to reach ZZZ:

LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)

1: L a -> b
2: L b -> a
3: R a -> b

4: L b -> a
5: L a -> b
6: R b -> z

Starting at AAA, follow the left/right instructions. How many steps are required to reach ZZZ?


*/
#[derive(Debug, Copy, Clone)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn get_mapping(line: &str) -> Node {
    let parts = line.split(' ').collect::<Vec<&str>>();
    let name = parts[0];
    let left = &parts[2][1..parts[2].len() - 1];
    let right = &parts[3][0..parts[3].len() - 1];

    return Node { name, left, right };
}

fn part1(input: &str) -> String {
    let mut lines = input.lines();
    let route = lines.next().expect("should have first line");
    lines.next(); // skip empty line

    let nodes = lines.fold(HashMap::new(), |mut acc, line| {
        let node = get_mapping(line);
        dbg!(&node);
        acc.insert(node.name, node);
        acc
    });

    let route_chars = route.chars().collect::<Vec<char>>();

    let mut pos = "AAA";
    let mut count = 0;
    let mut route_pos = 0;
    let mut node;
    while pos != "ZZZ" {
        node = nodes.get(pos).expect("should receive node");
        pos = match route_chars[route_pos] {
            'L' => node.left,
            'R' => node.right,
            _ => panic!("should only be L or R, got {}", route_chars[route_pos]),
        };
        count += 1;
        route_pos = count % route_chars.len();
    }

    return count.to_string();
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn it_works() {
        let result = part1(
            "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)",
        );
        assert_eq!(result, "6".to_string());
    }
}
