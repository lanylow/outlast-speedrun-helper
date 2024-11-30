use std::mem::{size_of, zeroed};
use toy_arms::external::{module::Module, read, write};

use crate::gamedata::*;

pub type Vector = [u8; 12];

pub struct GameManager {
  module: Module,
  is_32_bit: bool,
  offsets: Offsets,
  player_controller: usize,
  hero_pawn: usize,
  god_mode: bool
}

impl GameManager {
  pub fn new(module: Module, data: Box<dyn GameData>) -> Self {
    GameManager {
      module,
      is_32_bit: data.is_32_bit(),
      offsets: data.get_offsets(),
      player_controller: 0,
      hero_pawn: 0,
      god_mode: false
    }
  }

  pub fn update_data(&mut self) -> Result<(), ()> {
    self.player_controller = self.read_ptr(self.module.base_address + self.offsets.player_controller)?;
    self.hero_pawn = self.read_ptr(self.player_controller + self.offsets.hero_pawn)?;

    self.write_bit_field(self.player_controller, &self.offsets.god_mode, self.god_mode);

    Ok(())
  }

  pub fn get_location(&self) -> Option<Vector> {
    self.read_type::<Vector>(self.hero_pawn + self.offsets.location).ok()
  }
  
  pub fn set_location(&self, location: &mut Vector) {
    self.write_type(self.hero_pawn + self.offsets.location, location);
    self.write_type(self.hero_pawn + self.offsets.velocity, &mut [0u8; 12]);
  }

  pub fn teleport_to_debug_cam(&self) -> bool {
    let debug_free_cam = self.read_bit_field(self.player_controller, &self.offsets.debug_free_cam).unwrap();

    if debug_free_cam {
      let mut debug_cam_pos = self.read_type::<Vector>(self.player_controller + self.offsets.debug_cam_pos).unwrap();
      self.set_location(&mut debug_cam_pos);
    }

    debug_free_cam
  }

  pub fn toggle_god_mode(&mut self) -> bool {
    self.god_mode ^= true;
    self.god_mode
  }

  fn read_type<T>(&self, address: usize) -> Result<T, ()> {
    let mut value = unsafe { zeroed::<T>() };
    read::<T>(&self.module.process_handle, address, size_of::<T>(), &mut value).map_err(|_| { })?;
    Ok(value)
  }

  fn read_ptr(&self, address: usize) -> Result<usize, ()> {
    let value = if self.is_32_bit {
      self.read_type::<u32>(address)? as usize
    }
    else {
      self.read_type::<u64>(address)? as usize
    };

    Ok(value)
  }

  fn read_bit_field(&self, base: usize, field: &BitField) -> Result<bool, ()> {
    let value = self.read_type::<u32>(base + field.offset)? & field.mask;
    Ok(value != 0)
  }

  fn write_type<T>(&self, address: usize, value: &mut T) {
    let _ = write::<T>(&self.module.process_handle, address, value);
  }

  fn write_bit_field(&self, base: usize, field: &BitField, enable: bool) {
    let mut value = self.read_type::<u32>(base + field.offset).unwrap();

    if enable {
      value |= field.mask;
    }
    else {
      value &= !field.mask;
    }

    self.write_type::<u32>(base + field.offset, &mut value);
  }
}