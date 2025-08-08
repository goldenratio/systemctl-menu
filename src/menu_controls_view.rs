use raylib::{
  color::Color,
  prelude::{RaylibDraw, RaylibDrawHandle},
};

pub struct MenuControlsView {
  items: Vec<String>,
  window_size: (i32, i32),
  current_index: i8,
}

const MAX_LABEL_LENGTH: usize = 14;
const DEFAULT_COLOR: Color = Color::RAYWHITE;
const SELECTED_COLOR: Color = Color {
  r: 81,
  g: 162,
  b: 218,
  a: 255,
};

fn truncate_with_ellipsis(val: &str, max_str_length: usize) -> String {
  let chars = val.chars().collect::<Vec<char>>();
  if chars.len() > max_str_length {
    chars[..max_str_length].iter().collect::<String>() + "..."
  } else {
    val.to_string()
  }
}

impl MenuControlsView {
  pub fn new(items: &Vec<&str>, window_size: (i32, i32)) -> Self {
    MenuControlsView {
      items: items
        .iter()
        .map(|s| truncate_with_ellipsis(s, MAX_LABEL_LENGTH))
        .collect::<Vec<String>>(),
      window_size,
      current_index: 0,
    }
  }

  pub fn move_down(&mut self) {
    self.current_index += 1;
    if self.current_index >= self.items.len() as i8 {
      self.current_index = 0;
    }
  }

  pub fn move_up(&mut self) {
    self.current_index -= 1;
    if self.current_index < 0 {
      self.current_index = (self.items.len() as i8) - 1;
    }
  }

  pub fn get_current_index(&self) -> i8 {
    self.current_index
  }

  pub fn update(&self, draw: &mut RaylibDrawHandle) {
    let font_size = 30;
    let spacing = 14;
    let total_height = self.items.len() as i32 * (font_size + spacing);
    let start_y = (self.window_size.1 - total_height) / 2;

    for (i, item) in self.items.iter().enumerate() {
      let text_width = draw.measure_text(item, font_size);
      let x = (self.window_size.0 - text_width) / 2;
      let y = start_y + i as i32 * (font_size + spacing);
      let color = if (i as i8) == self.current_index {
        SELECTED_COLOR
      } else {
        DEFAULT_COLOR
      };
      draw.draw_text(item, x, y, font_size, color);
    }
  }
}
