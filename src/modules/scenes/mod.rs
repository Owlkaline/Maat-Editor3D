use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use maat_input_handler::MappedKeys;
use maat_input_handler::Controller;

use std::vec::Vec;

use winit;
use winit::MouseScrollDelta::LineDelta;
use winit::MouseScrollDelta::PixelDelta;

use cgmath::{Vector2, Vector3};

pub use self::load_screen::LoadScreen;
pub use self::editor_screen::EditorScreen;

mod load_screen;
mod editor_screen;

pub struct SceneData {
  pub should_close: bool,
  pub next_scene: bool,
  mouse_pos: Vector2<f32>,
  pub scroll_delta: f32,
  left_mouse: bool,
  right_mouse: bool,
  middle_mouse: bool,
  pub left_mouse_dragged: bool,
  pub right_mouse_dragged: bool,
  pub middle_mouse_dragged: bool,
  pub window_dim: Vector2<f32>,
  pub currently_pressed: Vec<u32>,
  pub released_this_render: Vec<u32>,
  pub keys: MappedKeys,
  pub window_resized: bool,
  pub controller: Controller,
  pub model_sizes: Vec<(String, Vector3<f32>)>,
}

impl SceneData {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> SceneData {
    SceneData {
      should_close: false,
      next_scene: false,
      mouse_pos: Vector2::new(0.0, 0.0),
      scroll_delta: 0.0, // Scroll Delta is either -1, 0 or 1
      left_mouse: false,
      right_mouse: false,
      middle_mouse: false,
      left_mouse_dragged: false,
      right_mouse_dragged: false,
      middle_mouse_dragged: false,
      window_dim: window_size,
      currently_pressed: Vec::new(),
      released_this_render: Vec::new(),
      keys: MappedKeys::new(),
      window_resized: false,
      controller: Controller::new(),
      model_sizes,
    }
  }
  
  pub fn new_default() -> SceneData {
    SceneData {
      should_close: false,
      next_scene: false,
      mouse_pos: Vector2::new(0.0, 0.0),
      scroll_delta: 0.0, // Scroll Delta is either -1, 0 or 1
      left_mouse: false,
      right_mouse: false,
      middle_mouse: false,
      left_mouse_dragged: false,
      right_mouse_dragged: false,
      middle_mouse_dragged: false,
      window_dim: Vector2::new(1.0, 1.0),
      currently_pressed: Vec::new(),
      released_this_render: Vec::new(),
      keys: MappedKeys::new(),
      window_resized: false,
      controller: Controller::new(),
      model_sizes: Vec::new(),
    }
  }
  
  pub fn update_mouse_pos(&mut self, mouse_position: Vector2<f32>) {
    self.mouse_pos = mouse_position;
  }
  
  pub fn update_window_dim(&mut self, dim: Vector2<f32>) {
    if self.window_dim != dim {
      self.window_resized = true;
      self.window_dim = dim;
    }
  }
}


pub trait Scene {
  fn data(&self) -> &SceneData;
  fn mut_data(&mut self) -> &mut SceneData;
  fn future_scene(&mut self, window_size: Vector2<f32>) -> Box<Scene>;
  
  fn update(&mut self, delta_time: f32);
  fn draw(&self, draw_calls: &mut Vec<DrawCall>, ui: Option<&Ui>);
  
  fn scene_finished(&self) -> bool {
    self.data().next_scene
  }
  
  fn reset_scroll_value(&mut self) {
    self.mut_data().scroll_delta = 0.0;
  }
  
  fn set_window_dimensions(&mut self, new_dim: Vector2<f32>) {
    self.mut_data().update_window_dim(new_dim);
  }
  
  fn set_mouse_position(&mut self, mouse_position: Vector2<f32>) {
    self.mut_data().update_mouse_pos(mouse_position);
  }
  
  fn add_model_size(&mut self, reference: String, size: Vector3<f32>) {
    self.mut_data().model_sizes.push((reference, size));
  }
  
  fn handle_input(&mut self, event: &winit::WindowEvent) -> bool {
    self.mut_data().released_this_render.clear();


    if self.data().left_mouse {
      self.mut_data().left_mouse_dragged = true;
    }
    
    if self.data().right_mouse {
      self.mut_data().right_mouse_dragged = true;
    }
    
    if self.data().middle_mouse {
      self.mut_data().middle_mouse_dragged = true;
    }
    
    match event {
      winit::WindowEvent::MouseWheel {device_id: _, delta, phase: _, modifiers: _} => {
        match delta {
          PixelDelta(scroll_delta) => {
            println!("Not used. Please contact Lilith@inet-sys.com: {}", scroll_delta.y);
          },
          LineDelta(_x, y) => {
            // Scroll Delta is either -1, 0 or 1
            self.mut_data().scroll_delta = *y;
          },
        }
      },
      winit::WindowEvent::ReceivedCharacter(character) => {
        if character.is_ascii() || character.is_ascii_control() || character.is_ascii_whitespace() {
          let mut string_char = character.to_string();
          
          if *character == '\n' || *character == '\r' {
            string_char = "Enter".to_string();
          } else if *character == '\x08' {
            string_char = "Backspace".to_string();
          } else if character.is_ascii_control() {
            string_char = "".to_string();
          }
          
          self.mut_data().keys.pressed_this_frame.push(string_char);
        }
      },
      winit::WindowEvent::KeyboardInput{device_id: _, input} => {
        let key = input.scancode;
        
        if input.state == winit::ElementState::Pressed {
          let mut already_pressed = false;
          for pressed_key in self.data().currently_pressed.iter() {
            if pressed_key == &key {
              already_pressed = true;
              break;
            }
          }
          
          if !already_pressed {
            self.mut_data().currently_pressed.push(key);
          }
        }
        
        if input.state == winit::ElementState::Released {
          self.mut_data().released_this_render.push(key);
          let index = self.mut_data().currently_pressed.iter().position(|x| *x == key);
          if index != None {
            self.mut_data().currently_pressed.remove(index.unwrap());
          }
        }
      },
      winit::WindowEvent::MouseInput {device_id: _, state, button, modifiers: _} =>{
        if *state == winit::ElementState::Pressed {
          if *button == winit::MouseButton::Left {
            self.mut_data().left_mouse = true;
            self.mut_data().left_mouse_dragged = true;
          }
          if *button == winit::MouseButton::Right {
            self.mut_data().right_mouse = true;
            self.mut_data().right_mouse_dragged = true;
          }
          if *button == winit::MouseButton::Middle {
            self.mut_data().middle_mouse = true;
          }
        }
        if *state == winit::ElementState::Released {
          if *button == winit::MouseButton::Left {
            self.mut_data().left_mouse = false;
            self.mut_data().left_mouse_dragged = false;
          }
          if *button == winit::MouseButton::Right {
            self.mut_data().right_mouse = false;
            self.mut_data().right_mouse_dragged = false;
          }
          if *button == winit::MouseButton::Middle {
            self.mut_data().middle_mouse = false;
            self.mut_data().middle_mouse_dragged = false;
          }
        }
      },
      _ => {},
    }
    let cp = self.data().currently_pressed.clone();
    let rr = self.data().released_this_render.clone();
    self.mut_data().keys.update_keys(cp, rr);
    
    self.data().should_close
  }
  
  fn get_keys_pressed_this_frame(&self) -> Vec<String> {
    self.data().keys.get_pressed_this_frame()
  }
}
