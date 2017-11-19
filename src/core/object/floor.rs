pub use core::object::{Entity, Tile};

///
/// Hold the current floor, including tiles and entities.
/// 
pub struct Floor {
  pub width: usize,
  pub height: usize,
  pub tile_vec: Vec<Tile>,
  pub entity_vec: Vec<Entity>,
}

impl Floor {

  ///
  /// Return a new `Floor`
  /// 
  #[inline]
  pub fn new(width: usize, height: usize, tile_vec: Vec<Tile>, entity_vec: Vec<Entity>) -> Floor {
    Floor { 
      width: width, 
      height: height, 
      tile_vec: tile_vec, 
      entity_vec: entity_vec
    }
  }

}