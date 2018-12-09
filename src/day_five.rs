pub fn find_remainder(file: &'static str) -> usize {
  calculate_remainer(file.chars())
}

pub fn find_extracted_remainder(file: &'static str) -> usize {
  (0..26)
    .fold([0usize;26], remove_letter_and_calcuate_remainder(file))
    .iter()
    .fold(std::usize::MAX, find_min())
}

#[inline]
fn remove_letter_and_calcuate_remainder(file: &'static str) -> impl FnMut([usize;26], u8) -> [usize;26] {
  move |mut r, i| {
    r[i as usize] = calculate_remainer(file.chars().filter(remove_char(i)));
    r
  }
}

#[inline]
fn remove_char(i: u8) -> impl FnMut(&char) -> bool {
  move |c| c.to_ascii_lowercase() != (i + b'a') as char
}

#[inline]
fn calculate_remainer(chars: impl Iterator<Item=char>) -> usize {
  chars
    .filter(|l| l.is_alphanumeric())
    .fold(Vec::new(), construct_new_string())
    .len()
}

#[inline]
fn find_min()-> impl FnMut(usize, &usize) -> usize {
  |r, &n| if n < r { n } else { r }
}

#[inline]
fn construct_new_string() -> impl FnMut(Vec<char>, char) -> Vec<char> {
  |mut result, next| {
    if let Some(&prev) = result.last() {
      if letters_explode(prev, next) {
        result.pop();
      } else {
        result.push(next);
      }
    } else {
      result.push(next);
    }

    result
  }
}

#[inline]
fn letters_explode(l1: char, l2: char) -> bool {
  l1.to_ascii_lowercase() == l2.to_ascii_lowercase()
  && l1 != l2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_find_remainder() {
    let test_file = include_str!("test_inputs/day_five/simple_polymer.txt");
    let actual = find_remainder(test_file);

    assert_eq!(actual, 10);
  }

  #[test]
  fn can_find_extracted_remainder() {
    let test_file = include_str!("test_inputs/day_five/simple_polymer.txt");
    let actual = find_extracted_remainder(test_file);

    assert_eq!(actual, 4);
  }

  #[test]
  fn can_compare_chars() {
    let char1 = 'C';
    let char2 = 'c';
    let char3 = 'c';
    let char4 = 'J';

    assert!(letters_explode(char1, char2));
    assert!(letters_explode(char2, char1));

    assert!(!letters_explode(char2, char3));
    assert!(!letters_explode(char3, char4));
    assert!(!letters_explode(char1, char4));
  }
}