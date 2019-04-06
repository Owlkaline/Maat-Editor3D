use maat_graphics::DrawCall;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;

use rand;
use rand::{thread_rng};

use cgmath::{Vector2, Vector3};

pub struct EditorScreen {
  data: SceneData,
  rng: rand::prelude::ThreadRng,
  // Put new variables you want to use here
}

impl EditorScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> EditorScreen {
    let rng =  thread_rng();
    
    EditorScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      // Make sure to initialize new variables here
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, model_sizes: Vec<(String, Vector3<f32>)>) -> EditorScreen {
    EditorScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      // And initialize  new variables here too
    }
  }
}

impl Scene for EditorScreen {
  fn data(&self) -> &SceneData {
    &self.data
  }
  
  fn mut_data(&mut self) -> &mut SceneData {
    &mut self.data
  }
  
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene> {
    if self.data().window_resized {
      Box::new(EditorScreen::new_with_data(window_size, self.rng.clone(), self.data.model_sizes.clone()))
    } else {
      Box::new(EditorScreen::new(window_size, self.data.model_sizes.clone()))
    }
  }
  
  fn update(&mut self, _delta_time: f32) {
    // Ignore
    self.mut_data().controller.update();
    
  }
  
  fn draw(&self, _draw_calls: &mut Vec<DrawCall>) {
    // Window width and height is 1280 x 720
    //let width = self.data().window_dim.x;
    //let height = self.data().window_dim.y;
    
  }
}
