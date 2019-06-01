use maat_graphics::imgui::*;

use cgmath::Vector2;

pub struct Logs {
  position: Vector2<f32>,
  size: Vector2<f32>,
  show: bool,
  last_error: String,
}

impl Logs {
  pub fn new(window_size: Vector2<f32>) -> Logs {
    Logs {
      position: window_size*0.5,
      size: Vector2::new(400.0, 200.0),
      show: false,
      last_error: "No Errors".to_string(),
    }
  }
  
  pub fn is_shown(&self) -> bool {
    self.show
  }
  
  pub fn add_error(&mut self, err: String) {
    self.last_error = err;
    self.show = true;
  }
  
  pub fn draw(&mut self, ui: Option<&Ui>) {
    if let Some(ui) = ui {
      ui.window(im_str!("Error"))
            .size((self.size.x, self.size.y), ImGuiCond::FirstUseEver)
            .position((self.position.x, self.position.y), ImGuiCond::FirstUseEver)
            .build(|| {
              ui.text_wrapped(&ImString::new("Error: ".to_owned() + &self.last_error));
              if ui.button(im_str!("Ok"), (0.0, 0.0)) {
                self.show = false;
              }
            });
    }
  }
}
