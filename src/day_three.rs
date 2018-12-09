use std::collections::HashMap;
use std::num::ParseIntError;

pub fn find_overlap(file: &'static str) -> usize {
  file.lines()
    .map(Rect::from_str)
    .filter_map(|r| r.ok())
    .flat_map(|r| r.to_vec())
    .fold(create_hashmap(), |mut map, point| {
      *map.entry(point).or_insert(0) += 1;
      map
    }).iter()
    .filter(|(_, &count)| count > 1)
    .count()
}

pub fn find_nonoverlap(file: &'static str) -> usize {
  let rects: Vec<Rect> = file.lines()
    .map(Rect::from_str)
    .filter_map(|r| r.ok())
    .collect();
    

  rects.clone().iter()
    .find(|rect| !rects.iter().any(|other| {
      if rect.id == other.id {
        return false
      }
      rect.overlaps(&other)
    }))
    .unwrap()
    .id
}

#[inline]
fn create_hashmap() -> HashMap<(usize, usize), usize> {
  HashMap::new()
}

#[derive(Clone)]
struct Rect {
  id: usize,
  left: usize,
  top: usize,
  width: usize,
  height: usize,
}

impl Rect {
  pub fn from_str(string: &'static str) -> Result<Rect, ParseIntError> {
    let input: Vec<&'static str> = string.split_whitespace().collect();
    let lt: Vec<&'static str> = input[2][..input[2].len() - 1].split(',').collect();
    let wh: Vec<&'static str> = input[3].split('x').collect();

    Ok(Rect {
      id: input[0][1..].to_string().parse()?,
      left: lt[0].to_string().parse()?,
      top: lt[1].to_string().parse()?,
      width: wh[0].to_string().parse()?,
      height: wh[1].to_string().parse()?,
    })
  }

  fn to_vec(&self) -> Vec<(usize, usize)> {
    (self.left..self.left + self.width)
      .flat_map(move |x| 
        (self.top..self.height + self.top)
          .map(move |y| (x, y))
      ).collect()
  }

  fn overlaps(&self, other: &Rect) -> bool {
    self.left < other.left + other.width
    && self.left + self.width > other.left
    && self.top < other.top + other.height
    && self.top + self.height > other.top
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_find_overlap() {
    let test_file = include_str!("test_inputs/day_three/simple_overlap.txt");
    let actual = find_overlap(test_file);

    assert_eq!(actual, 4);
  }

  #[test]
  fn can_parse_string() {
    let test_string = "#123 @ 3,2: 5x4";
    let actual = Rect::from_str(test_string).unwrap();

    assert_eq!(actual.id, 123);
    assert_eq!(actual.left, 3);
    assert_eq!(actual.top, 2);
    assert_eq!(actual.width, 5);
    assert_eq!(actual.height, 4);
  }

  #[test]
  fn can_tell_overlaps() {
    let r1 = Rect { id: 1, left: 1, top: 1,  width: 2, height: 2 };
    let r2 = Rect { id: 1, left: 2, top: 2,  width: 2, height: 2 };
    let r3 = Rect { id: 5, left: 5, top: 2,  width: 2, height: 2 };

    assert!(r1.overlaps(&r2));
    assert!(r2.overlaps(&r1));
    assert!(!r1.overlaps(&r3));
    assert!(!r2.overlaps(&r3));
  }

  #[test]
  fn can_find_nonoverlap() {
    let test_file = include_str!("test_inputs/day_three/simple_overlap.txt");
    let actual = find_nonoverlap(test_file);

    assert_eq!(actual, 3);
  }
}