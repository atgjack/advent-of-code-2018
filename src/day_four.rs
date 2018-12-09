use std::num::ParseIntError;
use std::collections::HashMap;

pub fn find_id_times_minute(file: &'static str) -> usize {
  let map: Vec<(usize, (usize, usize))> = sorted_lines(file).iter()
    .flat_map(|&s| GuardAction::from_str(s))
    .scan(None, enumerate_by_id())
    .flat_map(|o| o)
    .scan(None, compile_sleep_times())
    .flat_map(|o| o)
    .collect();

  let guard = map.iter()
    .fold(HashMap::new(), fold_total_minutes_asleep())
    .iter()
    .fold((0usize, 0usize), find_most_slept())
    .0;

  let minutes: Vec<usize> = map
    .iter()
    .fold(HashMap::new(), fold_minutes_asleep())
    .get_mut(&guard)
    .unwrap()
    .to_vec();

  let minute = (0usize..60usize)
    .fold((0usize, 0usize), |(m, c), n| {
      let count = minutes.iter().filter(|i| **i == n).count();
      if count > c { (n, count) } else { (m, c) }
    }).0;

  minute * guard
}

pub fn find_id_times_minute_most_asleep(file: &'static str) -> usize {
  let map: Vec<(usize, (usize, usize))> = sorted_lines(file).iter()
    .flat_map(|&s| GuardAction::from_str(s))
    .scan(None, enumerate_by_id())
    .flat_map(|o| o)
    .scan(None, compile_sleep_times())
    .flat_map(|o| o)
    .collect();

  let minutes = map
    .iter()
    .fold(HashMap::new(), fold_minutes_asleep())
    .iter()
    .fold((0usize, (0usize, 0usize)), fold_most_slept_minute());

  minutes.0 * (minutes.1).0
}

#[inline]
fn fold_most_slept_minute() -> impl FnMut((usize, (usize, usize)), (&usize, &Vec<usize>)) -> (usize, (usize, usize)) {
  |(rid, (rm, rc)), (nid, nms)| {
    let most_minute = (0..60).fold((0usize, 0usize), |(r, c), m| {
      let count = nms.iter().filter(|i| **i == m).count(); 
      if count > c { (m, count) } else { (r, c) } 
    });
    if most_minute.1 > rc { 
      (*nid, most_minute)
     } else {
      (rid, (rm, rc))
    }
  } 
}

#[inline]
fn sorted_lines(file: &'static str) -> Vec<&'static str> {
  let mut lines = file.lines()
    .collect::<Vec<&'static str>>();
  lines.sort();
  lines
}

#[inline]
fn enumerate_by_id() -> impl FnMut(&mut Option<usize>, GuardAction) -> Option<Option<(usize, GuardAction)>> {
  |o, a| {
    if let GuardAction::BeginShift(id) = a {
      *o = Some(id);
      Some(None)
    } else if let Some(id) = o {
      Some(Some((*id, a)))
    } else {
      Some(None)
    }
  }
}

#[inline]
fn compile_sleep_times() -> impl FnMut(&mut Option<usize>, (usize, GuardAction)) -> Option<Option<(usize, (usize, usize))>> {
  |o, (id, a)| {
    match a {
      GuardAction::FallAsleep(time) => {
        *o = Some(time);
        Some(None)
      },
      GuardAction::WakeUp(wake) => {
        Some(Some((id, (o.unwrap(), wake))))
      },
      _ => None
    }
  }
}

#[inline]
fn fold_total_minutes_asleep() -> impl FnMut(HashMap<usize, usize>, &(usize, (usize, usize))) -> HashMap<usize, usize> {
  |mut r, &(id, (start, end))| {
    *r.entry(id).or_insert(0) += end - start;
    r
  }
}

#[inline]
fn find_most_slept() -> impl FnMut((usize, usize), (&usize, &usize)) -> (usize, usize) {
  |(rk, rv), (k, v)| {
    if *v > rv {
      (*k, *v)
    } else {
      (rk, rv)
    }
  }
}

#[inline]
fn fold_minutes_asleep() -> impl FnMut(HashMap<usize, Vec<usize>>, &(usize, (usize, usize))) -> HashMap<usize, Vec<usize>> {
  |mut r, &(id, (start, end))| {
    (start..end).for_each(|n| r.entry(id).or_insert_with(Vec::new).push(n));
    r
  }
}

#[derive(Debug, PartialEq)]
enum GuardAction {
  BeginShift(usize),
  FallAsleep(usize),
  WakeUp(usize)
}

impl GuardAction {
  fn from_str(string: &'static str) -> Result<GuardAction, ParseIntError> {
    let minute: usize = string[15..17].parse()?;

    let action = match &string[19..24] {
      "Guard" => GuardAction::BeginShift(string[26..].split(' ').next().unwrap().parse()?),
      "falls" => GuardAction::FallAsleep(minute),
      "wakes" => GuardAction::WakeUp(minute),
      action  => panic!("Invalid guard action: {}.", action)
    };

    Ok(action)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_find_sleepiest_guard() {
    let test_file = include_str!("test_inputs/day_four/simple_schedule.txt");
    let actual = find_id_times_minute(test_file);

    assert_eq!(actual, 240);
  }

  #[test]
  fn can_find_guard_most_asleep() {
    let test_file = include_str!("test_inputs/day_four/simple_schedule.txt");
    let actual = find_id_times_minute_most_asleep(test_file);

    assert_eq!(actual, 4455);
  }

  #[test]
  fn can_parse_schedule() {
    let input = "[1518-09-30 23:58] Guard #9999 begins shift";
    let actual = GuardAction::from_str(input).unwrap();

    assert_eq!(actual, GuardAction::BeginShift(9999));
  }
}