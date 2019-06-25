use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use crate::modules::Logs;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct LightObject {
  reference_num: u32,
  name: String,
  
  position: Vector3<f32>,
  colour: Vector3<f32>,
  intensity: f32,
}

impl LightObject {
  pub fn new_on(reference_num: u32, name: String) -> LightObject {
    LightObject {
      reference_num,
      name,
      
      position: Vector3::new(0.0, 0.0, 0.0),
      colour: Vector3::new(1.0, 1.0, 1.0),
      intensity: 100.0,
    }
  }
  
  pub fn id(&self) -> u32 {
    self.reference_num
  }
  
  pub fn name(&self) -> String {
    self.name.to_string()
  }
  
  pub fn position(&self) -> Vector3<f32> {
    self.position
  }
  
  pub fn set_position(&mut self, pos: Vector3<f32>) {
    self.position = pos;
  }
  
  pub fn update(&mut self, ui: Option<&Ui>,window_dim: Vector2<f32>, _delta_time: f32, logs: &mut Logs) {
     if let Some(ui) = &ui {
        ui.window(im_str!("Light Options"))
            .always_auto_resize(true)
            .size((200.0, 200.0), ImGuiCond::FirstUseEver)
            .position((window_dim.x - 500.0, 200.0), ImGuiCond::FirstUseEver)
            .build(|| {
              ui.new_line();
              ui.text(im_str!("Position"));
              
              ui.columns(3, im_str!("x | y | z"), true);
              ui.text(im_str!("x:"));
              ui.same_line(0.0);
              ui.input_float(im_str!("##x"), &mut self.position.x).build();
              ui.next_column();
              ui.text(im_str!("y:"));
              ui.same_line(0.0);
              ui.input_float(im_str!("##y"), &mut self.position.y).build();
              ui.next_column();
              ui.text(im_str!("z:"));
              ui.same_line(0.0);
              ui.input_float(im_str!("##z"), &mut self.position.z).build();
              ui.columns(1, im_str!(""), false);
              ui.new_line();
              ui.text(im_str!("Intensity:"));
              ui.same_line(0.0);
              ui.slider_float(im_str!(""), &mut self.intensity, 0.1, 1000.0).build();
              ui.new_line();
              ui.tree_node(im_str!("Light Colour")).build(|| {
                let mut colour = [self.colour.x, self.colour.y, self.colour.z];
                ui.color_picker(im_str!("Colour"), &mut colour).build();
                self.colour = Vector3::new(colour[0], colour[1], colour[2]);
              });
        });
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
     draw_calls.push(DrawCall::set_light(self.position, self.colour, self.intensity));
  }
}
