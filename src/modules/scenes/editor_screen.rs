use maat_graphics::DrawCall;
use maat_graphics::camera;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::WorldObject;

use rand;
use rand::{thread_rng};

use cgmath::{Vector2, Vector3};

const CAMERA_DEFAULT_X: f32 = 83.93359;
const CAMERA_DEFAULT_Y: f32 = 128.62776;
const CAMERA_DEFAULT_Z: f32 = 55.85842;
const CAMERA_DEFAULT_PITCH: f32 = -62.27426;
const CAMERA_DEFAULT_YAW: f32 = 210.10083;
const CAMERA_DEFAULT_SPEED: f32 = 50.0;

pub struct EditorScreen {
  data: SceneData,
  rng: rand::prelude::ThreadRng,
  camera: camera::Camera,
  last_mouse_pos: Vector2<f32>,
  placing_height: f32,
  object_being_placed: Option<WorldObject>,
  world_objects: Vec<WorldObject>,
}

impl EditorScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> EditorScreen {
    let rng =  thread_rng();
    
    let mut camera = camera::Camera::default_vk();
    camera.set_position(Vector3::new(CAMERA_DEFAULT_X, CAMERA_DEFAULT_Y, CAMERA_DEFAULT_Z));
    camera.set_pitch(CAMERA_DEFAULT_PITCH);
    camera.set_yaw(CAMERA_DEFAULT_YAW);
    camera.set_move_speed(CAMERA_DEFAULT_SPEED);
    
    EditorScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      placing_height: 0.0,
      object_being_placed: None,
      world_objects: vec!(WorldObject::new_empty(0, "Hexagon".to_string())),
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, camera: camera::Camera, object_being_placed: Option<WorldObject>, placing_height: f32, world_objects: Vec<WorldObject>, model_sizes: Vec<(String, Vector3<f32>)>) -> EditorScreen {
    EditorScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      placing_height,
      object_being_placed,
      world_objects,
    }
  }
  
  pub fn update_input(&mut self, delta_time: f32) {
    self.data.controller.update();
    
    let mouse = self.data.mouse_pos;
    
    let w_pressed = self.data.keys.w_pressed();
    let a_pressed = self.data.keys.a_pressed();
    let s_pressed = self.data.keys.s_pressed();
    let d_pressed = self.data.keys.d_pressed();
    let r_pressed = self.data.keys.r_pressed();
    let f_pressed = self.data.keys.f_pressed();
    let i_pressed = self.data.keys.i_pressed();
    let k_pressed = self.data.keys.k_pressed();
    let one_pressed = self.data.keys.one_pressed();
    let scroll_delta = self.data.scroll_delta;
    
    let left_clicked = self.data.left_mouse;
    let right_clicked = self.data.right_mouse;
    
    if right_clicked {
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        let x_offset = self.last_mouse_pos.x - mouse.x;
        let y_offset = mouse.y - self.last_mouse_pos.y;
        self.camera.process_mouse_movement(x_offset, y_offset);
      }
    }
    
    if w_pressed {
      self.camera.process_movement(camera::Direction::YAlignedForward, delta_time);
    }
    if a_pressed {
      self.camera.process_movement(camera::Direction::YAlignedLeft, delta_time);
    }
    if s_pressed {
      self.camera.process_movement(camera::Direction::YAlignedBackward, delta_time);
    }
    if d_pressed {
      self.camera.process_movement(camera::Direction::YAlignedRight, delta_time);
    }
    if r_pressed {
      self.camera.process_movement(camera::Direction::PositiveY, delta_time);
    }
    if f_pressed {
      self.camera.process_movement(camera::Direction::NegativeY, delta_time);
    }
    if scroll_delta > 0.0 {
      self.camera.process_movement(camera::Direction::Forward, 10.0*delta_time);
    } else if scroll_delta < 0.0 {
      self.camera.process_movement(camera::Direction::Backward, 10.0*delta_time);
    }
    if i_pressed {
      self.placing_height += 10.0*delta_time;
    }
    if k_pressed {
      self.placing_height -= 10.0*delta_time;
    }
    
    if left_clicked {
      if let Some(object) = &self.object_being_placed {
        self.world_objects.push(object.clone());
      }
      self.object_being_placed = None;
    }
    
    if one_pressed {
      let id = { 
        if self.world_objects.len() > 0 {
          self.world_objects[self.world_objects.len()-1].id()
        } else {
          0
        }
      };
      
      self.object_being_placed = Some(WorldObject::new_empty(id, "Hexagon".to_string()));
    }
    
    self.last_mouse_pos = mouse;
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
      Box::new(EditorScreen::new_with_data(window_size, self.rng.clone(), self.camera.clone(), 
                                           self.object_being_placed.clone(), self.placing_height, 
                                           self.world_objects.clone(), self.data.model_sizes.clone()))
    } else {
      Box::new(EditorScreen::new(window_size, self.data.model_sizes.clone()))
    }
  }
  
  fn update(&mut self, delta_time: f32) {
    if self.data.window_resized {
      self.data.next_scene = true;
    }
    
    let mouse = self.data.mouse_pos;
    
    self.update_input(delta_time);
    
    let mut cam_pos = self.camera.get_position();
    let mouse_ray = self.camera.mouse_to_world_ray(mouse, self.data.window_dim);
    if mouse_ray.y < 0.0 {
      while cam_pos.y > 0.0  {
        cam_pos += mouse_ray;
      }
      // TODO: align with goal height
      cam_pos -= mouse_ray;
      cam_pos.y = self.placing_height;
    }
    
    if let Some(object) = &mut self.object_being_placed {
      object.set_position(cam_pos.xyz());
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    // Window width and height is 1280 x 720
    //let width = self.data().window_dim.x;
    //let height = self.data().window_dim.y;
    
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    for world_object in &self.world_objects {
      world_object.draw(draw_calls);
    }
    
    if let Some(object) = &self.object_being_placed {
      object.draw(draw_calls);
    }
  }
}
