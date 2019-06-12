use maat_graphics::DrawCall;
use maat_graphics::camera;
use maat_graphics::imgui::*;

use hlua::Lua;

use crate::modules::scenes::Scene;
use crate::modules::scenes::SceneData;
use crate::modules::WorldObject;
use crate::modules::import_export;
use crate::modules::import_export::{import, export};
use crate::modules::Logs;

use rand;
use rand::{thread_rng};

use cgmath::{Vector2, Vector3};

use std::fs;

const CAMERA_DEFAULT_X: f32 = 83.93359;
const CAMERA_DEFAULT_Y: f32 = 128.62776;
const CAMERA_DEFAULT_Z: f32 = 55.85842;
const CAMERA_DEFAULT_PITCH: f32 = -62.27426;
const CAMERA_DEFAULT_YAW: f32 = 210.10083;
const CAMERA_DEFAULT_SPEED: f32 = 50.0;

#[derive(Clone)]
pub struct Light {
  pos: Vector3<f32>,
  colour: Vector3<f32>,
  intensity: f32,
}

impl Light {
  pub fn new() -> Light {
    Light {
      pos: Vector3::new(0.0, 0.0, 0.0),
      colour: Vector3::new(1.0, 1.0, 1.0),
      intensity: 100.0,
    }
  }
}

enum MouseState {
  Ui,
  World,
}

#[derive(Clone)]
pub struct EditorWindows {
  world_objects: bool,
  model_list: bool,
  loaded_models: bool,
  scene_details: bool,
  camera_options: bool,
  lights: bool,
  load_window: bool,
  saved: bool,
  error_window: bool,
}

#[derive(Clone)]
pub struct EditorOptions {
  snap_to_grid: bool,
  show_axis: bool,
  place_with_mouse: bool,
  instanced_option: i32,
}

#[derive(Clone)]
pub struct GameOptions {
  first_game_loop: bool,
  pub camera_type: i32,
  pub camera_target: i32,
  pub camera_distance: f32,
  pub camera_location: Vector3<f32>,
  pub camera_horizontal_rotation: bool,
  pub camera_vertical_rotation: bool,
}

impl EditorWindows {
  pub fn new() -> EditorWindows {
    EditorWindows {
      world_objects: true,
      model_list: true,
      loaded_models: true,
      scene_details: true,
      camera_options: true,
      lights: true,
      load_window: true,
      saved: false,
      error_window: false,
    }
  }
}

impl EditorOptions {
  pub fn new() -> EditorOptions {
    EditorOptions {
      snap_to_grid: false,
      show_axis: true,
      place_with_mouse: true,
      instanced_option: 0,
    }
  }
}

impl GameOptions {
  pub fn new() -> GameOptions {
    GameOptions {
      first_game_loop: true,
      camera_type: 0,
      camera_target: 0,
      camera_distance: 90.0,
      camera_location: Vector3::new(0.0,0.0,0.0),
      camera_horizontal_rotation: false,
      camera_vertical_rotation: false,
    }
  }
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
  known_models: Vec<(String, String, bool)>,
  run_game: bool,
  f6_released_last_frame: bool,
  right_clicked_last_frame: bool,
  update_mouse_cursor: bool,
  scene_name: String,
  load_scene_option: i32,
  logs: Logs,
  windows: EditorWindows,
  options: EditorOptions,
  game_options: GameOptions,
  light: Light,
  instanced_buffers: Vec<String>,
  instanced_buffers_added: Vec<String>,
}

