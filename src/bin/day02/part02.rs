use anyhow::Result;

pub fn process(input: &str) -> Result<i64> {
    let result = input
        .split(",")
        .map(|range| {
            let (from, to) = range.split_once("-").unwrap();
            let from: i64 = from.trim().parse().unwrap();
            let to: i64 = to.trim().parse().unwrap();

            let mut sum = 0;

            // println!("Going from {from} to {to}");
            for number in from..=to {
                let string_num = format!("{number}");
                let mut buf = String::new();

                for c in string_num.chars() {
                    buf.push(c);
                    if buf == string_num {
                        continue;
                    }
                    let v: Vec<&str> = string_num.split(&buf).collect();
                    if v.len() >= 2 {
                        if v.windows(2).all(|w| w[0] == w[1]) {
                            sum += number;
                            break;
                        }
                    }
                }
            }

            sum
        })
        .sum();
    Ok(result)
}

mod tests {

    #[test]
    fn day02_part02() {
        use crate::part02::process;
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let result = process(input).unwrap();
        assert_eq!(result, 4174379265)
    }
}
