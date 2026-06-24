use super::*;

#[test]
fn new_stores_direction() {
  assert_eq!(
    Movement::new(Vec3::new(1.0, 0.0, 0.0), 100.0).direction,
    Vec3::new(1.0, 0.0, 0.0)
  );
}

#[test]
fn new_stores_speed() {
  assert_eq!(Movement::new(Vec3::new(1.0, 0.0, 0.0), 100.0).speed, 100.0);
}

#[test]
fn new_starts_with_no_distance_travelled() {
  assert_eq!(
    Movement::new(Vec3::new(1.0, 0.0, 0.0), 100.0).distance_travelled,
    0.0
  );
}

#[test]
fn displacement_moves_along_the_axis() {
  let movement = Movement::new(Vec3::new(1.0, 0.0, 0.0), 100.0);
  assert_eq!(movement.displacement(0.1).x, 10.0);
}

#[test]
fn displacement_length_is_speed_times_delta() {
  let movement = Movement::new(Vec3::new(1.0, 0.0, 0.0), 100.0);
  assert_eq!(movement.displacement(0.5).length(), 50.0);
}

#[test]
fn diagonal_displacement_is_normalized() {
  let movement = Movement::new(Vec3::new(1.0, 1.0, 0.0), 100.0);
  assert!((movement.displacement(1.0).length() - 100.0).abs() < 0.001);
}
