use super::*;

#[test]
fn visible_enemy_is_targetable_without_sight() {
  assert!(is_targetable(false, false));
}

#[test]
fn visible_enemy_is_targetable_with_sight() {
  assert!(is_targetable(false, true));
}

#[test]
fn invisible_enemy_is_not_targetable_without_sight() {
  assert!(!is_targetable(true, false));
}

#[test]
fn invisible_enemy_is_targetable_with_sight() {
  assert!(is_targetable(true, true));
}

#[test]
fn default_targeting_priority_is_first() {
  assert_eq!(TargetingPriority::default(), TargetingPriority::FIRST);
}

#[test]
fn first_has_index_0() {
  assert_eq!(TargetingPriority::FIRST.as_index(), 0);
}

#[test]
fn last_has_index_1() {
  assert_eq!(TargetingPriority::LAST.as_index(), 1);
}

#[test]
fn close_has_index_2() {
  assert_eq!(TargetingPriority::CLOSE.as_index(), 2);
}

#[test]
fn far_has_index_3() {
  assert_eq!(TargetingPriority::FAR.as_index(), 3);
}

#[test]
fn strong_has_index_4() {
  assert_eq!(TargetingPriority::STRONG.as_index(), 4);
}

#[test]
fn weak_has_index_5() {
  assert_eq!(TargetingPriority::WEAK.as_index(), 5);
}

#[test]
fn random_has_index_6() {
  assert_eq!(TargetingPriority::RANDOM.as_index(), 6);
}

#[test]
fn from_index_0_is_first() {
  assert_eq!(TargetingPriority::from_index(0), TargetingPriority::FIRST);
}

#[test]
fn from_index_1_is_last() {
  assert_eq!(TargetingPriority::from_index(1), TargetingPriority::LAST);
}

#[test]
fn from_index_2_is_close() {
  assert_eq!(TargetingPriority::from_index(2), TargetingPriority::CLOSE);
}

#[test]
fn from_index_3_is_far() {
  assert_eq!(TargetingPriority::from_index(3), TargetingPriority::FAR);
}

#[test]
fn from_index_4_is_strong() {
  assert_eq!(TargetingPriority::from_index(4), TargetingPriority::STRONG);
}

#[test]
fn from_index_5_is_weak() {
  assert_eq!(TargetingPriority::from_index(5), TargetingPriority::WEAK);
}

#[test]
fn from_index_6_is_random() {
  assert_eq!(TargetingPriority::from_index(6), TargetingPriority::RANDOM);
}

#[test]
fn from_index_wraps_at_count() {
  let count = TargetingPriority::iter().count();
  assert_eq!(
    TargetingPriority::from_index(count),
    TargetingPriority::FIRST
  );
}

#[test]
fn from_index_wraps_past_count() {
  let count = TargetingPriority::iter().count();
  assert_eq!(
    TargetingPriority::from_index(count + 1),
    TargetingPriority::LAST
  );
}

#[test]
fn next_from_first_is_last() {
  let mut p = TargetingPriority::FIRST;
  p.next_target();
  assert_eq!(p, TargetingPriority::LAST);
}

#[test]
fn next_from_last_is_close() {
  let mut p = TargetingPriority::LAST;
  p.next_target();
  assert_eq!(p, TargetingPriority::CLOSE);
}

#[test]
fn next_from_close_is_far() {
  let mut p = TargetingPriority::CLOSE;
  p.next_target();
  assert_eq!(p, TargetingPriority::FAR);
}

#[test]
fn next_from_far_is_strong() {
  let mut p = TargetingPriority::FAR;
  p.next_target();
  assert_eq!(p, TargetingPriority::STRONG);
}

#[test]
fn next_from_strong_is_weak() {
  let mut p = TargetingPriority::STRONG;
  p.next_target();
  assert_eq!(p, TargetingPriority::WEAK);
}

#[test]
fn next_from_weak_is_random() {
  let mut p = TargetingPriority::WEAK;
  p.next_target();
  assert_eq!(p, TargetingPriority::RANDOM);
}

#[test]
fn next_from_random_wraps_to_first() {
  let mut p = TargetingPriority::RANDOM;
  p.next_target();
  assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn prev_from_random_is_weak() {
  let mut p = TargetingPriority::RANDOM;
  p.prev_target();
  assert_eq!(p, TargetingPriority::WEAK);
}

#[test]
fn prev_from_weak_is_strong() {
  let mut p = TargetingPriority::WEAK;
  p.prev_target();
  assert_eq!(p, TargetingPriority::STRONG);
}

#[test]
fn prev_from_strong_is_far() {
  let mut p = TargetingPriority::STRONG;
  p.prev_target();
  assert_eq!(p, TargetingPriority::FAR);
}

#[test]
fn prev_from_far_is_close() {
  let mut p = TargetingPriority::FAR;
  p.prev_target();
  assert_eq!(p, TargetingPriority::CLOSE);
}

#[test]
fn prev_from_close_is_last() {
  let mut p = TargetingPriority::CLOSE;
  p.prev_target();
  assert_eq!(p, TargetingPriority::LAST);
}

#[test]
fn prev_from_last_is_first() {
  let mut p = TargetingPriority::LAST;
  p.prev_target();
  assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn prev_from_first_wraps_to_random() {
  let mut p = TargetingPriority::FIRST;
  p.prev_target();
  assert_eq!(p, TargetingPriority::RANDOM);
}

#[test]
fn full_next_cycle_returns_to_first() {
  let count = TargetingPriority::iter().count();
  let mut p = TargetingPriority::FIRST;
  for _ in 0..count {
    p.next_target();
  }
  assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn full_prev_cycle_returns_to_first() {
  let count = TargetingPriority::iter().count();
  let mut p = TargetingPriority::FIRST;
  for _ in 0..count {
    p.prev_target();
  }
  assert_eq!(p, TargetingPriority::FIRST);
}
