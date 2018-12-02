use std::collections::HashMap;

pub fn calculate_checksum(file: &'static str) -> usize {
  let counts: Vec<Vec<usize>> = file.lines()
    .map(count_dupes())
    .collect();
  let num_doubles = counts.iter()
    .filter(|v| v.contains(&2))
    .count();
  let num_triples = counts.iter()
    .filter(|v| v.contains(&3))
    .count();

  num_doubles * num_triples
}

pub fn calcuate_diff(file: &'static str) -> String {
  file.lines()
    .enumerate()
    .find_map(|(index, l1)| {
      file.lines()
        .skip(index + 1)
        .find(has_one_diff(l1))
        .map(find_similar(l1))
    }).unwrap()
}

#[inline]
fn count_dupes() -> impl FnMut(&str) -> Vec<usize> {
  |line| {
    line.chars()
      .fold(HashMap::with_capacity(26), |mut map, c| { *map.entry(c).or_insert(0) += 1; map })
      .values()
      .map(|&v| v as usize)
      .collect()
  }
}

#[inline]
fn has_one_diff(l1: &'static str) -> impl FnMut(&&str) -> bool {
  move |l2| {
    l1.chars()
      .zip(l2.chars())
      .filter(|(c1, c2)| c1 != c2)
      .count() == 1
  }
}

#[inline]
fn find_similar(l1: &'static str) -> impl FnOnce(&str) -> String {
  move |l2| {
    l1.chars()
      .zip(l2.chars())
      .filter(|(c1, c2)| c1 == c2)
      .map(|(c, _)| c)
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_calculate_simple_checksum() {
    let test_file = include_str!("test_inputs/day_two/simple_ids.txt");

    let actual = calculate_checksum(test_file);

    assert_eq!(actual, 12)
  }
  
  #[test]
  fn can_calculate_simple_diffs() {
    let test_file = include_str!("test_inputs/day_two/simple_diffs.txt");

    let actual = calcuate_diff(test_file);

    assert_eq!(actual, "fgij")
  }
}