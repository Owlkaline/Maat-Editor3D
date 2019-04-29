use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::scenes::EditorScreen;

use hlua::Lua;

use cgmath::{Vector2, Vector4};

const LOGO_TIMER: f32 = 1.5;

pub struct LoadScreen {
  data: SceneData,
  alpha: f32,
  logo_timer: f32,
  first_loop: bool,
  loop_num: u32,
}

impl LoadScreen {
  pub fn new() -> LoadScreen {
    LoadScreen {
      data: SceneData::new_default(),
      alpha: 0.0,
      logo_timer: LOGO_TIMER,
      first_loop: true,
      loop_num: 0,
    }
  }
}

impl Scene for LoadScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    Box::new(EditorScreen::new(window_size, self.data.model_sizes.clone()))
  }
  
  fn update(&mut self, _ui: Option<&Ui>, _lua: Option<&mut Lua>, delta_time: f32) {
    self.logo_timer -= delta_time as f32;
    self.alpha = 1.0 - (self.logo_timer / (LOGO_TIMER*0.7));
    
    if self.logo_timer <= 0.0 {
      self.mut_data().next_scene = true;
    }
    
    if self.loop_num == 1 {
      self.first_loop = false;
    }
    self.loop_num += 1;
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    let dim = self.data().window_dim;
    let (width, height) = (dim.x as f32, dim.y as f32);
    
    if self.first_loop {
      draw_calls.push(DrawCall::load_model("Axis".to_string()));
    }
    
    draw_calls.push(DrawCall::set_texture_scale(1.0));
    
    draw_calls.push(
        DrawCall::draw_coloured(Vector2::new(width*0.5, height*0.5),
                                Vector2::new(width*5.0, height*5.0),
                                Vector4::new(1.0, 1.0, 1.0, 1.0),
                                90.0)
    );
    
    draw_calls.push(
      DrawCall::draw_textured(Vector2::new(width*0.45, height*0.6), 
                              Vector2::new(500.0, 500.0),
                              90.0,
                              String::from("Logo"))
    );
    
    draw_calls.push(
      DrawCall::draw_text_basic(Vector2::new(width*0.45+50.0, height*0.6-100.0), 
                                Vector2::new(512.0, 512.0),
                                Vector4::new(0.0, 0.0, 0.0, 1.0),
                                String::from("tah"),
                                String::from("Arial"))
    );
    
    draw_calls.push(
        DrawCall::draw_coloured(Vector2::new(width*0.5, height*0.5),
                                Vector2::new(width*5.0, height*5.0),
                                Vector4::new(0.1, 0.1, 0.1, self.alpha),
                                90.0)
    );
  }
}
