pub struct Offsets {
  pub hero_pawn: usize,
  pub location: usize,
  pub velocity: usize
}

pub trait GameData {
  fn get_offsets(&self) -> Offsets;
  fn get_olpc_pattern(&self) -> &str;
}

pub struct OutlastGameData;

impl GameData for OutlastGameData {
  fn get_offsets(&self) -> Offsets {
    Offsets {
      hero_pawn: 0xa4c,
      location: 0x80,
      velocity: 0x18C
    }
  }

  fn get_olpc_pattern(&self) -> &str {
    "48 8B 05 ? ? ? ? 4C 8B D1 48 85"
  }
}

pub struct Outlast2GameData;

impl GameData for Outlast2GameData {
  fn get_offsets(&self) -> Offsets {
    Offsets {
      hero_pawn: 0xc38,
      location: 0x88,
      velocity: 0x194
    }
  }

  fn get_olpc_pattern(&self) -> &str {
    "48 8B 15 ? ? ? ? 48 85 D2 74 09 4C"
  }
}