use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

/**

*/
fn process(input: &str) -> String {
    let mut lines = input.lines();

    return 0.to_string();
}

#[cfg(test)]
mod tests {
    use crate::process;

    #[test]
    fn it_works() {
        let result = process(
            "",
        );
        assert_eq!(result, "".to_string());
    }
}
