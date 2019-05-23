use maat_graphics::DrawCall;
use maat_graphics::camera;
use maat_graphics::imgui::*;

use hlua::Lua;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::WorldObject;
use crate::modules::import_export;
use crate::modules::import_export::{import, export};

use rand;
use rand::{thread_rng};

use cgmath::{Vector2, Vector3};

const CAMERA_DEFAULT_X: f32 = 83.93359;
const CAMERA_DEFAULT_Y: f32 = 128.62776;
const CAMERA_DEFAULT_Z: f32 = 55.85842;
const CAMERA_DEFAULT_PITCH: f32 = -62.27426;
const CAMERA_DEFAULT_YAW: f32 = 210.10083;
const CAMERA_DEFAULT_SPEED: f32 = 50.0;

enum MouseState {
  Ui,
  World,
}

pub struct EditorScreen {
  data: SceneData,
  rng: rand::prelude::ThreadRng,
  camera: camera::Camera,
  last_mouse_pos: Vector2<f32>,
  placing_height: f32,
  object_being_placed: Option<WorldObject>,
  world_objects: Vec<WorldObject>,
  mouse_state: MouseState,
  selected_model: i32,
  object_selected: i32,
  mouse_placement: bool,
  window_unloaded_models: bool,
  window_world_objects: bool,
  known_models: Vec<(String, String, bool)>,
  show_axis: bool,
  snap_to_grid: bool,
  run_game: bool,
  f6_released_last_frame: bool,
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
      world_objects: Vec::new(),
      mouse_state: MouseState::World,
      selected_model: 0,
      object_selected: 0,
      mouse_placement: false,
      window_unloaded_models: true,
      window_world_objects: true,
      known_models: import_export::get_models(),
      show_axis: true,
      snap_to_grid: false,
      run_game: false,
      f6_released_last_frame: true,
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, camera: camera::Camera, object_being_placed: Option<WorldObject>, placing_height: f32, world_objects: Vec<WorldObject>, mouse_placement: bool, window_unloaded_models: bool, window_world_objects: bool, snap_to_grid: bool, run_game: bool, model_sizes: Vec<(String, Vector3<f32>)>) -> EditorScreen {
    EditorScreen {
      data: SceneData::new(window_size, model_sizes),
      rng,
      camera,
      last_mouse_pos: Vector2::new(-1.0, -1.0),
      placing_height,
      object_being_placed,
      world_objects,
      mouse_state: MouseState::World,
      selected_model: 0,
      object_selected: 0,
      mouse_placement,
      window_unloaded_models,
      window_world_objects,
      known_models: import_export::get_models(),
      show_axis: true,
      snap_to_grid,
      run_game,
      f6_released_last_frame: true,
    }
  }
  
  pub fn update_input(&mut self, lua: &Option<&mut Lua>, delta_time: f32) {
    self.data.controller.update();
    
    let mouse = self.data.mouse_pos;
    
    let w_pressed = self.data.keys.w_pressed();
    let a_pressed = self.data.keys.a_pressed();
    let s_pressed = self.data.keys.s_pressed();
    let d_pressed = self.data.keys.d_pressed();
    let r_pressed = self.data.keys.r_pressed();
    let f_pressed = self.data.keys.f_pressed();
    
    let u_pressed = self.data.keys.u_pressed();
    let j_pressed = self.data.keys.j_pressed();
    let i_pressed = self.data.keys.i_pressed();
    let k_pressed = self.data.keys.k_pressed();
    let o_pressed = self.data.keys.o_pressed();
    let l_pressed = self.data.keys.l_pressed();
    
    let f6_pressed = self.data.keys.f6_pressed();
    
    let one_pressed = self.data.keys.one_pressed();
    let scroll_delta = self.data.scroll_delta;
    
    let left_clicked = self.data.left_mouse;
    let right_clicked = self.data.right_mouse;
    
    if right_clicked {
      self.object_being_placed = None;
      self.object_selected = 0;
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        let x_offset = self.last_mouse_pos.x - mouse.x;
        let y_offset = mouse.y - self.last_mouse_pos.y;
      //  self.camera.process_mouse_movement(x_offset, y_offset);
        
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
        
        let point_of_rotation = cam_pos;
        
        self.camera.process_mouse_movement_around_point(x_offset, y_offset, point_of_rotation);
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
    
    if self.object_selected > 0 {
      let mut pos = {
        if self.object_selected == 1 {
          let mut pos = Vector3::new(0.0, 0.0, 0.0);
          if let Some(object) = &self.object_being_placed {
            pos = object.position();
          }
          pos
        } else {
          self.world_objects[self.object_selected as usize-2].position()
        }
      };
      
      if u_pressed {
        pos.x += 5.0*delta_time;
      }
      if j_pressed {
        pos.x -= 5.0*delta_time;
      }
       if o_pressed {
        pos.z += 5.0*delta_time;
      }
      if l_pressed {
        pos.z -= 5.0*delta_time;
      }
      
      if self.mouse_placement {
        if i_pressed {
          self.placing_height += 5.0*delta_time;
        }
        if k_pressed {
          self.placing_height -= 5.0*delta_time;
        }
      } else {
        if i_pressed {
          pos.y += 5.0*delta_time;
        }
        if k_pressed {
          pos.y -= 5.0*delta_time;
        }
      }
      
      if self.object_selected == 1 {
        if let Some(object) = &mut self.object_being_placed {
          object.set_position(pos);
        }
      } else {
        self.world_objects[self.object_selected as usize-2].set_position(pos);
      }
    }
    
    if left_clicked {
      if let Some(object) = &self.object_being_placed {
        self.world_objects.push(object.clone());
      }
      self.object_being_placed = None;
      self.object_selected = 0;
    }
    
    if one_pressed {
      self.change_selected_object(&lua)
    }
    
    self.last_mouse_pos = mouse;
  }
  
  pub fn change_selected_object(&mut self, mut lua: &Option<&mut Lua>) {
    let id = {
      if self.world_objects.len() > 0 {
        self.world_objects[self.world_objects.len()-1].id()+1
      } else {
        0
      }
    };
    
    if self.data().model_sizes.len() > self.selected_model as usize {
      let (model_name, _) = self.data().model_sizes[self.selected_model as usize].clone();
      for i in 0..self.known_models.len() {
        if model_name.to_string() == self.known_models[i].0 {
          let location = self.known_models[i].1.clone();
          self.object_being_placed = Some(WorldObject::new_empty(id, model_name.to_string(), location));
          self.object_selected = 1;
        }
      }
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
      Box::new(EditorScreen::new_with_data(window_size, self.rng.clone(), self.camera.clone(), 
                                           self.object_being_placed.clone(), self.placing_height, 
                                           self.world_objects.clone(), self.mouse_placement, 
                                           self.window_unloaded_models, self. window_world_objects, 
                                           self.snap_to_grid, self.run_game, self.data.model_sizes.clone()))
    } else {
      Box::new(EditorScreen::new(window_size, self.data.model_sizes.clone()))
    }
  }
  
  fn update(&mut self, ui: Option<&Ui>, mut lua: Option<&mut Lua>, delta_time: f32) {
    if self.data.window_resized {
      self.data.next_scene = true;
    }
    
    {
      let f6_pressed = self.data().keys.f6_pressed();
      if f6_pressed && self.f6_released_last_frame {
        self.run_game = !self.run_game;
      }
    }
    
    if let Some(ui) = &ui {
      self.mut_data().imgui_info.wants_mouse = ui.want_capture_mouse();
      self.mut_data().imgui_info.wants_keyboard = ui.want_capture_keyboard();
      
      let mut should_new = false;
      let mut should_save = false;
      let mut should_load = false;
      let mut should_exit = false;
      
      ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
          ui.menu_item(im_str!("New")).selected(&mut should_new).build();
          ui.menu_item(im_str!("Save")).selected(&mut should_save).build();
          ui.menu_item(im_str!("Load")).selected(&mut should_load).build();
          ui.menu_item(im_str!("Exit")).selected(&mut should_exit).build();
        });
        ui.menu(im_str!("Edit Options")).build(|| {
          ui.menu_item(im_str!("Mouse Placement")).shortcut(im_str!("Ctrl+M")).selected(&mut self.mouse_placement).build();
          ui.menu_item(im_str!("Show Axis")).shortcut(im_str!("Ctrl+A")).selected(&mut self.show_axis).build();
          ui.menu_item(im_str!("Snap to grid")).shortcut(im_str!("Ctrl+G")).selected(&mut self.snap_to_grid).build();
        });
        ui.menu(im_str!("Windows")).build(|| {
          ui.menu_item(im_str!("Model List")).selected(&mut self.window_unloaded_models).build();
          ui.menu_item(im_str!("World Objects")).selected(&mut self.window_world_objects).build();
        });
      });
      
      if should_new {
        self.data.next_scene = true;
      }
      if should_save {
        export(&self.world_objects);
      }
      if should_load {
        let (load_models, objects) = import();
        self.world_objects = objects;
        self.data.models_to_load = load_models;
      }
      if should_exit {
        self.data.should_close = true;
      }
    }
    
    match self.run_game {
      true => {
        self.show_axis = false;
        if let Some(lua) = &mut lua {
          lua.set("delta_time", delta_time);
          lua.set("mouse_x", self.data.mouse_pos.x);
          lua.set("mouse_y", self.data.mouse_pos.y);
          lua.set("left_mouse", self.data.left_mouse);
          lua.set("right_mouse", self.data.right_mouse);
          lua.set("window_dim_x", self.data.window_dim.x);
          lua.set("window_dim_y", self.data.window_dim.y);
        }
        for world_object in &mut self.world_objects {
          world_object.update_game(&mut lua);
        }
      },
      false => {
        for i in 0..self.data.model_sizes.len() {
          for j in 0..self.known_models.len() {
            if self.data.model_sizes[i].0 == self.known_models[j].0 {
              self.known_models[j].2 = true;
            }
          }
        }
        
        if let Some(ui) = &ui {
          if self.window_world_objects {
            ui.window(im_str!("World Objects"))
              .size((200.0, 400.0), ImGuiCond::FirstUseEver)
              .build(|| {
                ui.text("None");
                ui.same_line(0.0);
                ui.radio_button(im_str!("##{}", 0), &mut self.object_selected, 0);
                ui.text("Placing New");
                ui.same_line(0.0);
                ui.radio_button(im_str!("Key 1##{}", 1), &mut self.object_selected, 1);
                let mut should_delete_object = false;
                for i in 0..self.world_objects.len() {
                  ui.text(im_str!("{}: {}", self.world_objects[i].id(), self.world_objects[i].name()));
                  ui.same_line(0.0);
                  ui.radio_button(im_str!("##{}", i+2), &mut self.object_selected, i as i32+2);
                  if self.object_selected == i as i32 +2 {
                    ui.same_line(0.0);
                    should_delete_object = ui.button(im_str!("Delete"), (0.0,0.0));
                  }
                }
                
                if should_delete_object {
                  self.world_objects.remove(self.object_selected as usize-2);
                  self.object_selected = 0;
                }
              });
          }
          
          if self.object_selected == 1 {
            if self.data.model_sizes.len() == 0 {
              self.object_selected = 0;
            } else if self.object_being_placed.is_none() {
              self.change_selected_object(&lua);
            }
          } else {
            self.object_being_placed = None;
          }
          
          if self.window_unloaded_models {
            let mut should_load_all = false;
            
            let window_width = 200.0;
            ui.window(im_str!("Model List ./Models/*"))
              .position((self.data.window_dim.x-window_width*1.1, 32.0), ImGuiCond::FirstUseEver)
              .size((window_width, 400.0), ImGuiCond::FirstUseEver)
              .build(|| {
                if ui.button(im_str!("Load All"), (0.0, 0.0)) {
                  should_load_all = true;
                }
                for i in 0..self.known_models.len() {
                  let mut model_loaded = self.known_models[i].2;
                  ui.text(im_str!("{}", self.known_models[i].0));
                  ui.same_line(0.0);
                  ui.checkbox(im_str!("##{}", i), &mut model_loaded);
                  if !self.known_models[i].2 && model_loaded {
                    let reference = self.known_models[i].0.to_string();
                    let location = self.known_models[i].1.to_string();
                    self.mut_data().models_to_load.push((reference, location));
                  }
                  if self.known_models[i].2 {
                    ui.same_line(0.0);
                    if ui.button(im_str!("Unload"), (0.0, 0.0)) { 
                      self.data.models_to_unload.push(self.known_models[i].0.to_string());
                      self.known_models[i].2 = false;
                    }
                  }
                }
              });
            
            if should_load_all {
              for i in 0..self.known_models.len() {
                let reference = self.known_models[i].0.to_string();
                let location = self.known_models[i].1.to_string();
                self.mut_data().models_to_load.push((reference, location));
              }
            }
          }
          
          ui.window(im_str!("Loaded Models"))
            .position((60.0, 460.0), ImGuiCond::FirstUseEver)
            .size((200.0, 400.0), ImGuiCond::FirstUseEver)
            //.always_auto_resize(true)
          .build(|| {
            let old_selection = self.selected_model;
            for i in 0..self.data().model_sizes.len() {
              let (reference, _) = self.data().model_sizes[i].clone();
              let name = (reference.to_string()).to_owned();
              ui.radio_button(im_str!("{}##{}",name,i), &mut self.selected_model, i as i32);
            }
            if old_selection != self.selected_model {
              if self.object_being_placed.is_some() {
                self.change_selected_object(&lua);
              }
            }
          });
        }
        
        if self.data().imgui_info.wants_mouse {
          self.mouse_state = MouseState::Ui;
        } else {
          self.mouse_state = MouseState::World;
        }
        
        let mouse = self.data.mouse_pos;
        
        match self.mouse_state {
          MouseState::Ui => {
            
          },
          MouseState::World => {
            self.update_input(&lua, delta_time);
            
            if self.mouse_placement {
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
                if self.snap_to_grid {
                  cam_pos.x = cam_pos.x.round();
                  cam_pos.y = cam_pos.y.round();
                  cam_pos.z = cam_pos.z.round();
                }
                object.set_position(cam_pos.xyz());
              }
            }
          }
        }
            
        if let Some(object) = &mut self.object_being_placed {
          object.update(ui, &mut lua, self.data.window_dim, delta_time);
        }
        
        if self.object_selected > 1 {
          self.world_objects[self.object_selected as usize-2].update(ui, &mut lua, self.data.window_dim, delta_time);
        }
      }
    }
    self.f6_released_last_frame = !self.data.keys.f6_pressed();
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
    
    if self.show_axis {
      let axis_position = Vector3::new(0.0, 0.0, 0.0);
      let axis_size = Vector3::new(50.0, 10.0, 10.0);
      let rot_x_size = Vector3::new(0.0, 0.0, 0.0);
      let rot_y_size = Vector3::new(0.0, 0.0, 90.0);
      let rot_z_size = Vector3::new(0.0, 90.0, 0.0);
      let axis = String::from("Axis");
      draw_calls.push(DrawCall::draw_model(axis_position,
                                           axis_size,
                                           rot_x_size,
                                           axis.to_string()));
      draw_calls.push(DrawCall::draw_model(axis_position,
                                           axis_size,
                                           rot_y_size,
                                           axis.to_string()));
      draw_calls.push(DrawCall::draw_model(axis_position,
                                           axis_size,
                                           rot_z_size,
                                           axis.to_string()));
    }
  }
}
