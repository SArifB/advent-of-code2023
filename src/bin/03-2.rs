use itertools::Itertools;
use std::fs;

fn main() -> anyhow::Result<()> {
  let data = fs::read_to_string("data/03.txt")?;

  let mut stars = Vec::<Point>::new();
  let mut nums = Vec::<NumPos>::new();
  for (i, line) in data.lines().enumerate() {
    for (j, _) in line.char_indices().filter(|(_, x)| *x == '*') {
      stars.push((i, j))
    }

    let mut j = 0;
    while j < line.len() {
      let mut k = 0;
      while let Some(&x) = line.as_bytes().get(j + k) {
        if (x as char).is_numeric() {
          k += 1;
        } else {
          break;
        }
      }
      if k != 0 {
        let val = line[j..j + k].parse()?;
        let locs = (0..k).map(|k| (i, j + k)).collect_vec();
        nums.push(NumPos::new(locs, val));
        j += k;
      } else {
        j += 1;
      }
    }
  }

  let mut founds = Vec::<_>::new();
  for (a, b) in stars {
    let len = data.lines().fold(0, |_, x| x.len());
    let mut tmp = Vec::new();
    let mut func = |a, b| -> Option<NumPos> {
      for (i, num) in nums.clone().into_iter().enumerate() {
        for (x, y) in &num.locs {
          if *x == a && *y == b && !num.found {
            nums[i].found = true;
            return Some(num.clone());
          }
        }
      }
      None
    };

    if a != 0 {
      if b != 0 {
        if let Some(p) = func(a - 1, b - 1) {
          tmp.push(p);
        }
      }
      if let Some(p) = func(a - 1, b) {
        tmp.push(p);
      }
      if b != len - 1 {
        if let Some(p) = func(a - 1, b + 1) {
          tmp.push(p);
        }
      }
    }
    if b != 0 {
      if let Some(p) = func(a, b - 1) {
        tmp.push(p);
      }
    }
    if b != len - 1 {
      if let Some(p) = func(a, b + 1) {
        tmp.push(p);
      }
    }
    if a != len - 1 {
      if b != 0 {
        if let Some(p) = func(a + 1, b - 1) {
          tmp.push(p);
        }
      }
      if let Some(p) = func(a + 1, b) {
        tmp.push(p);
      }
      if b != len - 1 {
        if let Some(p) = func(a + 1, b + 1) {
          tmp.push(p);
        }
      }
    }

    if tmp.len() == 2 {
      let ratio = tmp.first().unwrap().val * tmp.last().unwrap().val;
      founds.push(ratio);
    }
  }

  let sum: u32 = founds.into_iter().sum();
  println!("{sum}");
  Ok(())
}

#[derive(Debug, Clone)]
pub struct NumPos {
  locs: Vec<Point>,
  val: u32,
  found: bool,
}

impl NumPos {
  pub fn includes(&self) {}
}

impl NumPos {
  pub fn new(locas: Vec<Point>, val: u32) -> Self {
    Self {
      locs: locas,
      val,
      found: false,
    }
  }
}

type Point = (usize, usize);
