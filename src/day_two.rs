pub fn calculate_checksum(file: &'static str) -> usize {
  file.lines()
    .map(count_dupes())
    .fold([0usize, 0usize], fold_dupe_counts())
    .iter()
    .product()
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
fn count_dupes() -> impl FnMut(&str) -> [usize; 26] {
  |line| {
    line.chars()
      .map(|c| c as usize - 'a' as usize)
      .fold([0usize; 26], |mut arr, i| { arr[i] += 1; arr })
  }
}

#[inline]
fn fold_dupe_counts() -> impl FnMut([usize; 2], [usize; 26]) -> [usize; 2] {
  |r, n| [ 
    r[0] + n.contains(&2) as usize, 
    r[1] + n.contains(&3) as usize
  ]
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