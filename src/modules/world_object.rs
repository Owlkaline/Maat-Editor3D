use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use cgmath::Vector3;

#[derive(Clone)]
pub struct WorldObject {
  reference_num: u32,
  model: String,
  position: Vector3<f32>,
  rotation: Vector3<f32>,
  size: Vector3<f32>,
}

impl WorldObject {
  pub fn new_empty(reference_num: u32, model: String) -> WorldObject {
    WorldObject {
      reference_num,
      model,
      position: Vector3::new(0.0, 0.0, 0.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      size: Vector3::new(1.0, 1.0, 1.0),
    }
  }
  
  pub fn id(&self) -> u32 {
    self.reference_num
  }
  
  pub fn set_position(&mut self, pos: Vector3<f32>) {
    self.position = pos;
  }
  
  pub fn update(&mut self, ui: Option<&Ui>, _delta_time: f32) {
     if let Some(ui) = &ui {
       ui.window(im_str!("Object Being Placed"))
       .size((300.0, 300.0), ImGuiCond::FirstUseEver)
       .build(|| {
          ui.text(im_str!(
            "Position: ({:.1},{:.1},{:.1})",
            self.position.x,
            self.position.y,
            self.position.z,
         ));
         ui.text(im_str!(
            "Rotation: ({:.1},{:.1},{:.1})",
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
         ));
         ui.text(im_str!(
            "Size: ({:.1},{:.1},{:.1})",
            self.size.x,
            self.size.y,
            self.size.z,
         ));
         ui.separator();
         //.display_format(im_str!("%.0f"))
        
      });
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position,
                                         self.size,
                                         self.rotation,
                                         self.model.to_string()));
  }
}