impl EditorScreen {
  pub fn new(window_size: Vector2<f32>, model_sizes: Vec<(String, Vector3<f32>)>) -> EditorScreen {
    let rng =  thread_rng();
    
    let mut camera = camera::Camera::default_vk();
    camera.set_position(Vector3::new(CAMERA_DEFAULT_X, CAMERA_DEFAULT_Y, CAMERA_DEFAULT_Z));
    camera.set_pitch(CAMERA_DEFAULT_PITCH);
    camera.set_yaw(CAMERA_DEFAULT_YAW);
    camera.set_move_speed(CAMERA_DEFAULT_SPEED);
    
    let mut logs = Logs::new(window_size);
    
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
      known_models: import_export::get_models(&mut logs),
      run_game: false,
      f6_released_last_frame: true,
      right_clicked_last_frame: false,
      update_mouse_cursor: false,
      scene_name: "empty_scene".to_string(),
      load_scene_option: 0,
      logs,
      windows: EditorWindows::new(),
      options: EditorOptions::new(),
      game_options: GameOptions::new(),
      light: Light::new(),
      instanced_buffers: Vec::new(),
      instanced_buffers_added: Vec::new(),
    }
  }
  
  pub fn new_with_data(window_size: Vector2<f32>, rng: rand::prelude::ThreadRng, camera: camera::Camera, object_being_placed: Option<WorldObject>, scene_name: String, placing_height: f32, world_objects: Vec<WorldObject>, light: Light, windows: EditorWindows, options: EditorOptions, game_options: GameOptions, run_game: bool, model_sizes: Vec<(String, Vector3<f32>)>, instanced_buffers: Vec<String>) -> EditorScreen {
    
    let mut logs = Logs::new(window_size);
    
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
      known_models: import_export::get_models(&mut logs),
      run_game,
      f6_released_last_frame: true,
      right_clicked_last_frame: false,
      update_mouse_cursor: false,
      scene_name,
      load_scene_option: 0,
      logs,
      windows,
      options,
      game_options,
      light,
      instanced_buffers,
      instanced_buffers_added: Vec::new(),
    }
  }
  
  pub fn update_input(&mut self, delta_time: f32) {
    self.data.controller.update();
    
    let mouse = self.data.mouse_pos;
    
    let _w_pressed = self.data.keys.w_pressed();
    let _a_pressed = self.data.keys.a_pressed();
    let _s_pressed = self.data.keys.s_pressed();
    let _d_pressed = self.data.keys.d_pressed();
    let _r_pressed = self.data.keys.r_pressed();
    let _f_pressed = self.data.keys.f_pressed();
    
    let u_pressed = self.data.keys.u_pressed();
    let j_pressed = self.data.keys.j_pressed();
    let i_pressed = self.data.keys.i_pressed();
    let k_pressed = self.data.keys.k_pressed();
    let o_pressed = self.data.keys.o_pressed();
    let l_pressed = self.data.keys.l_pressed();
    
    let one_pressed = self.data.keys.one_pressed();
    let scroll_delta = self.data.scroll_delta;
    
    let left_clicked = self.data.left_mouse;
    let right_clicked = self.data.right_mouse;
    self.update_mouse_cursor = false;
    
    if right_clicked {
      self.object_being_placed = None;
      self.object_selected = 0;
      if self.last_mouse_pos != Vector2::new(-1.0, -1.0) {
        
        let x_offset = self.last_mouse_pos.x - mouse.x;
        let y_offset = mouse.y - self.last_mouse_pos.y;
        self.camera.process_orbiting_camera_movement(x_offset*-1.0, y_offset);
        
        if !self.right_clicked_last_frame {
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
          self.camera.set_target(point_of_rotation);
        }
        //self.camera.process_mouse_movement_around_point(x_offset, y_offset, point_of_rotation);
       // self.camera.rotate_camera_horizontally(Vector3::new(0.0, 0.0, 0.0), 1.0);
        
      }
      /*
      let mouse = self.data.mouse_pos;
      let mut new_mouse_pos = mouse;
      
      if mouse.x < 10.0 {
        new_mouse_pos.x = self.data.window_dim.x - 15.0;
      }
      if mouse.x > self.data.window_dim.x - 10.0 {
        new_mouse_pos.x = 15.0;
      }
      if mouse.y < 20.0 {
        new_mouse_pos.y = self.data.window_dim.y -25.0;
      }
      if mouse.y > self.data.window_dim.y - 20.0 {
        new_mouse_pos.y = 25.0;
      }
      
      if mouse != new_mouse_pos {
        self.update_mouse_cursor = true;
        self.last_mouse_pos = new_mouse_pos;
      }*/
    }
    
    self.camera.change_zoom(scroll_delta*-1.0, 100.0*delta_time);
    /*
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
    }*/
    
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
      
      if self.options.place_with_mouse {
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
      self.change_selected_object()
    }
    
    self.right_clicked_last_frame = right_clicked;
    self.last_mouse_pos = mouse;
  }
  
  pub fn reset(&mut self) {
    self.world_objects.clear();
    self.placing_height = 0.0;
    self.object_being_placed = None;
    self.mouse_state = MouseState::World;
    self.selected_model = 0;
    self.object_selected = 0;
    self.run_game = false;
    self.f6_released_last_frame = true;
    self.scene_name = "new_scene".to_string();
    self.load_scene_option = 0;
    self.windows.load_window = false;
    
    self.camera = camera::Camera::default_vk();
    self.camera.set_position(Vector3::new(CAMERA_DEFAULT_X, CAMERA_DEFAULT_Y, CAMERA_DEFAULT_Z));
    self.camera.set_pitch(CAMERA_DEFAULT_PITCH);
    self.camera.set_yaw(CAMERA_DEFAULT_YAW);
    self.camera.set_move_speed(CAMERA_DEFAULT_SPEED);
  }
  
  pub fn change_selected_object(&mut self) {
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
          self.object_being_placed = Some(WorldObject::new_empty(id, model_name.to_string(), location, self.scene_name.to_string()));
          self.object_selected = 1;
        }
      }
    }
  }
  
  pub fn draw_imgui(&mut self, ui: Option<&Ui>) {
    if let Some(ui) = &ui {
      self.mut_data().imgui_info.wants_mouse = ui.want_capture_mouse();
      self.mut_data().imgui_info.wants_keyboard = ui.want_capture_keyboard();
      
      if self.windows.load_window {
        if let Err(e) = fs::create_dir_all("./Scenes") {
          self.logs.add_error(e.to_string());
        }
        
        let paths = fs::read_dir("./Scenes/").unwrap();
        
        let mut scenes = Vec::new();
        
        for path in paths {
          scenes.push(ImString::new(path.unwrap().path().display().to_string()));
        }
        
        let mut should_load = false;
        let mut should_cancel = false;
        let mut new = false;
        ui.window(im_str!("Load Scene"))
          .size((500.0, 100.0), ImGuiCond::FirstUseEver)
          .always_auto_resize(true)
          .collapsible(false)
          .build( || {
            let items: Vec<_> = scenes.iter().map(|p| 
              p.as_ref()
            ).collect();
            
            ui.text("Scene: ");
            ui.same_line(0.0);
            ui.combo(im_str!(""), &mut self.load_scene_option, &items[..], -1);
            should_cancel = ui.button(im_str!("Cancel"), (0.0, 0.0));
            ui.same_line(0.0);
            should_load = ui.button(im_str!("Load"), (0.0,0.0));
            ui.same_line(400.0);
            new = ui.button(im_str!("New"), (0.0,0.0));
          });
        
        if new {
          self.reset();
          self.windows.load_window = false;
          return;
        }
        
        if should_cancel {
          self.windows.load_window = false;
          return;
        }
        
        if should_load {
          if self.load_scene_option as usize+1 > scenes.len() {
            self.windows.load_window = false;
            return;
          }
          
          let mut path = scenes[self.load_scene_option as usize].to_str().to_string();
          path.remove(0);
          path.remove(0);
          path.remove(0);
          path.remove(0);
          path.remove(0);
          path.remove(0);
          path.remove(0);
          path.remove(0);
          path.remove(0);
          let (load_models, objects, game_options) = import(path.to_string(), &mut self.logs);
          self.world_objects = objects;
          self.data.models_to_load = load_models;
          self.game_options = game_options;
          self.windows.load_window = false;
          self.scene_name = path.to_string();
        }
        
        return;
      }
      
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
          ui.menu_item(im_str!("Mouse Placement")).shortcut(im_str!("Ctrl+M")).selected(&mut self.options.place_with_mouse).build();
          ui.menu_item(im_str!("Show Axis")).shortcut(im_str!("Ctrl+A")).selected(&mut self.options.show_axis).build();
          ui.menu_item(im_str!("Snap to grid")).shortcut(im_str!("Ctrl+G")).selected(&mut self.options.snap_to_grid).build();
        });
        ui.menu(im_str!("Run Options")).build(|| {
          ui.menu_item(im_str!("Run")).shortcut(im_str!("F6")).selected(&mut self.run_game).build();
        });
        ui.menu(im_str!("Windows")).build(|| {
          ui.menu_item(im_str!("Scene Details")).selected(&mut self.windows.scene_details).build();
          ui.menu_item(im_str!("Model List")).selected(&mut self.windows.model_list).build();
          ui.menu_item(im_str!("Loaded Models")).selected(&mut self.windows.loaded_models).build();
          ui.menu_item(im_str!("World Objects")).selected(&mut self.windows.world_objects).build();
          ui.menu_item(im_str!("Camera Options")).selected(&mut self.windows.camera_options).build();
          ui.menu_item(im_str!("Light Options")).selected(&mut self.windows.lights).build();
        });
      });
      
      if should_new {
        self.reset();
      }
      
      if should_save {
        for object in &mut self.world_objects {
          object.save_script(self.scene_name.to_string(), &mut self.logs);
        }
        export(self.scene_name.to_string(), &self.world_objects, &self.game_options, &mut self.logs);
        self.windows.saved = true;
      }
      if should_load {
        self.windows.load_window = true;
      }
      if should_exit {
        self.data.should_close = true;
      }
      
      if self.run_game {
        return;
      }
      
      if self.windows.scene_details {
        let mut imstr_scene_name = ImString::with_capacity(32);
        imstr_scene_name.push_str(&self.scene_name);
        
        ui.window(im_str!("Scene Details"))
          .size((250.0, 60.0), ImGuiCond::FirstUseEver)
          .position((0.0, 55.0), ImGuiCond::FirstUseEver)
          .always_auto_resize(true)
          .build( || {
            ui.text("Scene name:");
            ui.same_line(0.0);
            ui.push_item_width(150.0);
            ui.input_text(im_str!(""), &mut imstr_scene_name).build();
            ui.pop_item_width();
             if ui.button(im_str!("Delete Scene"), (0.0, 0.0)) {
               self.world_objects.clear();
               self.placing_height = 0.0;
               self.object_being_placed = None;
               self.mouse_state = MouseState::World;
               self.selected_model = 0;
               self.object_selected = 0;
               self.run_game = false;
               self.f6_released_last_frame = true;
               let mut new_scene = ImString::with_capacity(32);
               new_scene.push_str("empty_scene");
               imstr_scene_name = new_scene;
               self.load_scene_option = 0;
               
               if let Err(e) = fs::remove_dir_all("./Scenes/".to_owned() + &self.scene_name) {
                 self.logs.add_error(e.to_string());
               }
             }
          });
          
        self.scene_name = imstr_scene_name.to_str().to_string();
      }
      
      if self.windows.world_objects {
        ui.window(im_str!("World Objects"))
          .size((200.0, 400.0), ImGuiCond::FirstUseEver)
          .position((0.0, 140.0), ImGuiCond::FirstUseEver)
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
              self.world_objects[self.object_selected as usize-2].delete_script(&mut self.logs);
              self.world_objects.remove(self.object_selected as usize-2);
              self.object_selected = 0;
            }
          });
      }
      
      if self.windows.model_list {
        let mut should_load_all = false;
        
        let window_width = 200.0;
        ui.window(im_str!("Model List ./Models/*"))
          .position((self.data.window_dim.x-window_width*1.1, 32.0), ImGuiCond::FirstUseEver)
          .size((window_width, 400.0), ImGuiCond::FirstUseEver)
          .build(|| {
            if ui.button(im_str!("Load All"), (0.0, 0.0)) {
              should_load_all = true;
            }
            let (size_x, size_y) = ui.get_content_region_avail();//get_window_size();
            ui.child_frame(im_str!("child frame"), (size_x, size_y))
              .show_borders(true)
              .build(|| {
                for i in 0..self.known_models.len() {
                  let mut model_loaded = self.known_models[i].2;
                  ui.text(im_str!("{}", self.known_models[i].0));
                  
                  //ui.checkbox(im_str!("##{}", i), &mut model_loaded);
                  if !self.known_models[i].2 {
                    ui.same_line(0.0);
                    if ui.button(im_str!("Load##{}", i), (0.0, 0.0)) {
                      let reference = self.known_models[i].0.to_string();
                      let location = self.known_models[i].1.to_string();
                      self.mut_data().models_to_load.push((reference, location));
                    }
                  }
                  /*
                  if self.known_models[i].2 {
                    ui.same_line(0.0);
                    if ui.button(im_str!("Unload"), (0.0, 0.0)) { 
                      self.data.models_to_unload.push(self.known_models[i].0.to_string());
                      self.known_models[i].2 = false;
                    }
                  }*/
                }
              });
          });
        
        if should_load_all {
          for i in 0..self.known_models.len() {
            let reference = self.known_models[i].0.to_string();
            let location = self.known_models[i].1.to_string();
            self.mut_data().models_to_load.push((reference, location));
          }
        }
      }
      
      if self.windows.loaded_models {
        ui.window(im_str!("Loaded Models"))
          .position((0.0, 540.0), ImGuiCond::FirstUseEver)
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
              self.change_selected_object();
            }
          }
        });
      }
      
      if self.windows.camera_options {
        ui.window(im_str!("Game Camera"))
          .always_auto_resize(true)
          .position((self.data.window_dim.x - 500.0, 25.0), ImGuiCond::FirstUseEver)
          .build(|| {
             ui.text("Camera Type:");
             ui.same_line(0.0);
             ui.push_item_width(150.0);
             ui.combo(im_str!(""), &mut self.game_options.camera_type, &[im_str!("First Person"), im_str!("Orbiting")], -1);
             ui.pop_item_width();
             
             match self.game_options.camera_type {
               0 => {
                 ui.new_line();
                 ui.text(im_str!("Position"));
                 
                 ui.columns(3, im_str!("x | y | z"), true);
                 ui.text(im_str!("x:"));
                 ui.same_line(0.0);
                 ui.input_float(im_str!("##x"), &mut self.game_options.camera_location.x).build();
                 ui.next_column();
                 ui.text(im_str!("y:"));
                 ui.same_line(0.0);
                 ui.input_float(im_str!("##y"), &mut self.game_options.camera_location.y).build();
                 ui.next_column();
                 ui.text(im_str!("z:"));
                 ui.same_line(0.0);
                 ui.input_float(im_str!("##z"), &mut self.game_options.camera_location.z).build();
               },
               1 => {
                 ui.text("Target:");
                 ui.same_line(0.0);
                 
                 let mut objects = Vec::new();
                 
                 for i in 0..self.world_objects.len() {
                   objects.push(ImString::new(self.world_objects[i].name()));
                 }
                 
                 let items: Vec<_> = objects.iter().map(|p| 
                   p.as_ref()
                 ).collect();
                 ui.push_item_width(190.0);
                 ui.combo(im_str!("##"), &mut self.game_options.camera_target, &items[..], -1);
                 ui.pop_item_width();
                 ui.text("Distance:");
                 ui.same_line(0.0);
                 ui.push_item_width(100.0);
                 ui.input_float(im_str!("##dist"), &mut self.game_options.camera_distance).build();
                 ui.pop_item_width();
                 ui.text("Horizontal Rotation");
                 ui.same_line(0.0);
                 ui.checkbox(im_str!("##horz"), &mut self.game_options.camera_horizontal_rotation);
                 ui.text("Vertical Rotation");
                 ui.same_line(0.0);
                 ui.checkbox(im_str!("##vert"), &mut self.game_options.camera_vertical_rotation);
               },
               _ => {},
             }
          });
      }
      
      if !self.windows.lights {
        ui.window(im_str!("Light Options"))
            .always_auto_resize(true)
            .size((200.0, 200.0), ImGuiCond::FirstUseEver)
            .position((self.data.window_dim.x - 500.0, 200.0), ImGuiCond::FirstUseEver)
            .build(|| {
              ui.new_line();
              ui.text(im_str!("Position"));
              
              ui.columns(3, im_str!("x | y | z"), true);
              ui.text(im_str!("x:"));
              ui.same_line(0.0);
              ui.input_float(im_str!("##x"), &mut self.light.pos.x).build();
              ui.next_column();
              ui.text(im_str!("y:"));
              ui.same_line(0.0);
              ui.input_float(im_str!("##y"), &mut self.light.pos.y).build();
              ui.next_column();
              ui.text(im_str!("z:"));
              ui.same_line(0.0);
              ui.input_float(im_str!("##z"), &mut self.light.pos.z).build();
              ui.columns(1, im_str!(""), false);
              ui.new_line();
              ui.text(im_str!("Intensity:"));
              ui.same_line(0.0);
              ui.slider_float(im_str!(""), &mut self.light.intensity, 0.1, 1000.0).build();
              ui.new_line();
              ui.tree_node(im_str!("Light Colour")).build(|| {
                let mut colour = [self.light.colour.x, self.light.colour.y, self.light.colour.z];
                ui.color_picker(im_str!("Colour"), &mut colour).build();
                self.light.colour = Vector3::new(colour[0], colour[1], colour[2]);
              });
        });
      }
      
       ui.window(im_str!("Instanced Options"))
            .always_auto_resize(true)
            .size((200.0, 200.0), ImGuiCond::FirstUseEver)
            .position((self.data.window_dim.x - 500.0, 200.0), ImGuiCond::FirstUseEver)
            .build(|| {
                            
              ui.text("Existing Buffers");
              let mut offset = 0;
              for i in 0..self.instanced_buffers.len() {
                if i < offset {
                  break;
                }
                
                ui.text(&self.instanced_buffers[i-offset].to_string());
                ui.same_line(0.0);
                if ui.button(im_str!("Remove##{}",i), (0.0, 0.0)) {
                  let buffer = self.instanced_buffers.remove(i-offset);
                  offset += 1;
                  for objects in &mut self.world_objects {
                    objects.instanced_buffer_removed(buffer.to_string());
                  }
                }
              }
              
              ui.push_item_width(190.0);
              if self.data.model_sizes.len() != self.instanced_buffers.len() {
                let mut items = self.data.model_sizes.clone().into_iter().map(|model| {
                    ImString::new(model.0)
                  }).collect::<Vec<ImString>>();
                
                let mut offset = 0;
                for i in 0..items.len() {
                  if i < offset {
                    break;
                  }
                  
                  if self.instanced_buffers.contains(&items[i-offset].to_str().to_string()) {
                    items.remove(i-offset);
                    offset+=1;
                  }
                }
                
                let items: Vec<_> = items.iter().map(|p| 
                    p.as_ref()
                ).collect();
                
                ui.combo(im_str!("##"), &mut self.options.instanced_option, 
                  &items[..], -1);
                ui.same_line(0.0);
                if self.options.instanced_option > items.len() as i32-1 {
                  self.options.instanced_option = items.len() as i32-1;
                }
                
                if ui.button(im_str!("Add Buffer"), (0.0, 0.0)) {
                  // Actually add buffer
                  
                  self.instanced_buffers_added.push(items[self.options.instanced_option as usize].to_str().to_string());
                }
              }
      });
      
      if self.windows.saved {
        ui.window(im_str!("Scene Saved!"))
          .position((self.data.window_dim.x*0.5, self.data.window_dim.y*0.5), ImGuiCond::FirstUseEver)
          .size((200.0, 100.0), ImGuiCond::FirstUseEver)
          .build(|| {
            if ui.button(im_str!("Ok"), (0.0, 0.0)) {
              self.windows.saved = false;
            }
          });
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
                                           self.object_being_placed.clone(), self.scene_name.to_string(), 
                                           self.placing_height, self.world_objects.clone(), self.light.clone(), 
                                           self.windows.clone(), self.options.clone(), self.game_options.clone(),
                                           self.run_game, self.data.model_sizes.clone(), self.instanced_buffers.clone()))
    } else {
      Box::new(EditorScreen::new(window_size, self.data.model_sizes.clone()))
    }
  }
  
  fn update(&mut self, ui: Option<&Ui>, mut lua: Option<&mut Lua>, delta_time: f32) {
    if self.data.window_resized {
      self.data.next_scene = true;
    }
    
    for buffer in &self.instanced_buffers_added {
      self.instanced_buffers.push(buffer.to_string());
    }
    
    self.instanced_buffers_added.clear();
    
    {
      let f6_pressed = self.data().keys.f6_pressed();
      
      let should_run = self.run_game;
      self.draw_imgui(ui);
      if f6_pressed && self.f6_released_last_frame {
        self.run_game = !self.run_game;
      }
      
      // Load scripts if went from edit to game run
      if self.run_game && !should_run {
        for object in &mut self.world_objects {
          object.load_script();
        }
      // Reset positions if went from game run to edit
      } else if !self.run_game && should_run {
        for object in &mut self.world_objects {
          object.reset();
        }
      }
    }
    
    if self.object_selected == 1 {
      if self.data.model_sizes.len() == 0 {
        self.object_selected = 0;
      } else if self.object_being_placed.is_none() {
        self.change_selected_object();
      }
    } else {
      self.object_being_placed = None;
    }
    
    match self.run_game {
      true => {
        if self.game_options.first_game_loop {
          for object in &mut self.world_objects {
            object.load_script();
          }
          self.camera.set_zoom(self.game_options.camera_distance);
          self.options.show_axis = false;
          self.game_options.first_game_loop = false;
        }
        
        if let Some(lua) = &mut lua {
          lua.set("delta_time", delta_time);
          lua.set("mouse_x", self.data.mouse_pos.x);
          lua.set("mouse_y", self.data.mouse_pos.y);
          lua.set("left_mouse", self.data.left_mouse);
          lua.set("right_mouse", self.data.right_mouse);
          lua.set("window_dim_x", self.data.window_dim.x);
          lua.set("window_dim_y", self.data.window_dim.y);
          
          lua.set("w_key", self.data.keys.w_pressed());
          lua.set("a_key", self.data.keys.a_pressed());
          lua.set("s_key", self.data.keys.s_pressed());
          lua.set("d_key", self.data.keys.d_pressed());
        }
        
        let mut i = 0;
        for world_object in &mut self.world_objects {
          world_object.update_game(&mut lua, &mut self.logs);
          
          if i == self.game_options.camera_target {
            self.camera.set_target(world_object.position());
          }
          
          i += 1;
        }
        
      },
      false => {
        if !self.game_options.first_game_loop {
          self.options.show_axis = true;
          self.game_options.first_game_loop = true;
        }
        
        for i in 0..self.data.model_sizes.len() {
          for j in 0..self.known_models.len() {
            if self.data.model_sizes[i].0 == self.known_models[j].0 {
              self.known_models[j].2 = true;
            }
          }
        }
        
        let mouse = self.data.mouse_pos;
        
        match self.mouse_state {
          MouseState::Ui => {
            
          },
          MouseState::World => {
            self.update_input(delta_time);
            
            if self.options.place_with_mouse {
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
                if self.options.snap_to_grid {
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
          object.update(ui, &self.instanced_buffers, self.data.window_dim, delta_time, &mut self.logs);
        }
        
        if self.object_selected > 1 {
          self.world_objects[self.object_selected as usize-2].update(ui, &self.instanced_buffers, self.data.window_dim, delta_time, &mut self.logs);
        }
      }
    }
    
    if self.data().imgui_info.wants_mouse {
      self.mouse_state = MouseState::Ui;
    } else {
      self.mouse_state = MouseState::World;
    
    }
    self.f6_released_last_frame = !self.data.keys.f6_pressed();
    
    
    if self.logs.is_shown() {
      self.logs.draw(ui);
    }
  }
  
  fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
  /*  if self.update_mouse_cursor {
      draw_calls.push(DrawCall::set_cursor_position(self.last_mouse_pos.x, self.last_mouse_pos.y));
    }*/
    for buffer in &self.instanced_buffers_added {
      draw_calls.push(DrawCall::add_instanced_model_buffer(buffer.to_string()));
    }
    
    draw_calls.push(DrawCall::set_light(self.light.pos, self.light.colour, self.light.intensity));
    draw_calls.push(DrawCall::set_camera(self.camera.clone()));
    
    let mut i = 0;
    for world_object in &self.world_objects {
      if i == self.object_selected as i32-2 {
        world_object.draw_hologram(draw_calls);
      } else {
        world_object.draw(draw_calls);
      }
      
      i+=1;
    }
    
    if let Some(object) = &self.object_being_placed {
      object.draw_hologram(draw_calls);
    }
    
    if self.options.show_axis {
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
    
    for buffer in &self.instanced_buffers {
      draw_calls.push(DrawCall::draw_instanced_model(buffer.to_string()));
    }
  }
}
