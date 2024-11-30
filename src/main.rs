use std::{thread, time::Duration};
use toy_arms::{external::process::Process, utils::keyboard::VirtualKeyCode};
use toy_arms::utils::detect_keydown;
use toy_arms::utils::keyboard::detect_keypress;
use gamedata::*;
use gamemanager::*;

mod gamedata;
mod gamemanager;

fn find_game_process() -> Option<GameManager> {
  for name in vec!["OLGame.exe", "Outlast2.exe"] {
    if let Ok(process) = Process::from_process_name(name) {
      let module = process.get_module_info(name).unwrap();

      let data: Box<dyn GameData> = match name {
        "OLGame.exe" => if process.is_wow64 > 0 { Box::new(OutlastX86GameData) } else { Box::new(OutlastX64GameData) },
        "Outlast2.exe" => Box::new(Outlast2GameData),
        _ => unreachable!()
      };

      return Some(GameManager::new(module, data));
    }
  }

  None
}

fn run() -> Result<(), ()> {
  println!("Outlast Speedrun Helper by lanylow");

  let mut game_manager = match find_game_process() {
    Some(v) => v,
    None => {
      println!("ERROR: the game is not running, please open it first");
      return Err(());
    }
  };

  println!("Use CTRL + F1-F4 to store positions");
  println!("Use F1-F4 to restore positions");
  println!("Use F5 to teleport the character to freecam");
  println!("Use F6 to toggle god mode");
  println!("Use END to exit");

  let mut saved_positions: [Option<Vector>; 4] = [None; 4];

  loop {
    if detect_keypress(VirtualKeyCode::VK_END) {
      break;
    }

    if game_manager.update_data().is_err() {
      continue;
    };

    for i in 0usize..4usize {
      if !detect_keypress(VirtualKeyCode::VK_F1 + i as i32) {
        continue;
      }

      if detect_keydown!(VirtualKeyCode::VK_CONTROL) {
        if let Some(pos) = game_manager.get_location() {
          saved_positions[i] = Some(pos);
          win_beep::beep_with_hz_and_millis(800, 200);
        }
      }
      else {
        if let Some(mut pos) = saved_positions[i as usize] {
          game_manager.set_location(&mut pos);
          win_beep::beep_with_hz_and_millis(500, 200);
        }
      }
    }

    if detect_keypress(VirtualKeyCode::VK_F5) {
      if game_manager.teleport_to_debug_cam() {
        win_beep::beep_with_hz_and_millis(200, 200);
      }
    }

    if detect_keypress(VirtualKeyCode::VK_F6) {
      let enabled = game_manager.toggle_god_mode();
      win_beep::beep_with_hz_and_millis(if enabled { 800 } else { 500 }, 200);
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