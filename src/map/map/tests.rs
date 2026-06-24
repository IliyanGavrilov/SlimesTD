#[cfg(test)]
mod tests {
  use crate::{Coordinate, Map, Point, TerrainType, Tile};
  use bevy::math::Vec3;

  fn empty_map(tile_size: usize, width: usize, height: usize) -> Map {
    Map {
      width,
      height,
      tiles: vec![vec![Tile::Grass; width]; height],
      tile_size,
      checkpoints: vec![],
      route_paths: vec![],
      initialized: false,
    }
  }

  fn map_with_path(tile_size: usize, width: usize, height: usize, path_points: &[Point]) -> Map {
    let mut map = empty_map(tile_size, width, height);
    for &p in path_points {
      map.tiles[p.y][p.x] = Tile::Path(vec![0]);
    }
    map
  }

  #[test]
  fn point_to_vec3_maps_components() {
    assert_eq!(Point { x: 3, y: 4 }.to_vec3(), Vec3::new(3.0, 4.0, 0.0));
  }

  #[test]
  fn to_coordinate_scales_x_by_tile_size() {
    assert_eq!(Point { x: 3, y: 4 }.to_coordinate(80, false).x, 240.0);
  }

  #[test]
  fn to_coordinate_scales_y_by_tile_size() {
    assert_eq!(Point { x: 3, y: 4 }.to_coordinate(80, false).y, 320.0);
  }

  #[test]
  fn to_coordinate_centers_y_when_requested() {
    assert_eq!(Point { x: 3, y: 4 }.to_coordinate(80, true).y, 360.0);
  }

  #[test]
  fn to_coordinate_of_origin_is_zero() {
    assert_eq!(Point { x: 0, y: 0 }.to_coordinate(80, false).x, 0.0);
  }

  #[test]
  fn to_coordinate_respects_tile_size() {
    assert_eq!(Point { x: 2, y: 2 }.to_coordinate(10, false).x, 20.0);
  }

  #[test]
  fn horizontal_neighbours_are_adjacent() {
    assert!(Point { x: 1, y: 1 }.is_adjacent_to(Point { x: 2, y: 1 }));
  }

  #[test]
  fn vertical_neighbours_are_adjacent() {
    assert!(Point { x: 1, y: 1 }.is_adjacent_to(Point { x: 1, y: 2 }));
  }

  #[test]
  fn diagonal_points_are_not_adjacent() {
    assert!(!Point { x: 0, y: 0 }.is_adjacent_to(Point { x: 1, y: 1 }));
  }

  #[test]
  fn a_point_is_not_adjacent_to_itself() {
    assert!(!Point { x: 2, y: 2 }.is_adjacent_to(Point { x: 2, y: 2 }));
  }

  #[test]
  fn distant_points_are_not_adjacent() {
    assert!(!Point { x: 0, y: 0 }.is_adjacent_to(Point { x: 3, y: 0 }));
  }

  #[test]
  fn coordinate_to_vec3_maps_components() {
    assert_eq!(
      Coordinate { x: 12.0, y: 34.0 }.to_vec3(),
      Vec3::new(12.0, 34.0, 0.0)
    );
  }

  #[test]
  fn grass_is_grass_terrain() {
    assert_eq!(Tile::Grass.terrain_type(), Some(TerrainType::Grass));
  }

  #[test]
  fn water_is_water_terrain() {
    assert_eq!(Tile::Water.terrain_type(), Some(TerrainType::Water));
  }

  #[test]
  fn path_has_no_terrain() {
    assert_eq!(Tile::Path(vec![0]).terrain_type(), None);
  }

  #[test]
  fn lane_has_no_terrain() {
    assert_eq!(Tile::Lane(vec![1], vec![0]).terrain_type(), None);
  }

  #[test]
  fn spawn_has_no_terrain() {
    assert_eq!(Tile::Spawn.terrain_type(), None);
  }

  #[test]
  fn end_has_no_terrain() {
    assert_eq!(Tile::End.terrain_type(), None);
  }

  #[test]
  fn straight_path_has_spawn_tiles_and_end_checkpoints() {
    let tiles = [
      Point { x: 2, y: 1 },
      Point { x: 3, y: 1 },
      Point { x: 4, y: 1 },
    ];
    let mut map = map_with_path(80, 7, 4, &tiles);
    map.create_checkpoints(tiles.to_vec(), Point { x: 1, y: 1 }, Point { x: 5, y: 1 });
    assert_eq!(map.checkpoints.len(), 5);
  }

  #[test]
  fn straight_path_starts_at_spawn() {
    let tiles = [
      Point { x: 2, y: 1 },
      Point { x: 3, y: 1 },
      Point { x: 4, y: 1 },
    ];
    let mut map = map_with_path(80, 7, 4, &tiles);
    map.create_checkpoints(tiles.to_vec(), Point { x: 1, y: 1 }, Point { x: 5, y: 1 });
    assert_eq!(map.checkpoints[0], Vec3::new(80.0, 80.0, 0.0));
  }

