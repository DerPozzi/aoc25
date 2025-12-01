use anyhow::Result;

mod part01;
mod part02;

fn main() -> Result<()> {
    let f = include_str!("../../../inputs/day01.txt");
    let part01 = part01::process(f)?;
    println!("Result for part01: {part01}");

    let part02 = part02::process(f)?;
    println!("Result for part02: {part02}");
    Ok(())
}
