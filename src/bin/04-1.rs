use itertools::Itertools;
use std::fs;

fn main() -> anyhow::Result<()> {
  let data = fs::read_to_string("data/04.txt")?;

  let sum = data
    .lines()
    .enumerate()
    .map(|(_, line)| {
      line
        .split(':')
        .filter(|x| !x.contains("Card"))
        .flat_map(|x| x.split_once('|'))
        .map(|(lhs, rhs)| {
          (
            lhs
              .split_ascii_whitespace()
              .flat_map(|x| x.parse::<u32>())
              .collect_vec(),
            rhs
              .split_ascii_whitespace()
              .flat_map(|x| x.parse::<u32>())
              .collect_vec(),
          )
        })
        .map(|(lhs, rhs)| {
          lhs
            .iter()
            .enumerate()
            .filter(|(_, x)| rhs.contains(x))
            .map(|(i, _)| i)
            .collect_vec()
        })
        .map(|x| x.len())
    })
    .fold(0, |a, x| {
      let res = 1 << x.into_iter().sum::<usize>();
      a + res / 2
    });

  println!("{sum}");
  Ok(())
}
