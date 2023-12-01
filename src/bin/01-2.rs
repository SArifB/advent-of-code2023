use anyhow::Result;
use itertools::Itertools;
use std::fs;

const WORD_TABLE: [&str; 9] = [
  "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];
const NUM_TABLE: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

fn main() -> Result<()> {
  let data = fs::read_to_string("data/01.txt")?;

  let sum = data
    .lines()
    .map(|line| {
      line
        .bytes()
        .enumerate()
        .map(|(i, _)| {
          let rest_line = &line[i..];
          for (i, word) in WORD_TABLE.iter().enumerate() {
            if rest_line.starts_with(word) {
              return i as u32 + 1;
            }
          }
          for (i, &ch) in NUM_TABLE.iter().enumerate() {
            if rest_line.starts_with(|x| x == ch) {
              return i as u32 + 1;
            }
          }
          0
        })
        .filter(|&x| x != 0)
        .collect_vec()
    })
    .flat_map(|vec| format!("{}{}", vec[0], vec[vec.len() - 1]).parse::<usize>())
    .sum::<usize>();

  println!("{:?}", sum);
  Ok(())
}