  #[test]
  fn straight_path_ends_at_end() {
    let tiles = [
      Point { x: 2, y: 1 },
      Point { x: 3, y: 1 },
      Point { x: 4, y: 1 },
    ];
    let mut map = map_with_path(80, 7, 4, &tiles);
    map.create_checkpoints(tiles.to_vec(), Point { x: 1, y: 1 }, Point { x: 5, y: 1 });
    assert_eq!(map.checkpoints[4], Vec3::new(400.0, 120.0, 0.0));
  }

  #[test]
  fn turning_path_has_four_checkpoints() {
    let tiles = [Point { x: 2, y: 1 }, Point { x: 2, y: 2 }];
    let mut map = map_with_path(80, 5, 5, &tiles);
    map.create_checkpoints(tiles.to_vec(), Point { x: 1, y: 1 }, Point { x: 2, y: 3 });
    assert_eq!(map.checkpoints.len(), 4);
  }

  #[test]
  fn turning_path_turns_at_the_corner() {
    let tiles = [Point { x: 2, y: 1 }, Point { x: 2, y: 2 }];
    let mut map = map_with_path(80, 5, 5, &tiles);
    map.create_checkpoints(tiles.to_vec(), Point { x: 1, y: 1 }, Point { x: 2, y: 3 });
    assert_eq!(map.checkpoints[2], Vec3::new(160.0, 200.0, 0.0));
  }

  #[test]
  fn empty_path_only_links_spawn_and_end() {
    let mut map = empty_map(80, 4, 4);
    map.create_checkpoints(vec![], Point { x: 1, y: 1 }, Point { x: 2, y: 1 });
    assert_eq!(map.checkpoints.len(), 2);
  }

  #[test]
  fn numbered_path_orders_by_tile_number() {
    let mut map = empty_map(80, 6, 3);
    map.tiles[1][2] = Tile::Path(vec![1]);
    map.tiles[1][3] = Tile::Path(vec![2]);
    let tiles = vec![Point { x: 2, y: 1 }, Point { x: 3, y: 1 }];
    map.create_checkpoints(tiles, Point { x: 1, y: 1 }, Point { x: 4, y: 1 });
    assert_eq!(map.checkpoints[2], Vec3::new(240.0, 120.0, 0.0));
  }

  #[test]
  fn numbered_path_has_spawn_steps_and_end() {
    let mut map = empty_map(80, 6, 3);
    map.tiles[1][2] = Tile::Path(vec![1]);
    map.tiles[1][3] = Tile::Path(vec![2]);
    let tiles = vec![Point { x: 2, y: 1 }, Point { x: 3, y: 1 }];
    map.create_checkpoints(tiles, Point { x: 1, y: 1 }, Point { x: 4, y: 1 });
    assert_eq!(map.checkpoints.len(), 4);
  }

  #[test]
  fn orders_at_reads_a_path_tile() {
    let mut map = empty_map(80, 3, 3);
    map.tiles[1][2] = Tile::Path(vec![0]);
    assert_eq!(map.orders_at(Point { x: 2, y: 1 }), Some(&vec![0]));
  }

  #[test]
  fn orders_at_reads_a_lane_tile() {
    let mut map = empty_map(80, 3, 3);
    map.tiles[1][2] = Tile::Lane(vec![1], vec![3]);
    assert_eq!(map.orders_at(Point { x: 2, y: 1 }), Some(&vec![3]));
  }

  #[test]
  fn orders_at_returns_none_for_grass() {
    let map = empty_map(80, 3, 3);
    assert_eq!(map.orders_at(Point { x: 0, y: 0 }), None);
  }

  #[test]
  fn route_count_of_single_lane_map_is_one() {
    let map = empty_map(80, 3, 3);
    assert_eq!(map.route_count(), 1);
  }

  #[test]
  fn route_count_reflects_lane_count() {
    let mut map = empty_map(80, 3, 3);
    map.route_paths = vec![vec![Vec3::ZERO], vec![Vec3::ONE]];
    assert_eq!(map.route_count(), 2);
  }

  #[test]
  fn route_returns_the_requested_lane() {
    let mut map = empty_map(80, 3, 3);
    map.route_paths = vec![vec![Vec3::ZERO], vec![Vec3::ONE]];
    assert_eq!(map.route(1), &vec![Vec3::ONE]);
  }

  #[test]
  fn route_falls_back_to_main_checkpoints_when_out_of_range() {
    let mut map = empty_map(80, 3, 3);
    map.checkpoints = vec![Vec3::splat(5.0)];
    assert_eq!(map.route(9), &vec![Vec3::splat(5.0)]);
  }
}
