use super::*;

#[test]
fn test_targeting_priority_default() {
    assert_eq!(TargetingPriority::default(), TargetingPriority::FIRST);
}

#[test]
fn test_targeting_priority_as_index() {
    assert_eq!(TargetingPriority::FIRST.as_index(), 0);
    assert_eq!(TargetingPriority::LAST.as_index(), 1);
    assert_eq!(TargetingPriority::CLOSE.as_index(), 2);
    assert_eq!(TargetingPriority::FAR.as_index(), 3);
    assert_eq!(TargetingPriority::STRONG.as_index(), 4);
    assert_eq!(TargetingPriority::WEAK.as_index(), 5);
    assert_eq!(TargetingPriority::RANDOM.as_index(), 6);
}

#[test]
fn test_targeting_priority_from_index() {
    assert_eq!(TargetingPriority::from_index(0), TargetingPriority::FIRST);
    assert_eq!(TargetingPriority::from_index(1), TargetingPriority::LAST);
    assert_eq!(TargetingPriority::from_index(2), TargetingPriority::CLOSE);
    assert_eq!(TargetingPriority::from_index(3), TargetingPriority::FAR);
    assert_eq!(TargetingPriority::from_index(4), TargetingPriority::STRONG);
    assert_eq!(TargetingPriority::from_index(5), TargetingPriority::WEAK);
    assert_eq!(TargetingPriority::from_index(6), TargetingPriority::RANDOM);
}

#[test]
fn test_targeting_priority_from_index_wraps() {
    let count = TargetingPriority::iter().count();
    assert_eq!(TargetingPriority::from_index(count), TargetingPriority::FIRST);
    assert_eq!(TargetingPriority::from_index(count + 1), TargetingPriority::LAST);
}

#[test]
fn test_targeting_priority_next_target_sequence() {
    let mut p = TargetingPriority::FIRST;

    p.next_target();
    assert_eq!(p, TargetingPriority::LAST);

    p.next_target();
    assert_eq!(p, TargetingPriority::CLOSE);

    p.next_target();
    assert_eq!(p, TargetingPriority::FAR);

    p.next_target();
    assert_eq!(p, TargetingPriority::STRONG);

    p.next_target();
    assert_eq!(p, TargetingPriority::WEAK);

    p.next_target();
    assert_eq!(p, TargetingPriority::RANDOM);
}

#[test]
fn test_targeting_priority_next_target_wraps() {
    let mut p = TargetingPriority::RANDOM;
    p.next_target();
    assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn test_targeting_priority_prev_target_sequence() {
    let mut p = TargetingPriority::RANDOM;

    p.prev_target();
    assert_eq!(p, TargetingPriority::WEAK);

    p.prev_target();
    assert_eq!(p, TargetingPriority::STRONG);

    p.prev_target();
    assert_eq!(p, TargetingPriority::FAR);

    p.prev_target();
    assert_eq!(p, TargetingPriority::CLOSE);

    p.prev_target();
    assert_eq!(p, TargetingPriority::LAST);

    p.prev_target();
    assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn test_targeting_priority_prev_target_wraps() {
    let mut p = TargetingPriority::FIRST;
    p.prev_target();
    assert_eq!(p, TargetingPriority::RANDOM);
}

#[test]
fn test_targeting_priority_cycling() {
    let mut p = TargetingPriority::FIRST;

    p.next_target();
    p.prev_target();
    assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn test_targeting_priority_full_cycle_next() {
    let count = TargetingPriority::iter().count();
    let mut p = TargetingPriority::FIRST;

    for _ in 0..count {
        p.next_target();
    }

    assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn test_targeting_priority_full_cycle_prev() {
    let count = TargetingPriority::iter().count();
    let mut p = TargetingPriority::FIRST;

    for _ in 0..count {
        p.prev_target();
    }

    assert_eq!(p, TargetingPriority::FIRST);
}

#[test]
fn test_targeting_logic_strong_weak_selection() {
    let weak_enemy_health = 10;
    let strong_enemy_health = 100;

    let enemies = vec![
        (weak_enemy_health, 50.0),
        (strong_enemy_health, 50.0),
    ];

    let strong_target = enemies.iter().max_by_key(|(hp, _)| *hp);
    let weak_target = enemies.iter().min_by_key(|(hp, _)| *hp);

    assert_eq!(strong_target.unwrap().0, 100);
    assert_eq!(weak_target.unwrap().0, 10);
}

#[test]
fn test_targeting_logic_first_last_selection() {
    let dist_near = 10.0;
    let dist_far = 500.0;

    let distances = vec![dist_near, dist_far];

    let first_target = distances.iter().max_by_key(|dist| FloatOrd(**dist));
    let last_target = distances.iter().min_by_key(|dist| FloatOrd(**dist));

    assert_eq!(*first_target.unwrap(), 500.0);
    assert_eq!(*last_target.unwrap(), 10.0);
}