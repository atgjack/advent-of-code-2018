use std::collections::VecDeque;

pub fn find_high_score(file: &'static str) -> usize {
  let (num_players, last_marble) = parse_line(file);
  calculate_high_score(num_players, last_marble)
}

pub fn find_higher_score(file: &'static str) -> usize {
  let (num_players, last_marble) = parse_line(file);
  calculate_high_score(num_players, last_marble * 100)
}

fn calculate_high_score(num_players: usize, last_marble: usize) -> usize {
  let circle_size = last_marble - ((last_marble / 23) * 2);

  let mut game: VecDeque<usize> = VecDeque::with_capacity(circle_size);
  let mut players: Vec<usize> = vec![0; num_players];

  game.push_back(0);

  for next_marble in 1..last_marble {
    if next_marble % 23 == 0 {
      game.rotate_backward(7);
      let extra_point = game.pop_front().unwrap();
      players[next_marble % num_players] += next_marble + extra_point;
    } else {
      game.rotate_forward(2);
      game.push_front(next_marble);
    }
  }

  *players.iter().max().unwrap()
}

trait Rotate {
  fn rotate_backward(&mut self, num: usize);
  fn rotate_forward(&mut self, num: usize);
}

impl<T> Rotate for VecDeque<T> {
  fn rotate_backward(&mut self, num: usize) {
    (0..num).for_each(|_| {
      let item = self.pop_back().unwrap();
      self.push_front(item);
    })
  }

  fn rotate_forward(&mut self, num: usize) {
    (0..num).for_each(|_| {
      let item = self.pop_front().unwrap();
      self.push_back(item);
    })
  }
}

#[inline]
fn parse_line(line: &'static str) -> (usize, usize) {
  let sections: Vec<&str> = line.split(' ').collect();

  (
    sections[0].parse().unwrap(),
    sections[6].parse().unwrap()
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_find_high_score_simple() {
    let actual = find_high_score("9 players; last marble is worth 25 points");

    assert_eq!(actual, 32);
  }

  #[test]
  fn can_find_high_score() {
    let test_file: Vec<&str> = include_str!("test_inputs/day_nine/simple_scores.txt")
      .lines()
      .collect();

    let actual_one = find_high_score(test_file[0]);
    let actual_two = find_high_score(test_file[1]);
    let actual_tree = find_high_score(test_file[2]);
    let actual_four = find_high_score(test_file[3]);
    let actual_five = find_high_score(test_file[4]);

    assert_eq!(actual_one, 8317);
    assert_eq!(actual_two, 14_6373);
    assert_eq!(actual_tree, 2764);
    assert_eq!(actual_four, 54718);
    assert_eq!(actual_five, 37305);
  }

}