use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
  let data = fs::read_to_string("data/05.txt")?;
  let mut data = data.split("\n\n").map(|piece| piece.lines());

  let seed_range: Vec<(i64, i64)> = data
    .next()
    .context("No seeds found")?
    .flat_map(|line| line.split_once(':'))
    .map(|(_, nums)| {
      nums
        .split_ascii_whitespace()
        .flat_map(|x| x.parse::<i64>())
        .tuples::<(_, _)>()
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

  let mut min = 0;
  'outer: for (i, mut x) in (0..).enumerate() {
    for chunk in data.iter().rev() {
      'inner: for &(dest, src, len) in chunk {
        if x >= dest && x < dest + len {
          x = x - dest + src;
          break 'inner;
        }
      }
    }
    for &(beg, len) in &seed_range {
      if x >= beg && x < beg + len {
        min = i;
        break 'outer;
      }
    }
  }
  println!("{min}");
  Ok(())
}
