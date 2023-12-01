use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
  let data = fs::read_to_string("data/01.txt")?;

  println!("Hello, from 01-1!");
  Ok(())
}
