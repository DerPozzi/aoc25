use anyhow::Result;

pub fn process(input: &str) -> Result<i32> {
    let test = input
        .lines()
        .map(|bank| {
            let mut first = 0;
            let mut second = 0;

            for (i, c) in bank.chars().enumerate() {
                if i == bank.len() - 1 {
                    break;
                }
                let num = c.to_digit(10).unwrap();
                first = u32::max(num, first);
            }
            let (_, right) = bank.split_at(bank.find(format!("{first}").as_str()).unwrap() + 1);
            for c in right.chars() {
                let num = c.to_digit(10).unwrap();
                second = u32::max(num, second);
            }

            format!("{first}{second}").parse::<i32>().unwrap()
        })
        .sum();
    Ok(test)
}

mod tests {

    #[test]
    fn day03_part01() {
        use crate::part01::process;
        let input = "987654321111111
811111111111119
234234234234278
818181911112111
";

        let result = process(input).unwrap();
        assert_eq!(result, 357)
    }
}
