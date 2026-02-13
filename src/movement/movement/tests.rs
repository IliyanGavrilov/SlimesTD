#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use bevy::prelude::Transform;
    use crate::Movement;

    #[test]
    fn test_movement_math_logic() {
        let mut movement = Movement::new(Vec3::new(10.0, 0.0, 0.0), 100.0);
        let mut transform = Transform::from_translation(Vec3::ZERO);
        let delta_seconds = 0.1;

        let distance_vec = movement.direction.normalize() * movement.speed * delta_seconds;
        movement.distance_travelled += distance_vec.length();
        transform.translation += distance_vec;

        assert_eq!(transform.translation.x, 10.0);
        assert_eq!(movement.distance_travelled, 10.0);
    }

    #[test]
    fn test_movement_diagonal_normalization() {
        let movement = Movement::new(Vec3::new(1.0, 1.0, 0.0), 100.0);
        let distance_vec = movement.direction.normalize() * movement.speed * 1.0;

        assert!((distance_vec.length() - 100.0).abs() < 0.1);
        assert!(distance_vec.x < 100.0);
    }
}