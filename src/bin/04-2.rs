use std::{collections::HashSet, fs};

fn main() -> anyhow::Result<()> {
  let data = fs::read_to_string("data/04.txt")?;

  let mut counts = vec![0; data.lines().count()];
  for (i, line) in data.lines().enumerate() {
    counts[i] += 1;

    for (lhs, rhs) in line
      .split(':')
      .filter(|x| !x.contains("Card"))
      .flat_map(|x| x.split_once('|'))
    {
      let lhs: HashSet<_> = lhs.split_ascii_whitespace().collect();
      let rhs: HashSet<_> = rhs.split_ascii_whitespace().collect();
      let len = lhs.intersection(&rhs).count();
      for j in 0..len {
        counts[i + j + 1] += counts[i];
      }
    }
  }
  let sum: usize = counts.iter().sum();
  println!("{sum}");
  Ok(())
}
