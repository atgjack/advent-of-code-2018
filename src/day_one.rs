use std::collections::HashSet;
use std::iter::FromIterator;

pub fn find_final_freqeuency(file: &'static str) -> isize {
  lines_to_iter(file)
    .sum()
}

pub fn find_first_repeated_frequency(file: &'static str) -> isize {
  lines_to_iter(file)
    .cycle()
    .scan(0, |sum, v| { *sum += v; Some(*sum) })
    .scan(init_hashset(), |set, v| Some((*set).replace(v)))
    .find_map(|o| o)
    .unwrap()
}

#[inline]
fn lines_to_iter(file: &'static str) -> impl Iterator<Item=isize> + Clone {
  file.lines()
    .map(|l| l.to_string().parse())
    .filter(|r| r.ok())
}

#[inline]
fn init_hashset() -> HashSet<isize> {
  HashSet::from_iter(vec![0isize].into_iter())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_sum_all_frequency_changes() {
    let test_file = include_str!("../test_inputs/day_one/simple_sum.txt");
    let actual = find_final_freqeuency(test_file);

    assert_eq!(actual, 350);
  }


  #[test]
  fn can_find_first_repeated_frequency() {
    let test_file = include_str!("../test_inputs/day_one/simple_repeat.txt");
    let actual = find_first_repeated_frequency(test_file);

    assert_eq!(actual, 3);
  }

  #[test]
  fn can_find_first_repeated_frequency_with_looping() {
    let test_file = include_str!("../test_inputs/day_one/looping_repeat.txt");
    let actual = find_first_repeated_frequency(test_file);

    assert_eq!(actual, 0);
  }
}
