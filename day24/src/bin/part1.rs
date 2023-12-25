use std::collections::{BTreeSet, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let min_bound: f64 = 200_000_000_000_000.;
    let max_bound: f64 = 400_000_000_000_000.;

    let output = process(input, min_bound, max_bound);
    dbg!(output);
}

/**
--- Day 24: Never Tell Me The Odds ---

It seems like something is going wrong with the snow-making process.
Instead of forming snow, the water that's been absorbed into the air
seems to be forming hail!

Maybe there's something you can do to break up the hailstones?

Due to strong, probably-magical winds, the hailstones are all flying
through the air in perfectly linear trajectories. You make a note of
each hailstone's position and velocity (your puzzle input). For example:

19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3

Each line of text corresponds to the position and velocity of a single
hailstone. The positions indicate where the hailstones are right now
(at time 0). The velocities are constant and indicate exactly how far
each hailstone will move in one nanosecond.

Each line of text uses the format px py pz @ vx vy vz. For instance,
the hailstone specified by 20, 19, 15 @ 1, -5, -3 has initial X position 20,
Y position 19, Z position 15, X velocity 1, Y velocity -5, and Z velocity -3.
After one nanosecond, the hailstone would be at 21, 14, 12.

Perhaps you won't have to do anything. How likely are the hailstones
to collide with each other and smash into tiny ice crystals?

To estimate this, consider only the X and Y axes; ignore the Z axis.
Looking forward in time, how many of the hailstones' paths will intersect
within a test area? (The hailstones themselves don't have to collide,
just test for intersections between the paths they will trace.)

In this example, look for intersections that happen with an X and Y position
each at least 7 and at most 27; in your actual data, you'll need to check a
much larger test area. Comparing all pairs of hailstones' future
paths produces the following results:

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 18, 19, 22 @ -1, -1, -2
Hailstones' paths will cross inside the test area (at x=14.333, y=15.333).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths will cross inside the test area (at x=11.667, y=16.667).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=6.2, y=19.4).

Hailstone A: 19, 13, 30 @ -2, 1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone A.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 25, 34 @ -2, -2, -4
Hailstones' paths are parallel; they never intersect.

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-6, y=-5).

Hailstone A: 18, 19, 22 @ -1, -1, -2
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 12, 31, 28 @ -1, -2, -1
Hailstones' paths will cross outside the test area (at x=-2, y=3).

Hailstone A: 20, 25, 34 @ -2, -2, -4
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for hailstone B.

Hailstone A: 12, 31, 28 @ -1, -2, -1
Hailstone B: 20, 19, 15 @ 1, -5, -3
Hailstones' paths crossed in the past for both hailstones.

So, in this example, 2 hailstones' future paths cross inside
the boundaries of the test area.

However, you'll need to search a much larger test area if you want to
see if any hailstones might collide. Look for intersections that
happen with an X and Y position each at least
200000000000000 and at most 400000000000000. Disregard the Z axis entirely.

Considering only the X and Y axes, check all pairs of hailstones'
future paths for intersections. How many of these intersections
occur within the test area?

*/
#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Hail {
    // px py pz @ vx vy vz
    position: Vec3,
    velocity: Vec3,
    m: f64,
    b: f64,
}

#[derive(Debug)]
struct Collision {
    hail_a: usize,
    hail_b: usize,
    // time: f64,
    collision: Vec3,
}

impl Hail {
    fn new(line: &str) -> Hail {
        let parts = line
            .split_whitespace()
            .filter_map(|x| x.parse::<f64>().ok())
            .collect::<Vec<_>>();
        // dbg!(&parts);
        let position = Vec3 {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        };
        let velocity = Vec3 {
            x: parts[3],
            y: parts[4],
            z: parts[5],
        };

        // rise over run
        let m = velocity.y / velocity.x;

        // y = mx + b
        // b = y - mx
        let b = position.y - (m * position.x);

        Hail {
            m,
            b,
            position,
            velocity,
        }
    }

    fn is_after_x(&self, x: f64) -> bool {
        if self.velocity.x > 0. {
            self.position.x < x
        } else if self.velocity.x < 0. {
            self.position.x > x
        } else {
            println!("---Vertical");
            true
        }
    }

    fn get_collision(&self, other: &Hail, min: f64, max: f64) -> Option<Vec3> {
        if other.m == self.m {
            // println!("parallel m1:{}, m2:{}", self.m, other.m);
            return None;
        }
        let x = (other.b - self.b) / (self.m - other.m);
        if x <= min || x >= max {
            // println!(
            //     "out of bounds x1:{}, x2:{}, x:{x}",
            //     self.position.x, other.position.x
            // );
            return None;
        }
        if !self.is_after_x(x) {
            // println!(
            //     "intersects before1 px:{}, vx:{}, x:{x}",
            //     self.position.x, self.velocity.x
            // );
            return None;
        }
        if !other.is_after_x(x) {
            // println!(
            //     "intersects before2 px:{}, vx:{}, x:{x}",
            //     other.position.x, other.velocity.x
            // );
            return None;
        }
        let y = self.m * x + self.b;
        Some(Vec3 { x, y, z: 0. })
    }
}

struct Storm {
    hail: Vec<Hail>,
}

impl Storm {
    fn new(input: &str) -> Storm {
        Storm {
            hail: input.lines().map(Hail::new).collect(),
        }
    }

    fn get_collisions(&mut self, min: f64, max: f64) -> Vec<Collision> {
        let mut collisions = vec![];
        for i in 0..self.hail.len() - 1 {
            for j in i + 1..self.hail.len() {
                let hail_a = &self.hail[i];
                let hail_b = &self.hail[j];
                if let Some(collision) = hail_a.get_collision(hail_b, min, max) {
                    collisions.push(Collision {
                        hail_a: i,
                        hail_b: j,
                        collision,
                    });
                }
            }
        }
        collisions
    }
}

fn process(input: &str, min: f64, max: f64) -> String {
    let mut storm = Storm::new(input);
    let collisions = storm.get_collisions(min, max);
    // dbg!(&collisions);
    return collisions.len().to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let input = include_str!("./t1.txt");
        let result = process(input, 7., 27.);
        assert_eq!(result, "_".to_string());
    }
}
