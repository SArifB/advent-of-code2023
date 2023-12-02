use anyhow::Result;
use itertools::Itertools;
use std::fs;

fn main() -> Result<()> {
  let data = fs::read_to_string("data/02.txt")?;

  let sum = data
    .lines()
    .map(|line| line.split_terminator([',', ':', ';']).collect_vec())
    .flat_map(|vec| {
      let mut game = Game::new();
      for part in vec {
        let (lhs, rhs) = part.trim().split_once(' ').expect("No space found");
        if lhs == "Game" {
          game.id = rhs.parse().expect("rhs is not a num")
        } else if rhs == "red" {
          let num = lhs.parse::<u32>().expect("lhs is not a num");
          game.red += if num <= 12 {
            num
          } else {
            return None;
          }
        } else if rhs == "green" {
          let num = lhs.parse::<u32>().expect("lhs is not a num");
          game.green += if num <= 13 {
            num
          } else {
            return None;
          }
        } else if rhs == "blue" {
          let num = lhs.parse::<u32>().expect("lhs is not a num");
          game.blue += if num <= 14 {
            num
          } else {
            return None;
          }
        }
      }
      Some(game)
    })
    .fold(0, |a, x| a + x.id);
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
