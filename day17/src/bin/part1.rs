use core::fmt;
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
};

fn main() {
    // let input = include_str!("./input.txt");
    let input = include_str!("./t1.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Day 17: Clumsy Crucible ---

The lava starts flowing rapidly once the
Lava Production Facility is operational.
As you leave, the reindeer offers you a
parachute, allowing you to quickly reach
Gear Island.

As you descend, your bird's-eye view of Gear Island
reveals why you had trouble finding anyone on your
way up: half of Gear Island is empty, but the half
below you is a giant factory city!

You land near the gradually-filling pool of lava at
the base of your new lavafall. Lavaducts will eventually
carry the lava throughout the city, but to make use of
it immediately, Elves are loading it into large
crucibles on wheels.

The crucibles are top-heavy and pushed by hand.
Unfortunately, the crucibles become very difficult
to steer at high speeds, and so it can be hard
to go in a straight line for very long.

To get Desert Island the machine parts it needs as
soon as possible, you'll need to find the best way
to get the crucible from the lava pool to the machine
parts factory. To do this, you need to minimize heat
loss while choosing a route that doesn't require the
crucible to go in a straight line for too long.

Fortunately, the Elves here have a map (your puzzle input)
that uses traffic patterns, ambient temperature, and hundreds
of other parameters to calculate exactly how much heat loss
can be expected for a crucible entering any particular city block.

For example:

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533

Each city block is marked by a single digit that
represents the amount of heat loss if the crucible
enters that block. The starting point, the lava pool,
is the top-left city block; the destination,
the machine parts factory, is the bottom-right city block.
(Because you already start in the top-left block, you
    don't incur that block's heat loss unless you leave
    that block and then return to it.)

Because it is difficult to keep the top-heavy crucible
going in a straight line for very long, it can move
at most three blocks in a single direction before
it must turn 90 degrees left or right. The crucible
also can't reverse direction; after entering each
city block, it may only turn left, continue straight,
or turn right.

One way to minimize heat loss is this path:

2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>

This path never moves more than three
consecutive blocks in the same direction
and incurs a heat loss of only 102.

Directing the crucible from the lava
pool to the machine parts factory, but
not moving more than three consecutive
blocks in the same direction, what
is the least heat loss it can incur?

*/
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Node {
    heat_loss: u32,
    least_heat_loss: u32,
    x: isize,
    y: isize,
    is_visited: bool,
    from_x: isize,
    from_y: isize,
    direction: Option<Direction>,
}

impl Node {
    fn new(c: char, (x, y): (isize, isize)) -> Node {
        Node {
            heat_loss: c.to_digit(10).unwrap(),
            x,
            y,
            is_visited: false,
            from_x: -1,
            from_y: -1,
            direction: None,
            least_heat_loss: 0,
        }
    }

    fn get_pos_at_direction(&self, direction: Direction) -> (isize, isize) {
        match direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        }
    }

    fn get_parent(&self, graph: &BTreeMap<(isize, isize), Node>) -> Option<Node> {
        if let Some(direction) = self.direction {
            let pos = self.get_pos_at_direction(direction.get_opposite());
            if let Some(&node) = graph.get(&pos) {
                return Some(node);
            }
        }

        None
    }

    fn get_same_direction_parent(&self, graph: &BTreeMap<(isize, isize), Node>) -> Option<Node> {
        if let Some(parent) = self.get_parent(graph) {
            if parent.direction == self.direction {
                return Some(parent);
            }
        }

        None
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare based on heat_loss in ascending order
        self.least_heat_loss.cmp(&other.least_heat_loss)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_opposite(&self, other: Direction) -> bool {
        other == self.get_opposite()
    }

    fn get_opposite(&self) -> Direction {
        use Direction::*;
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }
}

/**
#[derive(Debug, Clone, PartialEq, Eq)]
struct Traversal {
    order: Uuid,
    heat_loss: u32,
    curr_straight: u8,
    direction: Direction,
    last_x: isize,
    last_y: isize,
    distance_from_end: isize,
}

impl Traversal {
    fn new(node: Node, direction: Direction, end: (isize, isize)) -> Traversal {
        Traversal {
            order: Uuid::new_v4(),
            heat_loss: 0,
            curr_straight: 1,
            direction,
            last_x: node.x,
            last_y: node.y,
            distance_from_end: (end.0 - node.x) + (end.1 - node.y),
        }
    }

    fn new_append(
        &self,
        pos: (isize, isize),
        direction: Direction,
        graph: &Graph,
        traversals: &mut Vec<Traversal>,
        end: (isize, isize),
    ) {
        if direction.is_opposite(self.direction) {
            return;
        }
        if let Some(node) = graph.graph.get(&pos) {
            let mut curr_straight = self.curr_straight;
            if direction == self.direction {
                curr_straight += 1;
            } else {
                curr_straight = 1;
            }
            if curr_straight <= 3 {
                traversals.push(Traversal {
                    order: Uuid::new_v4(),
                    last_x: pos.0,
                    last_y: pos.1,
                    heat_loss: self.heat_loss + node.heat_loss,
                    curr_straight,
                    direction,
                    distance_from_end: (end.0 - node.x) + (end.1 - node.y),
                });
            }
        }
    }

    fn is_at(&self, (x, y): (isize, isize)) -> bool {
        // dbg!(self.last_x, x, self.last_y, y);
        self.last_x == x && self.last_y == y
    }

    fn get_next(&self, graph: &Graph, end: (isize, isize)) -> Vec<Traversal> {
        use Direction::*;
        let mut traversals = vec![];
        self.new_append(
            (self.last_x, self.last_y - 1),
            Up,
            graph,
            &mut traversals,
            end,
        );
        self.new_append(
            (self.last_x, self.last_y + 1),
            Down,
            graph,
            &mut traversals,
            end,
        );
        self.new_append(
            (self.last_x - 1, self.last_y),
            Left,
            graph,
            &mut traversals,
            end,
        );
        self.new_append(
            (self.last_x + 1, self.last_y),
            Right,
            graph,
            &mut traversals,
            end,
        );
        // dbg!(&traversals);
        traversals
    }
}

impl Ord for Traversal {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Compare based on heat_loss in ascending order
        let mut cmp = self.heat_loss.cmp(&other.heat_loss);

        if cmp == Ordering::Equal {
            cmp = self.distance_from_end.cmp(&other.distance_from_end);
        }
        if cmp == Ordering::Equal {
            cmp = self.order.cmp(&other.order);
        }
        cmp
    }
}

impl PartialOrd for Traversal {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
*/

#[derive(Debug, Clone)]
struct Graph {
    graph: BTreeMap<(isize, isize), Node>,
    size_x: isize,
    size_y: isize,
}

impl Graph {
    fn new(input: &str) -> Graph {
        let mut graph = BTreeMap::new();
        let mut size_y = 0;
        let mut size_x = 0;
        for (y, line) in input.lines().enumerate() {
            size_y = size_y.max(y as isize);
            for (x, c) in line.chars().enumerate() {
                size_x = size_x.max(x as isize);
                graph.insert(
                    (x as isize, y as isize),
                    Node::new(c, (x as isize, y as isize)),
                );
            }
        }
        dbg!(&graph, size_x, size_y);
        Graph {
            graph,
            size_x,
            size_y,
        }
    }

    fn process(&self, start: (isize, isize), end: (isize, isize)) -> Node {
        use Direction::*;
        let mut graph: BTreeMap<(isize, isize), Node> = self.graph.clone();
        let mut nodes_to_process = vec![*graph.get(&start).unwrap()];
        dbg!(&nodes_to_process);
        while let Some(node) = nodes_to_process.pop() {
            if node.x == end.0 && node.y == end.1 {
                println!("{}", node.least_heat_loss);
                self.print(&graph);
            }
            // dbg!(traversals.len());
            // dbg!(&nodes_to_process);
            Graph::new_append(&node, Up, &mut graph, &mut nodes_to_process);
            Graph::new_append(&node, Down, &mut graph, &mut nodes_to_process);
            Graph::new_append(&node, Left, &mut graph, &mut nodes_to_process);
            Graph::new_append(&node, Right, &mut graph, &mut nodes_to_process);
            nodes_to_process.sort();
            // dbg!(&nodes_to_process.len());
        }

        // dbg!(&graph);

        todo!();
    }

    fn print(&self, graph: &BTreeMap<(isize, isize), Node>) {
        let mut output = String::new();

        for row in 0..=self.size_y {
            for col in 0..=self.size_x {
                let position = (col, row);
                let space = graph.get(&position).unwrap();

                let least_heat_loss = space.least_heat_loss.to_string();
                output.push_str(least_heat_loss.as_str());
                output.push('(');
                output.push_str(space.heat_loss.to_string().as_str());
                output.push(')');
                for _ in 0..3 - least_heat_loss.len() {
                    output.push('-');
                }
                if let Some(direction) = space.direction {
                    let mut temp_node = graph.get(&position).unwrap().clone();
                    while let Some(parent) = temp_node.get_parent(graph) {
                        if parent.x != temp_node.from_x && parent.y != temp_node.from_y {
                            break;
                        }
                        temp_node = parent;
                    }
                    if temp_node.x == 0 && temp_node.y == 0 {
                        match direction {
                            Direction::Up => output.push('v'),
                            Direction::Down => output.push('^'),
                            Direction::Left => output.push('>'),
                            Direction::Right => output.push('<'),
                        }
                    }
                } else {
                    output.push(' ');
                }

                output.push(' ');
            }
            output.push('\n');
        }

        println!("{}", output);
    }

    fn new_append(
        from_node: &Node,
        direction: Direction,
        graph: &mut BTreeMap<(isize, isize), Node>,
        nodes_to_process: &mut Vec<Node>,
    ) {
        let current_direction = from_node.direction.unwrap_or(direction);
        if direction.is_opposite(current_direction) {
            return;
        }

        let pos = from_node.get_pos_at_direction(direction);

        if let Some(_node) = graph.get(&pos) {
            let mut same_direction_count = 0;
            if direction == current_direction {
                same_direction_count += 1;
                if let Some(node_1) = from_node.get_same_direction_parent(graph) {
                    same_direction_count += 1;
                    if let Some(_node_2) = node_1.get_same_direction_parent(graph) {
                        same_direction_count += 1;
                    }
                }
            }
            if same_direction_count < 3 {
                let node = graph.get_mut(&pos).unwrap();
                let heat_loss = from_node.least_heat_loss + node.heat_loss;
                if (node.is_visited && node.least_heat_loss > heat_loss) || !node.is_visited {
                    node.least_heat_loss = heat_loss;
                    node.from_x = from_node.x;
                    node.from_y = from_node.y;
                    node.direction = Some(direction);
                    node.is_visited = true;
                    nodes_to_process.push(*node);
                }
            }
        }
    }
}

fn process(input: &str) -> String {
    let graph = Graph::new(input);
    let traversal = graph.process((0, 0), (graph.size_x, graph.size_y));

    return traversal.least_heat_loss.to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works_small() {
        let result = process(
            "241
321",
        );
        assert_eq!(result, "102".to_string());
    }

    //     #[test]
    //     fn it_works() {
    //         let result = process(
    //             "2413432311323
    // 3215453535623
    // 3255245654254
    // 3446585845452
    // 4546657867536
    // 1438598798454
    // 4457876987766
    // 3637877979653
    // 4654967986887
    // 4564679986453
    // 1224686865563
    // 2546548887735
    // 4322674655533",
    //         );
    //         assert_eq!(result, "102".to_string());
    //     }
}
