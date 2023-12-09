use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
  let data = fs::read_to_string("data/05.txt")?;
  let mut data = data.split("\n\n").map(|piece| piece.lines());

  let mut seeds = data
    .next()
    .context("No seeds found")?
    .flat_map(|line| line.split_once(':'))
    .map(|(_, nums)| {
      nums
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<i64>())
        .collect_vec()
    })
    .next()
    .context("Didnt receive numbers")?;

  let data: Vec<Vec<(i64, i64, i64)>> = data
    .map(|chunk| {
      chunk
        .skip(1)
        .map(|line| line.split_ascii_whitespace())
        .flat_map(|nums| nums.flat_map(|x| x.parse()).collect_tuple())
        .collect_vec()
    })
    .collect_vec();

  for num in &mut seeds {
    for batch in &data {
      'batch: for tup in batch {
        if *num - tup.1 >= 0 && *num - (tup.1 + tup.2) < 0 {
          *num = *num - tup.1 + tup.0;
          break 'batch;
        }
      }
    }
  }
  let min = seeds.into_iter().min().context("No viable seeds found")?;

  println!("{min:?}");
  Ok(())
}
