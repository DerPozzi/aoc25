use anyhow::{Result, bail};
const DIAL_TOTAL: i32 = 100;

pub fn process(input: &str) -> Result<i32> {
    let mut number = 50;
    let mut counter = 0;

    for line in input.lines() {
        let (dir, steps) = line.split_at(1);
        let steps = steps.parse::<i32>()?;

        let rot = match dir {
            "R" => steps,
            "L" => -steps,
            _ => bail!("invalid direction"),
        };

        let (new_number, add_counter) = spin(number, rot);
        number = new_number;
        counter += add_counter;
    }
    Ok(counter)
}
fn spin(dial: i32, rot: i32) -> (i32, i32) {
    let dial_long = dial + rot;
    let mut revolutions = (dial_long / DIAL_TOTAL).abs();

    if dial != 0 && dial_long <= 0 {
        revolutions += 1;
    }

    (dial_long.rem_euclid(DIAL_TOTAL), revolutions)
}
mod tests {

    #[test]
    fn part02() {
        use crate::part02::process;
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
        assert_eq!(result, 6)
    }

    #[test]
    fn part02_large() {
        use crate::part02::process;
        let input = "R1000\nL1000";
        let result = process(input).unwrap();
        assert_eq!(result, 20)
    }

    #[test]
    fn part02_spec() {
        use crate::part02::process;
        let input = "R732";
        let result = process(input).unwrap();
        assert_eq!(result, 7)
    }
    #[test]
    fn part02_spec_case1() {
        use crate::part02::process;
        let input = "R120\nL50\nR300";
        let result = process(input).unwrap();
        assert_eq!(result, 4);
    }

    // #[test]
    // fn part02_spec_case2() {
    //     use crate::part02::process;
    //     let input = "L200\nR75\nL25";
    //     let result = process(input).unwrap();
    //     assert_eq!(result, 2);
    // }

    #[test]
    fn part02_spec_case3() {
        use crate::part02::process;
        let input = "R501\nR32\nL99";
        let result = process(input).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn part02_spec_case4() {
        use crate::part02::process;
        let input = "L333\nR100\nR250";
        let result = process(input).unwrap();
        assert_eq!(result, 6);
    }

    #[test]
    fn part02_spec_case5() {
        use crate::part02::process;
        let input = "R10\nR10\nR10\nR10\nR10";
        let result = process(input).unwrap();
        assert_eq!(result, 1);
    }

    // #[test]
    // fn part02_spec_case6() {
    //     use crate::part02::process;
    //     let input = "L150\nL150\nL150";
    //     let result = process(input).unwrap();
    //     assert_eq!(result, 4);
    // }

    #[test]
    fn part02_spec_case7() {
        use crate::part02::process;
        let input = "R99\nL1";
        let result = process(input).unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn part02_spec_case8() {
        use crate::part02::process;
        let input = "R250\nR250\nR250\nR250";
        let result = process(input).unwrap();
        assert_eq!(result, 10);
    }

    #[test]
    fn part02_spec_case9() {
        use crate::part02::process;
        let input = "L500\nR123\nL77";
        let result = process(input).unwrap();
        assert_eq!(result, 7);
    }

    #[test]
    fn part02_spec_case10() {
        use crate::part02::process;
        let input = "R42\nR58\nL100\nR200";
        let result = process(input).unwrap();
        assert_eq!(result, 4);
    }
}
