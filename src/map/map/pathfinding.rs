use bevy::prelude::*;

use super::{Coordinate, Map, Point, Tile};

/// Route construction for a map: turning the tile grid into the ordered world-space
/// checkpoints enemies walk along. Kept separate from the tile model, rendering and
/// enemy-following systems in `map.rs`.
impl Map {
  pub(super) fn create_checkpoints(&mut self, path_tiles: Vec<Point>, spawn: Point, end: Point) {
    self.checkpoints = self.build_route(path_tiles, spawn, end);
  }

  /// Spawn entry → each path tile in traversal order → end, all in world space. The
  /// ordering strategy is chosen by `ordered_path_tiles`; this method only turns the
  /// resulting tiles into centred coordinates, so there's a single place that does it.
  pub(super) fn build_route(&self, path_tiles: Vec<Point>, spawn: Point, end: Point) -> Vec<Vec3> {
    let ordered = self.ordered_path_tiles(path_tiles, spawn);

    let mut checkpoints = Vec::with_capacity(ordered.len() + 2);
    checkpoints.push(self.spawn_entry(spawn).to_vec3());
    checkpoints.extend(ordered.iter().map(|p| self.tile_centre(*p)));
    checkpoints.push(self.tile_centre(end));
    checkpoints
  }

  /// The path tiles in traversal order. Maps that number their tiles use that order
  /// (which allows a route to cross itself); otherwise we walk by adjacency from spawn.
  fn ordered_path_tiles(&self, path_tiles: Vec<Point>, spawn: Point) -> Vec<Point> {
    if self.has_numbered_path(&path_tiles) {
      self.tiles_by_order(&path_tiles)
    } else {
      self.tiles_by_adjacency(path_tiles, spawn)
    }
  }

  /// The order list of a path tile, whichever lane it belongs to.
  pub(super) fn orders_at(&self, p: Point) -> Option<&Vec<usize>> {
    match &self.tiles[p.y][p.x] {
      Tile::Path(orders) | Tile::Lane(_, orders) => Some(orders),
      _ => None,
    }
  }

  /// Whether any path tile carries an explicit order (> 0), which lets a route cross
  /// itself instead of relying on adjacency.
  fn has_numbered_path(&self, path_tiles: &[Point]) -> bool {
    path_tiles.iter().any(|&p| {
      self
        .orders_at(p)
        .is_some_and(|o| o.iter().any(|&order| order > 0))
    })
  }

  /// Tiles sorted by their explicit order number (0, 1, 2, …).
  fn tiles_by_order(&self, path_tiles: &[Point]) -> Vec<Point> {
    let max_order = path_tiles
      .iter()
      .filter_map(|&p| self.orders_at(p).and_then(|o| o.iter().max().copied()))
      .max()
      .unwrap_or(0);

    (0..=max_order)
      .filter_map(|i| {
        path_tiles
          .iter()
          .copied()
          .find(|&p| self.orders_at(p).is_some_and(|o| o.contains(&i)))
      })
      .collect()
  }

  /// Tiles walked in order by repeatedly stepping to an adjacent unused tile, starting
  /// from the spawn. Suits simple, non-crossing paths.
  fn tiles_by_adjacency(&self, mut remaining: Vec<Point>, spawn: Point) -> Vec<Point> {
    let mut ordered = Vec::with_capacity(remaining.len());
    let mut last = spawn;
    while let Some(idx) = remaining.iter().position(|p| last.is_adjacent_to(*p)) {
      last = remaining.remove(idx);
      ordered.push(last);
    }
    ordered
  }

  /// The spawn point pushed two tiles off the nearest edge, so enemies walk in from
  /// off-screen rather than popping in at the border.
  fn spawn_entry(&self, spawn: Point) -> Coordinate {
    let offset_distance = (self.tile_size * 2) as f32;
    let mut coord = spawn.to_coordinate(self.tile_size, false);
    if spawn.y == 0 {
      coord.y -= offset_distance;
    } else if spawn.y == self.height - 1 {
      coord.y += offset_distance;
    } else if spawn.x == 0 {
      coord.x -= offset_distance;
    } else if spawn.x == self.width - 1 {
      coord.x += offset_distance;
    }
    coord
  }

  /// World-space centre of a tile (where a checkpoint sits).
  fn tile_centre(&self, tile: Point) -> Vec3 {
    tile.to_coordinate(self.tile_size, true).to_vec3()
  }
}
