use std::collections::HashMap;

type Coordinate = (isize, isize);

pub fn find_largest_area(file: &'static str) -> usize {
  let coords = parse_coordinates(file);

  let (max_x, max_y) = coords.iter()
    .fold((0isize, 0isize), find_grid_corner());

  let index_to_coords = (0..=max_x)
    .flat_map(|x| (0..=max_y).map(move |y| (x, y)))
    .fold(HashMap::new(), fold_to_indexs(coords));

  index_to_coords
    .iter()
    .filter(filter_not_on_border(max_x, max_y))
    .fold(0, fold_most_area())
}

pub fn find_safest_region_area(file: &'static str, total_distance: usize) -> usize {
  let coords = parse_coordinates(file);

  let (max_x, max_y) = coords.iter()
    .fold((0isize, 0isize), find_grid_corner());

  (0..=max_x)
    .flat_map(|x| (0..=max_y).map(move |y| (x, y)))
    .map(|coord| coords.iter().map(map_distances(coord)).sum())
    .filter(|t: &usize| *t < total_distance)
    .count()
}

#[inline]
fn parse_coordinates(file: &'static str) -> Vec<Coordinate> {
  file.lines()
    .map(|l| l.split(", ").collect())
    .map(|s: Vec<&'static str>| (s[0].parse().unwrap(), s[1].parse().unwrap()))
    .collect()
}

#[inline]
fn fold_most_area() -> impl FnMut(usize, (&usize, &Vec<Coordinate>)) -> usize {
  |m, (_, v)| if v.len() > m { v.len() } else { m }
}

#[inline]
fn filter_not_on_border(max_x: isize, max_y: isize) -> impl FnMut(&(&usize, &Vec<Coordinate>)) -> bool {
  move |(_, v)| {
    v.iter().filter(|&(x,y)| {
      *x == 0isize || *y == 0isize || *x == max_x || *y == max_y
    }).count() == 0
  }
}

#[inline]
fn fold_to_indexs(coords: Vec<Coordinate>) -> impl FnMut(HashMap<usize, Vec<Coordinate>>, Coordinate) -> HashMap<usize, Vec<Coordinate>> {
  move |mut map, coord| {
    let (_, owners) = coords.iter()
      .map(map_distances(coord))
      .enumerate()
      .fold((std::usize::MAX, Vec::new()), |(m, mut v), (i, d)| {
        if d < m {
          v.clear();
          v.push(i);
          (d, v)
        } else if d == m {
          v.push(i);
          (d, v)
        } else {
          (m, v)
        }
      });
    
    if owners.len() == 1 {
      map.entry(owners[0]).or_insert_with(Vec::new).push(coord)
    }

    map
  }
}

#[inline]
fn map_distances((cx, cy): Coordinate) -> impl FnMut(&Coordinate) -> usize {
  move |(x,y)| ((x - cx).abs() + (y - cy).abs()) as usize
}

#[inline]
fn find_grid_corner() -> impl FnMut(Coordinate, &Coordinate) -> Coordinate {
  |(x, y), &(nx, ny)| {
    (
      if nx > x { nx } else { x },
      if ny > y { ny } else { y }
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_find_largest_area() {
    let test_file = include_str!("test_inputs/day_six/simple_coordinates.txt");
    let actual = find_largest_area(test_file);

    assert_eq!(actual, 17);
  }


  #[test]
  fn can_find_safest_region_area() {
    let test_file = include_str!("test_inputs/day_six/simple_coordinates.txt");
    let actual = find_safest_region_area(test_file, 32);

    assert_eq!(actual, 16);
  }
}