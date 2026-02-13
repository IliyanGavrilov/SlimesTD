use super::*;

#[test]
fn test_window_to_world_ndc_conversion_center() {
    let window_size = Vec2::new(1280.0, 720.0);
    let cursor_pos = Vec2::new(640.0, 360.0);
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;

    assert!((ndc.x).abs() < 0.001);
    assert!((ndc.y).abs() < 0.001);
}

#[test]
fn test_window_to_world_ndc_conversion_top_left() {
    let window_size = Vec2::new(1280.0, 720.0);
    let cursor_pos = Vec2::new(0.0, 0.0);
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;

    assert_eq!(ndc, Vec2::new(-1.0, -1.0));
}

#[test]
fn test_window_to_world_ndc_conversion_bottom_right() {
    let window_size = Vec2::new(1280.0, 720.0);
    let cursor_pos = Vec2::new(1280.0, 720.0);
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;

    assert_eq!(ndc, Vec2::new(1.0, 1.0));
}

#[test]
fn test_window_to_world_ndc_conversion_quarter_screen() {
    let window_size = Vec2::new(1280.0, 720.0);
    let cursor_pos = Vec2::new(320.0, 180.0);
    let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;

    assert!((ndc.x - (-0.5)).abs() < 0.001);
    assert!((ndc.y - (-0.5)).abs() < 0.001);
}

#[test]
fn test_ui_boundary_detection() {
    let window_height = 1080.0;
    let node_pos = Vec2::new(960.0, 108.0);
    let node_size = Vec2::new(1920.0, 216.0);

    let half_size = 0.5 * Vec2::new(node_size.x, window_height * 0.20);
    let min = node_pos - half_size;
    let max = node_pos + half_size;

    let cursor_inside = Vec2::new(960.0, 50.0);
    let cursor_outside = Vec2::new(960.0, 500.0);

    assert!((min.x..max.x).contains(&cursor_inside.x));
    assert!((min.y..max.y).contains(&cursor_inside.y));
    assert!(!(min.y..max.y).contains(&cursor_outside.y));
}

#[test]
fn test_tower_placement_distance_threshold() {
    let existing_tower = Vec3::new(100.0, 100.0, 0.0);
    let valid_pos = Vec3::new(150.0, 150.0, 0.0);
    let invalid_pos = Vec3::new(110.0, 110.0, 0.0);

    assert!(Vec3::distance(valid_pos, existing_tower) > 40.0);
    assert!(Vec3::distance(invalid_pos, existing_tower) <= 40.0);
}

#[test]
fn test_sprite_follower_overlap_detection() {
    let follower_pos = Vec3::new(0.0, 0.0, 0.0);
    let tower_within_range = Vec3::new(49.0, 0.0, 0.0);
    let tower_outside_range = Vec3::new(51.0, 0.0, 0.0);

    assert!(Vec3::distance(follower_pos, tower_within_range) <= 50.0);
    assert!(Vec3::distance(follower_pos, tower_outside_range) > 50.0);
}