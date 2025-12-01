use anyhow::{Result, bail};

pub fn process(input: &str) -> Result<i32> {
    let mut number = 50;
    let mut counter = 0;

    for line in input.lines() {
        let (dir, steps) = line.split_at(1);
        let steps = steps.parse::<i32>()?;

        match dir {
            "R" => number += steps,
            "L" => number -= steps,
            _ => bail!("invalid direction"),
        }

        number = number.rem_euclid(100);
        if number == 0 {
            counter += 1;
        }
    }
    Ok(counter)
}

mod tests {

    #[test]
    fn part01() {
        use crate::part01::process;
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        let result = process(input).unwrap();
        assert_eq!(result, 3)
    }
}
