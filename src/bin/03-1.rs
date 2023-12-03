use std::fs;
use std::str;

const CHAR_TABLE: [char; 10] = ['*', '/', '+', '-', '=', '@', '%', '#', '&', '$'];

fn main() -> anyhow::Result<()> {
  let data = fs::read_to_string("data/03.txt")?;

  let mut sum = 0usize;
  let str_matrix = data.parse::<Matrix<_, 140>>()?;
  for (i, line) in str_matrix.0.iter().enumerate() {
    let mut j = 0;
    while j < line.len() {
      let rest_line = &line[j..];
      let mut poss_tagg = false;
      let mut k = 0;

      while let Some(x) = rest_line.get(k) {
        if x.is_numeric() {
          if str_matrix.scan_unit(i, j + k, &CHAR_TABLE) {
            poss_tagg = true;
          }
          k += 1;
        } else {
          break;
        }
      }
      if poss_tagg {
        sum += &line[j..j + k].iter().collect::<String>().parse()?;
      }
      j += if k == 0 { 1 } else { k };
    }
  }

  println!("{sum}\n");
  Ok(())
}

#[derive(Debug)]
pub struct Matrix<T, const N: usize>(pub [[T; N]; N]);

impl<const N: usize> str::FromStr for Matrix<char, N> {
  type Err = anyhow::Error;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut matrix = Self([['\0'; N]; N]);
    for (str_line, mat_line) in s.lines().zip(matrix.0.iter_mut()) {
      for (s, m) in str_line.chars().zip(mat_line.iter_mut()) {
        *m = s;
      }
    }
    Ok(matrix)
  }
}

impl<T: PartialEq<T>, const N: usize> Matrix<T, N> {
  pub fn get(&self, i: usize, j: usize) -> Option<&T> {
    self.0.get::<usize>(i)?.get(j)
  }

  pub fn scan_unit(&self, i: usize, j: usize, scan_arr: &[T]) -> bool {
    let mut ret = false;
    let mut try_scan = |ii, jj| {
      for x in scan_arr {
        if *self.get(ii, jj).unwrap() == *x {
          ret = true;
        }
      }
    };

    if i != 0 {
      if j != 0 {
        try_scan(i - 1, j - 1)
      }
      try_scan(i - 1, j);
      if j != N - 1 {
        try_scan(i - 1, j + 1)
      }
    }
    if j != 0 {
      try_scan(i, j - 1)
    }
    try_scan(i, j);
    if j != N - 1 {
      try_scan(i, j + 1);
    }

    if i != N - 1 {
      if j != 0 {
        try_scan(i + 1, j - 1)
      }
      try_scan(i + 1, j);
      if j != N - 1 {
        try_scan(i + 1, j + 1);
      }
    }
    ret
  }
}
