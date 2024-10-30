use std::mem::{size_of, zeroed};
use toy_arms::external::{module::Module, read, write};

use crate::gamedata::GameData;

pub type Vector = [u8; 12];

pub struct GameManager {
  module: Module,
  data: Box<dyn GameData>
}

impl GameManager {
  pub fn new(module: Module, data: Box<dyn GameData>) -> Self {
    GameManager {
      module: module,
      data: data
    }
  }

  fn read_type<T>(&self, address: usize) -> Result<T, ()> {
    let mut value = unsafe { zeroed::<T>() };
    read::<T>(&self.module.process_handle, address, size_of::<T>(), &mut value).map_err(|_| { })?;
    Ok(value)
  }
  
  fn write_type<T>(&self, address: usize, value: &mut T) {
    let _ = write::<T>(&self.module.process_handle, address, value);
  }
  
  pub fn get_olpc_ptr(&mut self) -> Result<usize, ()> {
    let ptr = self.module.find_pattern(self.data.get_olpc_pattern()).ok_or(())?;
    let offset = self.read_type::<u32>(self.module.base_address + ptr + 3)? as usize;
    Ok(ptr + offset + 7)
  }

  pub fn get_hero_pawn(&self, ptr: usize) -> Result<usize, ()> {
    let olpc = self.read_type::<usize>(self.module.base_address + ptr)?;
    let hero_pawn = self.read_type::<usize>(olpc + self.data.get_offsets().hero_pawn)?;
    Ok(hero_pawn)
  }

  pub fn get_location(&self, hero_pawn: usize) -> Option<Vector> {
    self.read_type::<Vector>(hero_pawn + self.data.get_offsets().location).ok()
  }
  
  pub fn set_location(&self, hero_pawn: usize, location: &mut Vector) {
    let mut zero = [0u8; 12];
    let offsets = self.data.get_offsets();

    self.write_type(hero_pawn + offsets.location, location);
    self.write_type(hero_pawn + offsets.velocity, &mut zero);
  }
}