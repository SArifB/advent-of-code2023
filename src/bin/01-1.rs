use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
  let data = fs::read_to_string("data/01.txt")?;
  let sum: u32 = data
    .lines()
    .map(|line| line.chars().filter(|x| x.is_numeric()).collect_vec())
    .map(|vec| [vec[0] as u8, vec[vec.len() - 1] as u8])
    .flat_map(|slice| String::from_utf8_lossy(&slice).parse::<u32>())
    .sum();
  println!("{sum:?}");

  println!("Hello, from 01-1!");
  Ok(())
}
