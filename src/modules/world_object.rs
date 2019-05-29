use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use crate::modules::Logs;

use std::io::{Write, BufWriter};
use std::fs::File;
use std::fs;
use std::fs::copy;
use std::path::Path;
use std::io::Read;
use std::io::BufReader;

use hlua;
use hlua::Lua;

use cgmath::{Vector2, Vector3};

const LOCATION: &str = "./Scenes/";
const OBJECTS: &str = "/Objects/";

pub struct WorldObject {
  reference_num: u32,
  model: String,
  name: String,
  location: String,
  directory: String,
  position: Vector3<f32>,
  rotation: Vector3<f32>,
  size: Vector3<f32>,
  position_edit: bool,
  size_edit: bool,
  rotation_edit: bool,
  has_script: bool,
  update_function: Option<File>,
}

impl Clone for WorldObject {
  fn clone(&self) -> Self {
    let mut obj = WorldObject::new_with_name(self.reference_num, self.name.to_string(), self.directory.to_string(), self.model.to_string(), self.location.to_string(), self.position, self.rotation, self.size);
    if let Some(function) = &self.update_function {
      obj.update_function = Some(function.try_clone().unwrap());
    }
    
    obj
  }
}

impl WorldObject {
  pub fn new_empty(reference_num: u32, model: String, location: String, scene_name: String) -> WorldObject {
    WorldObject {
      reference_num,
      model: model.to_string(),
      location,
      directory: scene_name.to_string(),
      name: model.to_owned() + &reference_num.to_string(),
      position: Vector3::new(0.0, 0.0, 0.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      size: Vector3::new(1.0, 1.0, 1.0),
      position_edit: false,
      size_edit: false,
      rotation_edit: false,
      has_script: false,
      update_function: None,
    }
  }
  
  pub fn new_with_name(reference_num: u32, object_name: String, directory: String, model: String, location: String, position: Vector3<f32>, rotation: Vector3<f32>, size: Vector3<f32>) -> WorldObject {
    let function = None;
    
    let file_name = object_name.to_owned() + ".lua";
    let mut has_script = false;
    if let Ok(_) = File::open(&Path::new(&(LOCATION.to_owned() + &directory.to_string() + &OBJECTS.to_string() + &file_name))) {
      has_script = true;
    }
    
    let object = WorldObject {
      reference_num,
      model,
      name: object_name,
      location,
      directory,
      position,
      rotation,
      size,
      position_edit: false,
      size_edit: false,
      rotation_edit: false,
      has_script,
      update_function: function,
    };
    
    object
  }
  
  pub fn _new(reference_num: u32, model: String, location: String, directory: String, position: Vector3<f32>, rotation: Vector3<f32>, size: Vector3<f32>) -> WorldObject {
    let object_name  = model.to_owned() + &reference_num.to_string();
    
    WorldObject::new_with_name(reference_num, object_name.to_string(), directory, model, location, position, rotation, size)
  }
  
  pub fn create_script(&mut self, logs: &mut Logs) {
    if self.has_script {
      return;
    }
    
    let file_name = self.name.to_owned() + ".lua";
    
    // Create lua file
    if let Err(e) = fs::create_dir_all(LOCATION.to_owned() + &self.directory.to_string() + &OBJECTS.to_string()) {
      logs.add_error(e.to_string());
    }
    
    match File::create(LOCATION.to_owned() + &self.directory.to_string() + &OBJECTS.to_string() + &file_name.to_string()) {
      Ok(f) => {
        let mut f = BufWriter::new(f);
      
        let data = "-- ref_num
-- delta_time
-- mouse_x
-- mouse_y
-- left_mouse
-- right_mouse
-- window_dim_x
-- window_dim_y

-- x
-- y
-- z
-- rot_x
-- rot_y
-- rot_z
-- size_x
-- size_y
-- size_z

function ".to_owned() + &self.name.to_string() + "update()
  x = x + 100.0*delta_time
end";
        
        if let Err(e) = f.write_all(data.as_bytes()) {
          logs.add_error(e.to_string());
        }
      },
      Err(e) => {
        logs.add_error(e.to_string());
      }
    }
  }
  
  pub fn save_script(&mut self, directory: String, logs: &mut Logs) {
    if !self.has_script {
      return;
    }
    
    let file_name = self.name.to_owned() + ".lua";
    
    // Create lua folder
    if let Err(e) = fs::create_dir_all(LOCATION.to_owned() + &directory.to_string() + &OBJECTS.to_string()) {
      logs.add_error(e.to_string());
    }
    
    let file_from = LOCATION.to_owned() + &self.directory.to_string() + &OBJECTS.to_string() + &file_name.to_string();
    let file_to = LOCATION.to_owned() + &directory.to_string() + &OBJECTS.to_string() + &file_name.to_string();
    
    if file_from.eq(&file_to) {
      return;
    }
    
    if let Err(e) = copy(file_from, file_to) {
      logs.add_error(e.to_string());
    }
  }
  
  pub fn load_script(&mut self) {
    self.update_function = None;
    
    let file_name = self.name.to_owned() + ".lua";
    if let Ok(f) = File::open(&Path::new(&(LOCATION.to_owned() + &self.directory.to_string() + &OBJECTS.to_string() + &file_name))) {
      
      self.update_function = Some(f);
    }
  }
  
  pub fn _get_id(&mut self) -> i64 {
    self.reference_num as i64
  }
  
  pub fn id(&self) -> u32 {
    self.reference_num
  }
  
  pub fn name(&self) -> String {
    self.name.to_string()
  }
  
  pub fn model(&self) -> String {
    self.model.to_string()
  }
  
  pub fn location(&self) -> String {
    self.location.to_string()
  }
  
  pub fn position(&self) -> Vector3<f32> {
    self.position
  }
  
  pub fn size(&self) -> Vector3<f32> {
    self.size
  }
  
  pub fn rotation(&self) -> Vector3<f32> {
    self.rotation
  }
  
  pub fn set_position(&mut self, pos: Vector3<f32>) {
    self.position = pos;
  }
  
  pub fn update_game(&mut self, lua: &mut Option<&mut Lua>) {
    
    if let Some(lua) = lua {
      lua.set("ref_num", self.reference_num);
      lua.set("x", self.position.x);
      lua.set("y", self.position.y);
      lua.set("z", self.position.z);
      lua.set("size_x", self.size.x);
      lua.set("size_y", self.size.y);
      lua.set("size_z", self.size.z);
      lua.set("rot_x", self.rotation.x);
      lua.set("rot_y", self.rotation.y);
      lua.set("rot_z", self.rotation.z);
      
      if let Some(function) = &self.update_function {
        lua.execute_from_reader::<(), _>(function);
        let function_name = self.name.to_owned() + "update";
        let mut update: hlua::LuaFunction<_> = lua.get(function_name).unwrap();
        update.call::<()>().unwrap();
      }
      
      self.position.x = lua.get("x").unwrap();
      self.position.y = lua.get("y").unwrap();
      self.position.z = lua.get("z").unwrap();
      self.size.x = lua.get("size_x").unwrap();
      self.size.y = lua.get("size_y").unwrap();
      self.size.z = lua.get("size_z").unwrap();
      self.rotation.x = lua.get("rot_x").unwrap();
      self.rotation.y = lua.get("rot_y").unwrap();
      self.rotation.z = lua.get("rot_z").unwrap();
    }
  }
  
  pub fn update(&mut self, ui: Option<&Ui>, window_dim: Vector2<f32>, _delta_time: f32, logs: &mut Logs) {
     if let Some(ui) = &ui {
       let ui_window_size = (450.0, 200.0);
       
       let mut imstr_name = ImString::with_capacity(32);
       imstr_name.push_str(&self.name);
       
       ui.window(im_str!("Object Being Placed"))
       .size(ui_window_size, ImGuiCond::FirstUseEver)
       .position((window_dim.x-ui_window_size.0-20.0, 432.0), ImGuiCond::FirstUseEver)
       //.always_auto_resize(true)
       .build(|| {
          if self.has_script {
            let txt = "Script: ".to_owned() + &self.name.to_string() + ".lua";
            let mut imstr_script = ImString::with_capacity(32);
            imstr_script.push_str(&txt);
            ui.text(imstr_script);
          } else {
            if ui.button(im_str!("Create Script"), (0.0, 0.0)) {
              self.create_script(logs);
              self.has_script = true;
            }
          }
          ui.text("Name:");
          ui.same_line(0.0);
          ui.input_text(im_str!(""), &mut imstr_name).build();
          ui.text(im_str!(
            "Position: ({:.1},{:.1},{:.1})",
            self.position.x,
            self.position.y,
            self.position.z,
         ));
         ui.same_line(0.0);
         ui.checkbox(im_str!("Edit"), &mut self.position_edit);
         ui.text(im_str!(
            "Rotation: ({:.1},{:.1},{:.1})",
            self.rotation.x,
            self.rotation.y,
            self.rotation.z,
         ));
         ui.same_line(0.0);
         ui.checkbox(im_str!("Edit##1"), &mut self.rotation_edit);
         ui.text(im_str!(
            "Size: ({:.1},{:.1},{:.1})",
            self.size.x,
            self.size.y,
            self.size.z,
         ));
         ui.same_line(0.0);
         ui.checkbox(im_str!("Edit##2"), &mut self.size_edit);
         ui.separator();
         ui.columns(4, im_str!("x | y | z"), true);
         if self.position_edit {
           ui.text(im_str!("Position"));
           ui.next_column();
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
         }
         if self.rotation_edit {
           loop {
             if ui.get_column_index() == 0 {
               break;
             }
             ui.next_column();
           }
           ui.text(im_str!("Rotation"));
           ui.next_column();
           ui.text(im_str!("X:"));
           ui.same_line(0.0);
           ui.input_float(im_str!("##rotx"), &mut self.rotation.x).build();
           ui.next_column();
           ui.text(im_str!("Y:"));
           ui.same_line(0.0);
           ui.input_float(im_str!("##roty"), &mut self.rotation.y).build();
           ui.next_column();
           ui.text(im_str!("Z:"));
           ui.same_line(0.0);
           ui.input_float(im_str!("##rotz"), &mut self.rotation.z).build();
         }
         if self.size_edit {
           loop {
             if ui.get_column_index() == 0 {
               break;
             }
             ui.next_column();
           }
           ui.text(im_str!("Size"));
           ui.next_column();
           ui.text(im_str!("X:"));
           ui.same_line(0.0);
           ui.input_float(im_str!("##Sizex"), &mut self.size.x).build();
           ui.next_column();
           ui.text(im_str!("Y:"));
           ui.same_line(0.0);
           ui.input_float(im_str!("##Sizey"), &mut self.size.y).build();
           ui.next_column();
           ui.text(im_str!("Z:"));
           ui.same_line(0.0);
           ui.input_float(im_str!("##Sizez"), &mut self.size.z).build();
         }
         //.display_format(im_str!("%.0f"))
        
      });
      
      self.name = imstr_name.to_str().to_string();
    }
  }
  
  pub fn draw(&self, draw_calls: &mut Vec<DrawCall>) {
    draw_calls.push(DrawCall::draw_model(self.position,
                                         self.size,
                                         self.rotation,
                                         self.model.to_string()));
  }
}
    /*
    if let Some(ui) = &ui {
     ui.window(im_str!("Object Details"))
        .size((300.0, 300.0), ImGuiCond::FirstUseEver)
         .build(|| {
            ui.text(im_str!("Hello world!"));
            ui.text(im_str!("This...is...imgui-rs!"));
             ui.separator();
             let mouse_pos = ui.imgui().mouse_pos();
             ui.text(im_str!(
                "Mouse Position: ({:.1},{:.1})",
                mouse_pos.0,
                mouse_pos.1
           ));
           ui.radio_button_bool(im_str!("Slider"), true);
           ui.same_line(0.0);
           ui.radio_button_bool(im_str!("Input"), false);
           
           ui.text(im_str!("Position: "));
           ui.same_line(0.0);
           ui.drag_float(im_str!(""), &mut 0.0).build();
           ui.same_line(50.0);
           ui.drag_float(im_str!(""), &mut 1.0).build();
           ui.same_line(100.0);
           ui.drag_float(im_str!(""), &mut 2.0).build();
           
           ui.separator();
           ui.input_float(im_str!("size"), &mut 0.1)
               //.display_format(im_str!("%.0f"))
               .build();
        });
    }*/

