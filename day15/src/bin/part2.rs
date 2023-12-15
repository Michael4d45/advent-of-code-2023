use std::collections::{BTreeMap, BTreeSet, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**
--- Part Two ---

You convince the reindeer to bring you the page;
the page confirms that your HASH algorithm is working.

The book goes on to describe a series of 256 boxes
numbered 0 through 255. The boxes are arranged in a
line starting from the point where light enters the
facility. The boxes have holes that allow light to
pass from one box to the next all the way down the line.

      +-----+  +-----+         +-----+
Light | Box |  | Box |   ...   | Box |
----------------------------------------->
      |  0  |  |  1  |   ...   | 255 |
      +-----+  +-----+         +-----+

Inside each box, there are several lens slots that
will keep a lens correctly positioned to focus light
passing through the box. The side of each box has a
panel that opens to allow you to insert or remove
lenses as necessary.

Along the wall running parallel to the boxes is a
large library containing lenses organized by focal
length ranging from 1 through 9. The reindeer also
brings you a small handheld label printer.

The book goes on to explain how to perform each step
in the initialization sequence, a process it calls
the Holiday ASCII String Helper Manual Arrangement
Procedure, or HASHMAP for short.

Each step begins with a sequence of letters that
indicate the label of the lens on which the step
operates. The result of running the HASH algorithm
on the label indicates the correct box for that step.

The label will be immediately followed by a character
that indicates the operation to perform: either an
equals sign (=) or a dash (-).

If the operation character is a dash (-), go to the
relevant box and remove the lens with the given
label if it is present in the box. Then, move any
remaining lenses as far forward in the box as they
can go without changing their order, filling any
space made by removing the indicated lens.
(If no lens in that box has the given label, nothing happens.)

If the operation character is an equals sign (=), it
will be followed by a number indicating the focal length
of the lens that needs to go into the relevant box; be
sure to use the label maker to mark the lens with the
label given in the beginning of the step so you can
find it later. There are two possible situations:

    If there is already a lens in the box with the same label, replace
    the old lens with the new lens: remove the old lens and put the new
    lens in its place, not moving any other lenses in the box.

    If there is not already a lens in the box with the same label, add
    the lens to the box immediately behind any lenses already in the box.
    Don't move any of the other lenses when you do this. If there aren't
    any lenses in the box, the new lens goes all the way to the front of the box.

Here is the contents of every box after each step in
the example initialization sequence above:

After "rn=1":
Box 0: [rn 1]

After "cm-":
Box 0: [rn 1]

After "qp=3":
Box 0: [rn 1]
Box 1: [qp 3]

After "cm=2":
Box 0: [rn 1] [cm 2]
Box 1: [qp 3]

After "qp-":
Box 0: [rn 1] [cm 2]

After "pc=4":
Box 0: [rn 1] [cm 2]
Box 3: [pc 4]

After "ot=9":
Box 0: [rn 1] [cm 2]
Box 3: [pc 4] [ot 9]

After "ab=5":
Box 0: [rn 1] [cm 2]
Box 3: [pc 4] [ot 9] [ab 5]

After "pc-":
Box 0: [rn 1] [cm 2]
Box 3: [ot 9] [ab 5]

After "pc=6":
Box 0: [rn 1] [cm 2]
Box 3: [ot 9] [ab 5] [pc 6]

After "ot=7":
Box 0: [rn 1] [cm 2]
Box 3: [ot 7] [ab 5] [pc 6]

All 256 boxes are always present; only the boxes that contain
any lenses are shown here. Within each box, lenses are listed
from front to back; each lens is shown as its label and focal
length in square brackets.

To confirm that all of the lenses are installed correctly,
add up the focusing power of all of the lenses. The focusing
power of a single lens is the result of multiplying together:

    One plus the box number of the lens in question.
    The slot number of the lens within the box: 1 for the
        first lens, 2 for the second lens, and so on.
    The focal length of the lens.

At the end of the above example, the focusing power of each lens is as follows:

    rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
    cm: 1 (box 0) * 2 (second slot) * 2 (focal length) = 4
    ot: 4 (box 3) * 1 (first slot) * 7 (focal length) = 28
    ab: 4 (box 3) * 2 (second slot) * 5 (focal length) = 40
    pc: 4 (box 3) * 3 (third slot) * 6 (focal length) = 72

So, the above example ends up with a total focusing power of 145.

With the help of an over-enthusiastic reindeer in a hard hat,
follow the initialization sequence. What is the focusing power
of the resulting lens configuration?

*/

/**
   Determine the ASCII code for the current character of the string.
   Increase the current value by the ASCII code you just determined.
   Set the current value to itself multiplied by 17.
   Set the current value to the remainder of dividing itself by 256.
*/
fn hash(s: &str) -> usize {
    let mut result = 0;
    for c in s.chars() {
        result += c as usize;
        result *= 17;
        result = result % 256;
    }
    result
}

enum OperationType {
    Dash(),
    Equal(usize),
}

struct Operation {
    operation_type: OperationType,
    label: String,
    hash: usize,
}

impl Operation {
    fn new(s: &str) -> Operation {
        if s.contains("=") {
            let mut parts = s.split("=");
            let label = parts.next().expect("Should have at least first part");
            let lense_size = parts.next().expect("Should have a lense size");
            Operation {
                label: label.to_string(),
                hash: hash(label),
                operation_type: OperationType::Equal(
                    lense_size
                        .parse::<usize>()
                        .expect("This should be a number"),
                ),
            }
        } else if s.contains("-") {
            let mut parts = s.split("-");
            let label = parts.next().expect("Should have at least first part");
            Operation {
                label: label.to_string(),
                hash: hash(label),
                operation_type: OperationType::Dash(),
            }
        } else {
            unreachable!("Invalid input: {s}");
        }
    }
}

#[derive(Debug)]
struct StateMachine {
    boxes: BTreeMap<usize, Vec<(String, usize)>>,
}

impl StateMachine {
    fn new() -> StateMachine {
        let mut boxes = BTreeMap::new();

        for i in 0..256 {
            boxes.insert(i, vec![]);
        }

        StateMachine { boxes }
    }

    fn do_op(&mut self, op: Operation) -> &mut StateMachine {
        match op.operation_type {
            //     If the operation character is a dash (-), go to the
            // relevant box and remove the lens with the given
            // label if it is present in the box. Then, move any
            // remaining lenses as far forward in the box as they
            // can go without changing their order, filling any
            // space made by removing the indicated lens.
            // (If no lens in that box has the given label, nothing happens.)
            OperationType::Dash() => {
                if let Some(lense_box) = self.boxes.get_mut(&op.hash) {
                    if let Some(index) = lense_box.iter().position(|(x, _)| x == &op.label) {
                        lense_box.remove(index);
                    }
                }
            }

            // If the operation character is an equals sign (=), it
            // will be followed by a number indicating the focal length
            // of the lens that needs to go into the relevant box; be
            // sure to use the label maker to mark the lens with the
            // label given in the beginning of the step so you can
            // find it later. There are two possible situations:

            //     If there is already a lens in the box with the same label, replace
            //     the old lens with the new lens: remove the old lens and put the new
            //     lens in its place, not moving any other lenses in the box.

            //     If there is not already a lens in the box with the same label, add
            //     the lens to the box immediately behind any lenses already in the box.
            //     Don't move any of the other lenses when you do this. If there aren't
            //     any lenses in the box, the new lens goes all the way to the front of the box.
            OperationType::Equal(focal_length) => {
                if let Some(lense_box) = self.boxes.get_mut(&op.hash) {
                    if let Some(index) = lense_box.iter().position(|(x, _)| x == &op.label) {
                        lense_box[index] = (op.label, focal_length);
                    } else {
                        lense_box.push((op.label, focal_length));
                    }
                }
            }
        };

        self
    }

    fn get_focus_power(&self) -> usize {
        // dbg!(self);
        // One plus the box number of the lens in question.
        // The slot number of the lens within the box: 1 for the
        //     first lens, 2 for the second lens, and so on.
        // The focal length of the lens.

        // rn: 1 (box 0) * 1 (first slot) * 1 (focal length) = 1
        // cm: 1 (box 0) * 2 (second slot) * 2 (focal length) = 4
        // ot: 4 (box 3) * 1 (first slot) * 7 (focal length) = 28
        // ab: 4 (box 3) * 2 (second slot) * 5 (focal length) = 40
        // pc: 4 (box 3) * 3 (third slot) * 6 (focal length) = 72

        let mut factor = 0;
        for (k, lenses) in &self.boxes {
            if lenses.is_empty() {
                continue;
            }
            let mut cur = 0;
            for (i, (_, focal_length)) in lenses.iter().enumerate() {
                cur += (1 + k) * (i + 1) * focal_length;
            }
            factor += cur;
        }
        factor
    }
}

fn process(input: &str) -> String {
    input
        .split(",")
        .map(Operation::new)
        .fold(&mut StateMachine::new(), |state, op| state.do_op(op))
        .get_focus_power()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = process("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");
        assert_eq!(result, "145".to_string());
    }

    #[test]
    fn hash_works() {
        let result = hash("HASH");
        assert_eq!(result, 52);
    }
}
