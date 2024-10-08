use std::{mem::{size_of, zeroed}, thread, time::Duration};
use toy_arms::{external::{module::Module, process::Process, read, write}, utils::keyboard::{self, VirtualKeyCode}};

const PLAYER_CONTROLLER_PATTERN: &str = "48 8B 05 ? ? ? ? 4C 8B D1 48 85";

fn read_type<T>(module: &Module, address: usize) -> Result<T, ()> {
  let mut value = unsafe { zeroed::<T>() };
  read::<T>(&module.process_handle, address, size_of::<T>(), &mut value).map_err(|_| { })?;
  Ok(value)
}

fn write_type<T>(module: &Module, address: usize, value: &mut T) {
  let _ = write::<T>(&module.process_handle, address, value);
}

fn get_player_controller_ptr(module: &mut Module) -> Result<usize, ()> {
  let ptr = module.find_pattern(PLAYER_CONTROLLER_PATTERN).ok_or(())?;
  let offset = read_type::<u32>(module, module.base_address + ptr + 3)? as usize;
  Ok(ptr + offset + 7)
}

fn get_ol_hero(module: &Module, ptr: usize) -> Result<usize, ()> {
  let o1 = read_type::<usize>(module, module.base_address + ptr)?;
  let o2 = read_type::<usize>(module, o1 + 0xa4c)?;
  Ok(o2)
}

fn get_location(module: &Module, hero: usize) -> Option<[u8; 12]> {
  read_type::<[u8; 12]>(&module, hero + 0x80).ok()
}

fn set_location(module: &Module, hero: usize, location: &mut [u8; 12]) {
  write_type(&module, hero + 0x80, location);

  let mut vec_zero = [0u8; 12];
  write_type(&module, hero + 0x18C, &mut vec_zero);
}

fn run() -> Result<(), ()> {
  println!("Outlast Speedrun Helper by lanylow");

  let process = Process::from_process_name("OLGame.exe").map_err(|_| {
    println!("ERROR: the game is not running, please open it first")
  })?;

  let mut module = process.get_module_info("OLGame.exe").map_err(|_| {
    println!("ERROR: failed to get module info")
  })?;

  let controller_ptr = get_player_controller_ptr(&mut module).map_err(|_| {
    println!("ERROR: failed to find the player controller")
  })?;

  println!("Use hotkeys CTRL + F1-F4 to store positions");
  println!("Use hotkeys F1-F4 to restore positions");
  println!("Use hotkey END to exit");

  let mut saved_positions: [Option<[u8; 12]>; 4] = [None; 4];

  loop {
    let hero = match get_ol_hero(&module, controller_ptr) {
      Ok(x) => x,
      Err(_) => continue
    };

    for i in 0usize..4usize {
      if !keyboard::detect_keypress(VirtualKeyCode::VK_F1 + i as i32) {
        continue;
      }

      if unsafe { keyboard::GetAsyncKeyState(VirtualKeyCode::VK_CONTROL) } != 0 {
        if let Some(pos) = get_location(&module, hero) {
          saved_positions[i] = Some(pos);
          println!("Position {} saved", i + 1);
          win_beep::beep_with_hz_and_millis(800, 200);
        }
      }
      else {
        if let Some(mut pos) = saved_positions[i as usize] {
          set_location(&module, hero, &mut pos);
          println!("Position {} restored", i + 1);
          win_beep::beep_with_hz_and_millis(500, 200);
        }
      }
    }

    if keyboard::detect_keypress(VirtualKeyCode::VK_END) {
      break;
    }

    thread::sleep(Duration::from_millis(50));
  }

  Ok(())
}

fn main() -> Result<(), ()> {
  run().map_err(|_| {
    thread::sleep(Duration::from_secs(5))
  })?;

  Ok(())
}