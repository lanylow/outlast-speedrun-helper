use std::{thread, time::Duration};
use gamedata::{GameData, Outlast2GameData, OutlastGameData};
use gamemanager::{GameManager, Vector};
use toy_arms::{external::process::Process, utils::keyboard::{self, VirtualKeyCode}};

mod gamedata;
mod gamemanager;

fn find_game_process() -> Option<GameManager> {
  for name in vec!["OLGame.exe", "Outlast2.exe"] {
    if let Ok(process) = Process::from_process_name(name) {
      let module = process.get_module_info(name).unwrap();

      let data: Box<dyn GameData> = match name {
        "OLGame.exe" => Box::new(OutlastGameData),
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

  let olpc_ptr = game_manager.get_olpc_ptr().map_err(|_| {
    println!("ERROR: failed to find the player controller")
  })?;

  println!("Use hotkeys CTRL + F1-F4 to store positions");
  println!("Use hotkeys F1-F4 to restore positions");
  println!("Use hotkey END to exit");

  let mut saved_positions: [Option<Vector>; 4] = [None; 4];

  loop {
    let hero_pawn = match game_manager.get_hero_pawn(olpc_ptr) {
      Ok(x) => x,
      Err(_) => continue
    };

    for i in 0usize..4usize {
      if !keyboard::detect_keypress(VirtualKeyCode::VK_F1 + i as i32) {
        continue;
      }

      if unsafe { keyboard::GetAsyncKeyState(VirtualKeyCode::VK_CONTROL) } != 0 {
        if let Some(pos) = game_manager.get_location(hero_pawn) {
          saved_positions[i] = Some(pos);
          println!("Position {} saved", i + 1);
          win_beep::beep_with_hz_and_millis(800, 200);
        }
      }
      else {
        if let Some(mut pos) = saved_positions[i as usize] {
          game_manager.set_location(hero_pawn, &mut pos);
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