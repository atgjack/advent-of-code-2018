use std::collections::HashMap;

pub fn calculate_checksum(file: &'static str) -> usize {
  let num_doubles = file.lines()
    .filter(count_dupes(2))
    .count();
  let num_tiples = file.lines()
    .filter(count_dupes(3))
    .count();

  num_doubles * num_tiples
}

pub fn calcuate_diff(file: &'static str) -> String {
  file.lines()
    .find_map(|l1| {
      file.lines()
        .find(has_one_diff(l1))
        .map(find_similar(l1))
    }).unwrap()
}

#[inline]
fn count_dupes(num: usize) -> impl FnMut(&&str) -> bool {
  move |line| {
    line.chars()
      .fold(HashMap::with_capacity(26), |mut map, c| { *map.entry(c).or_insert(0) += 1; map })
      .values()
      .any(|&x| x == num)
  }
}

#[inline]
fn has_one_diff(l1: &'static str) -> impl FnMut(&&str) -> bool {
  move |l2| {
    let chars: Vec<char> = l2.chars().collect();
    l1.chars()
      .enumerate()
      .filter(|(index, c)| *c != chars[*index])
      .count() == 1
  }
}

#[inline]
fn find_similar(l1: &'static str) -> impl FnOnce(&str) -> String {
  move |l2| {
    let chars: Vec<char> = l2.chars().collect();
    l1.chars()
      .enumerate()
      .filter(|(index, c)| *c == chars[*index])
      .map(|(_, c)| c)
      .collect::<String>()
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