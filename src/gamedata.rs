pub struct Offsets {
  pub player_controller: usize,
  pub debug_free_cam: usize,
  pub debug_free_cam_bit: usize,
  pub debug_cam_pos: usize,
  pub hero_pawn: usize,
  pub location: usize,
  pub velocity: usize
}

pub trait GameData {
  fn get_offsets(&self) -> Offsets;
  fn is_32_bit(&self) -> bool;
}

pub struct OutlastX64GameData;

impl GameData for OutlastX64GameData {
  fn get_offsets(&self) -> Offsets {
    Offsets {
      player_controller: 0x2020f38,
      debug_free_cam: 0xa84,
      debug_free_cam_bit: 0x100000,
      debug_cam_pos: 0xd50,
      hero_pawn: 0xa4c,
      location: 0x80,
      velocity: 0x18C
    }
  }

  fn is_32_bit(&self) -> bool {
    false
  }
}

pub struct OutlastX86GameData;

impl GameData for OutlastX86GameData {
  fn get_offsets(&self) -> Offsets {
    Offsets {
      player_controller: 0x17e7764,
      debug_free_cam: 0x8a8,
      debug_free_cam_bit: 0x100000,
      debug_cam_pos: 0xb34,
      hero_pawn: 0x88c,
      location: 0x54,
      velocity: 0x13c
    }
  }

  fn is_32_bit(&self) -> bool {
    true
  }
}

pub struct Outlast2GameData;

impl GameData for Outlast2GameData {
  fn get_offsets(&self) -> Offsets {
    Offsets {
      player_controller: 0x219ff58,
      debug_free_cam: 0xc7c,
      debug_free_cam_bit: 0x200,
      debug_cam_pos: 0x10c4,
      hero_pawn: 0xc38,
      location: 0x88,
      velocity: 0x194
    }
  }

  fn is_32_bit(&self) -> bool {
    false
  }
}