use std::process::Command;

use menu_controls_view::MenuControlsView;
use raylib::prelude::*;

mod menu_controls_view;

#[derive(Debug)]
struct MenuItem {
  label: String,
  command: String,
}

fn main() {
  let window_size = (400, 300);

  let (mut rl, thread) = raylib::init()
    .size(window_size.0, window_size.1)
    .undecorated()
    .build();

  // ConfigFlags::FLAG_WINDOW_TOPMOST
  rl.set_window_state(rl.get_window_state().set_window_topmost(true));

  rl.set_target_fps(60);

  let items = [
    MenuItem {
      label: "Power Off".to_owned(),
      command: "systemctl poweroff".to_owned(),
    },
    MenuItem {
      label: "Restart".to_owned(),
      command: "systemctl reboot".to_owned(),
    },
    MenuItem {
      label: "Suspend".to_owned(),
      command: "systemctl suspend".to_owned(),
    },
  ];

  let labels = items
    .iter()
    .map(|item| item.label.as_str())
    .collect::<Vec<_>>();

  let mut menu_view = MenuControlsView::new(&labels, window_size);

  while !rl.window_should_close() {
    // update logic
    if rl.is_key_pressed(KeyboardKey::KEY_DOWN) {
      menu_view.move_down();
    } else if rl.is_key_pressed(KeyboardKey::KEY_UP) {
      menu_view.move_up();
    } else if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
      if let Some(menu_item) = items.get(menu_view.get_current_index() as usize) {
        Command::new("sh")
          .arg("-c")
          .arg(&menu_item.command)
          .output()
          .expect("Failed to execute action command!");

        // Exit the loop and close the Raylib window
        break;
      }
    } else if rl.is_key_pressed(KeyboardKey::KEY_Q) {
      break;
    }

    // Draw
    let mut d = rl.begin_drawing(&thread);
    d.clear_background(Color::BLACK);
    menu_view.update(&mut d);
  }
}
