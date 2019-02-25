use core::world::dungeon::map::{self, Tile};

use super::AI;
use core::creature::{Actions, Creature, Actor, Stats};

///
/// PlayerAI does nothing
///
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct PlayerAI;

impl PlayerAI {
  pub fn new() -> Self {
    PlayerAI {}
  }
}

impl AI for PlayerAI {
  
  ///
  /// Do nothing
  ///
  fn take_turn(&mut self, _map: &map::Grid<Tile>, _player: &Creature, _me: &mut Actor, _stats: &mut Stats) -> Actions {
    Actions::Unknown
  }

  ///
  /// Allow Box<AI> cloning
  ///
  fn box_clone(&self) -> Box<AI> {
    Box::new((*self).clone())
  }

}