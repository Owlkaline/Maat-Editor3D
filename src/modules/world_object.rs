use maat_graphics::DrawCall;
use maat_graphics::imgui::*;

use hlua;
use hlua::Lua;

use cgmath::{Vector2, Vector3};

#[derive(Clone)]
pub struct WorldObject {
  reference_num: u32,
  model: String,
  location: String,
  position: Vector3<f32>,
  rotation: Vector3<f32>,
  size: Vector3<f32>,
  position_edit: bool,
  size_edit: bool,
  rotation_edit: bool,
}

impl WorldObject {
  pub fn new_empty(reference_num: u32, model: String, location: String) -> WorldObject {
    let mut object = WorldObject {
      reference_num,
      model,
      location,
      position: Vector3::new(0.0, 0.0, 0.0),
      rotation: Vector3::new(0.0, 0.0, 0.0),
      size: Vector3::new(1.0, 1.0, 1.0),
      position_edit: false,
      size_edit: false,
      rotation_edit: false,
    };
    
    object
  }
  
  pub fn new(reference_num: u32, model: String, location: String, position: Vector3<f32>, rotation: Vector3<f32>, size: Vector3<f32>) -> WorldObject {
    let mut object = WorldObject {
      reference_num,
      model,
      location,
      position,
      rotation,
      size,
      position_edit: false,
      size_edit: false,
      rotation_edit: false,
    };
    
    object
  }
  
  pub fn get_id(&mut self) -> i64 {
    self.reference_num as i64
  }
  
  pub fn id(&self) -> u32 {
    self.reference_num
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
      lua.set("x", self.reference_num);
      //  lua.execute_from_reader::<(), _>(File::open(&Path::new("test.lua")).unwrap());
      {
        let mut update: hlua::LuaFunction<_> = lua.get("update").unwrap();
        update.call::<()>().unwrap();
      }
      //lua.execute::<()>("x = x + 1").unwrap();
      let x: i32 = lua.get("x").unwrap();  // x is equal to 3
      println!("object id: {}, lua add one: {}", self.reference_num, x);
    }
  }
  
  pub fn update(&mut self, ui: Option<&Ui>, lua: &mut Option<&mut Lua>, window_dim: Vector2<f32>, _delta_time: f32) {
     if let Some(ui) = &ui {
       let ui_window_size = (450.0, 200.0);
       ui.window(im_str!("Object Being Placed"))
       .size(ui_window_size, ImGuiCond::FirstUseEver)
       .position((window_dim.x-ui_window_size.0-20.0, 432.0), ImGuiCond::FirstUseEver)
       //.always_auto_resize(true)
       .build(|| {
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

