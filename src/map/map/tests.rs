#[cfg(test)]
mod tests {
    use bevy::math::Vec3;
    use crate::{Map, Point, Tile};

    fn make_map(tile_size: usize) -> Map {
        Map {
            width: 0,
            height: 0,
            tiles: vec![],
            tile_size,
            checkpoints: vec![],
        }
    }

    fn make_map_with_tiles(tile_size: usize, width: usize, height: usize, path_points: &[Point]) -> Map {
        let mut tiles = vec![vec![Tile::Grass; width]; height];
        for &p in path_points {
            tiles[p.y][p.x] = Tile::Path(vec![0]);
        }
        Map {
            width,
            height,
            tiles,
            tile_size,
            checkpoints: vec![],
        }
    }


    #[test]
    fn test_point_to_coordinate_no_center() {
        let point = Point { x: 3, y: 4 };
        let coord = point.to_coordinate(80, false);
        assert_eq!(coord.x, 240.0);
        assert_eq!(coord.y, 320.0);
    }

    #[test]
    fn test_point_to_coordinate_with_center() {
        let point = Point { x: 3, y: 4 };
        let coord = point.to_coordinate(80, true);
        assert_eq!(coord.x, 240.0);
        assert_eq!(coord.y, 360.0);
    }

    #[test]
    fn test_point_not_adjacent_diagonal() {
        let a = Point { x: 0, y: 0 };
        let b = Point { x: 1, y: 1 };
        assert!(!a.is_adjacent_to(b));
    }

    #[test]
    fn test_point_not_adjacent_same() {
        let a = Point { x: 2, y: 2 };
        assert!(!a.is_adjacent_to(a));
    }

    #[test]
    fn test_point_adjacency() {
        let p1 = Point { x: 1, y: 1 };
        let p2 = Point { x: 1, y: 2 };
        let p3 = Point { x: 2, y: 2 };
        assert!(p1.is_adjacent_to(p2));
        assert!(p2.is_adjacent_to(p1));
        assert!(p2.is_adjacent_to(p3));
        assert!(!p1.is_adjacent_to(p3));
    }

    #[test]
    fn test_create_checkpoints_straight_path() {
        let path_tiles = vec![
            Point { x: 2, y: 1 },
            Point { x: 3, y: 1 },
            Point { x: 4, y: 1 },
        ];
        let mut map = make_map_with_tiles(80, 7, 4, &path_tiles);
        let spawn = Point { x: 1, y: 1 };
        let end = Point { x: 5, y: 1 };
        map.create_checkpoints(path_tiles, spawn, end);
        assert_eq!(map.checkpoints.len(), 5);
        assert_eq!(map.checkpoints[0], Vec3::new(80.0, 80.0, 0.0));
        assert_eq!(map.checkpoints[1], Vec3::new(160.0, 120.0, 0.0));
        assert_eq!(map.checkpoints[4], Vec3::new(400.0, 120.0, 0.0));
    }

    #[test]
    fn test_create_checkpoints_no_path_tiles() {
        let mut map = make_map_with_tiles(80, 4, 4, &[]);
        let spawn = Point { x: 1, y: 1 };
        let end = Point { x: 2, y: 1 };
        map.create_checkpoints(vec![], spawn, end);
        assert_eq!(map.checkpoints.len(), 2);
        assert_eq!(map.checkpoints[0], Vec3::new(80.0, 80.0, 0.0));
        assert_eq!(map.checkpoints[1], Vec3::new(160.0, 120.0, 0.0));
    }

    #[test]
    fn test_create_checkpoints_unordered_path_tiles() {
        let path_tiles = vec![
            Point { x: 3, y: 1 },
            Point { x: 2, y: 1 },
        ];
        let mut map = make_map_with_tiles(80, 6, 4, &path_tiles);
        let spawn = Point { x: 1, y: 1 };
        let end = Point { x: 4, y: 1 };
        map.create_checkpoints(path_tiles, spawn, end);
        assert_eq!(map.checkpoints.len(), 4);
        assert_eq!(map.checkpoints[1], Vec3::new(160.0, 120.0, 0.0));
        assert_eq!(map.checkpoints[2], Vec3::new(240.0, 120.0, 0.0));
    }

    #[test]
    fn test_create_checkpoints_with_turn() {
        let path_tiles = vec![
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ];
        let mut map = make_map_with_tiles(80, 5, 5, &path_tiles);
        let spawn = Point { x: 1, y: 1 };
        let end = Point { x: 2, y: 3 };
        map.create_checkpoints(path_tiles, spawn, end);
        assert_eq!(map.checkpoints.len(), 4);
        assert_eq!(map.checkpoints[0], Vec3::new(80.0, 80.0, 0.0));
        assert_eq!(map.checkpoints[1], Vec3::new(160.0, 120.0, 0.0));
        assert_eq!(map.checkpoints[2], Vec3::new(160.0, 200.0, 0.0));
        assert_eq!(map.checkpoints[3], Vec3::new(160.0, 280.0, 0.0));
    }

    #[test]
    fn test_create_checkpoints_different_tile_size() {
        let path_tiles = vec![
            Point { x: 2, y: 1 },
            Point { x: 2, y: 2 },
        ];
        let mut map = make_map_with_tiles(10, 5, 5, &path_tiles);
        let spawn = Point { x: 1, y: 1 };
        let end = Point { x: 3, y: 2 };
        map.create_checkpoints(path_tiles, spawn, end);
        assert_eq!(map.checkpoints.len(), 4);
        assert_eq!(map.checkpoints[1], Vec3::new(20.0, 15.0, 0.0));
        assert_eq!(map.checkpoints[3], Vec3::new(30.0, 25.0, 0.0));
    }
}