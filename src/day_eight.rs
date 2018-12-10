pub fn find_metadata_sum(file: &'static str) -> usize {
  find_metadata(&parse_file(file)).0
}

pub fn find_root_node_value(file: &'static str) -> usize {
  find_node_value(&parse_file(file)).0
}

fn find_node_value(input: &[usize]) -> (usize, usize) {
  let num_children = input[0];
  let num_metadata = input[1];

  let children: Vec<(usize, usize)> = (0..num_children)
    .fold(Vec::new(), |mut v, _| {
      let current_len: usize = v.iter().map(|(_,l)| l).sum();
      let child = &input[2 + current_len..];
      let (val, len) = find_node_value(child);
      v.push((val, len));
      v
    });

  if num_children > 0 {

    let children_length: usize = children.iter()
      .map(|(_,l)| l)
      .sum();

    let value: usize = input.iter()
      .skip(2)
      .skip(children_length)
      .take(num_metadata)
      .filter_map(|i| children.get(*i - 1))
      .map(|(v,_)| v)
      .sum();

    (value, 2 + num_metadata + children_length)
  } else {
    let current_metadata: usize = input.iter()
      .skip(2)
      .take(num_metadata)
      .sum();
    
    (current_metadata, 2 + num_metadata)
  }
}

fn find_metadata(input: &[usize]) -> (usize, usize) {
  let num_children = input[0];
  let num_metadata = input[1];

  let (child_metadata, child_length) = (0..num_children)
    .fold((0usize, 0usize), |r, _| {
      let child = &input[2 + r.1..];
      let (md, len) = find_metadata(child);
      (r.0 + md, r.1 + len)
    });

  let current_metadata: usize = input.iter()
    .skip(2)
    .skip(child_length)
    .take(num_metadata)
    .sum();

  (current_metadata + child_metadata, num_metadata + child_length + 2)
}

#[inline]
fn parse_file(file: &'static str) -> Vec<usize> {
  file
    .trim()
    .split(' ')
    .map(|s| s.parse().unwrap())
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_find_metadata_sum() {
    let test_file = include_str!("test_inputs/day_eight/simple_license.txt");
    let actual = find_metadata_sum(test_file);

    assert_eq!(actual, 138);
  }

  #[test]
  fn can_find_root_node_value() {
    let test_file = include_str!("test_inputs/day_eight/simple_license.txt");
    let actual = find_root_node_value(test_file);

    assert_eq!(actual, 66);
  }
}