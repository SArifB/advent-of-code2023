use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
  let data = fs::read_to_string("data/02.txt")?;

  let sum = data
    .lines()
    .map(|line| line.split_terminator([',', ':', ';']).collect_vec())
    .map(|vec| {
      let mut game = Game::new();
      for part in vec {
        let (lhs, rhs) = part.trim().split_once(' ').expect("No space found");
        if lhs == "Game" {
          game.id = rhs.parse().expect("rhs is not a num")
        } else if rhs == "red" {
          let num = lhs.parse().expect("lhs is not a num");
          game.red = if num > game.red { num } else { game.red }
        } else if rhs == "green" {
          let num = lhs.parse().expect("lhs is not a num");
          game.green = if num > game.green { num } else { game.green }
        } else if rhs == "blue" {
          let num = lhs.parse().expect("lhs is not a num");
          game.blue = if num > game.blue { num } else { game.blue }
        }
      }
      game
    })
    .fold(0, |a, x| a + x.red * x.green * x.blue);
  println!("{sum}");
  Ok(())
}

#[derive(Debug)]
struct Game {
  id: u32,
  red: u32,
  green: u32,
  blue: u32,
}

impl Game {
  fn new() -> Self {
    Self {
      id: 0,
      red: 0,
      green: 0,
      blue: 0,
    }
  }
}
