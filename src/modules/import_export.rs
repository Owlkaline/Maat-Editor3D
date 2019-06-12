use csv;

use crate::modules::WorldObject;
use crate::modules::scenes::GameOptions;
use crate::modules::Logs;

use cgmath::Vector3;

use std::fs::File;
use std::fs;

pub fn get_models(logs: &mut Logs) -> Vec<(String, String, bool)> {
  if let Err(e) = fs::create_dir_all("./Models") {
    logs.add_error(e.to_string());
  }
  
  let paths = fs::read_dir("./Models/").unwrap();
  
  let mut models = Vec::new();
  
  for path in paths {
    models.push(path.unwrap().path().display().to_string());
  }
  
  let mut known_models = Vec::new();
  for i in 0..models.len() {
    let mut location = models[i].to_string();
    let mut name = "".to_string();
    
    let b = location.pop();
    let l = location.pop();
    let g = location.pop();
    let full_stop = location.pop();
    
    if full_stop.unwrap() == '.' && g.unwrap() == 'g' && l.unwrap() == 'l' && b.unwrap() == 'b' {
      loop {  
        let letter = location.pop();
        if let Some(letter) = letter {
          name.push_str(&letter.to_string());
          if letter == '/' {
            name.pop();
            name = name.chars().rev().collect::<String>();
            break;
          }
        } else {
          break;
        }
      }
      
      known_models.push((name, models[i].to_string(), false));
    }
  }
  
  known_models
}

pub fn export(scene_name: String, world_objects: &Vec<WorldObject>, camera_details: &GameOptions, logs: &mut Logs) {
  if let Err(e) = fs::create_dir_all("./Scenes/".to_owned() + &scene_name) {
    logs.add_error(e.to_string());
  }
  
  match csv::Writer::from_path("./Scenes/".to_owned() + &scene_name + "/" + &scene_name + ".csv") {
    Ok(mut file) => {
      file.write_record(&["id", "name", "model", "location", "instanced", "x", "y", "z", "rot_x", "rot_y", "rot_z", "size_x", "size_y", "size_z"]).unwrap();
      for object in world_objects {
        let id = object.id().to_string();
        let name = object.name().to_string();
        let model = object.model();
        let location = object.location();
        let instanced = object.instanced_rendered().to_string();
        let x = object.position().x.to_string();
        let y = object.position().y.to_string();
        let z = object.position().z.to_string();
        let rot_x = object.rotation().x.to_string();
        let rot_y = object.rotation().y.to_string();
        let rot_z = object.rotation().z.to_string();
        let size_x = object.size().x.to_string();
        let size_y = object.size().y.to_string();
        let size_z = object.size().z.to_string();
        if let Err(e) = file.write_record(&[id, name, model, location, instanced, x, y, z, rot_x, rot_y, rot_z, size_x, size_y, size_z]) {
          logs.add_error(e.to_string());
        }
      }
      
      file.flush().unwrap();
    },
    Err(e) => {
      logs.add_error(e.to_string());
    }
  }
  
  match csv::Writer::from_path("./Scenes/".to_owned() + &scene_name + "/camera.csv") {
    Ok(mut file) => {
      file.write_record(&["type", "target_id", "distance", "x", "y", "z"]).unwrap();
      
      let camera_type = camera_details.camera_type.to_string();
      let target_id = camera_details.camera_target.to_string();
      let distance = camera_details.camera_distance.to_string();
      let x = camera_details.camera_location.x.to_string();
      let y = camera_details.camera_location.y.to_string();
      let z = camera_details.camera_location.z.to_string();
      
      file.write_record(&[camera_type, target_id, distance, x, y, z]).unwrap();
      file.flush().unwrap();
    },
    Err(e) => {
      logs.add_error(e.to_string());
    }
  }
}

pub fn import(scene_name: String, logs: &mut Logs) -> (Vec<(String, String)>, Vec<WorldObject>, GameOptions) {
  let mut world_objects = Vec::new();
  let mut used_models: Vec<(String, String)> = Vec::new();
  let mut game_options = GameOptions::new();
  
  match File::open("./Scenes/".to_owned() + &scene_name + "/" + &scene_name + ".csv") {
    Ok(file) => {
      let mut reader = csv::Reader::from_reader(file);
      
      for whole_object in reader.records() {
        match whole_object {
          Ok(object) => {
            let id: u32 = object[0].parse().unwrap();
            let name: String = object[1].parse().unwrap();
            let model: String = object[2].parse().unwrap();
            let location: String = object[3].parse().unwrap();
            let instanced: bool = object[4].parse().unwrap();
            let x: f32 = object[5].parse().unwrap();
            let y: f32 = object[6].parse().unwrap();
            let z: f32 = object[7].parse().unwrap();
            let rot_x: f32 = object[8].parse().unwrap();
            let rot_y: f32 = object[9].parse().unwrap();
            let rot_z: f32 = object[10].parse().unwrap();
            let size_x: f32 = object[11].parse().unwrap();
            let size_y: f32 = object[12].parse().unwrap();
            let size_z: f32 = object[13].parse().unwrap();
            
            let mut unique = true;
            for i in 0..used_models.len() {
              if used_models[i].0 == model {
                unique = false;
                break;
              }
            }
            
            if unique {
              used_models.push((model.to_string(), location.to_string()));
            }
            
            world_objects.push(WorldObject::new_with_data(id, name, scene_name.to_string(), model, location,
                                                Vector3::new(x, y, z),
                                                Vector3::new(rot_x, rot_y, rot_z),
                                                Vector3::new(size_x, size_y, size_z),
                                                instanced));
          },
          Err(e) => {
            logs.add_error("Scene details data:".to_owned() + &e.to_string());
          }
        }
      }
    },
    Err(e) => {
      logs.add_error("Scene details: ".to_owned() + &e.to_string());
    },
  }
  
  match File::open("./Scenes/".to_owned() + &scene_name + "/camera.csv") {
    Ok(file) => {
      let mut reader = csv::Reader::from_reader(file);
      
      for whole_object in reader.records() {
        match whole_object {
          Ok(object) => {
            let camera_type: i32 = object[0].parse().unwrap();
            let target_id: i32 = object[1].parse().unwrap();
            let distance: f32 = object[2].parse().unwrap();
            let x: f32 = object[3].parse().unwrap();
            let y: f32 = object[4].parse().unwrap();
            let z: f32 = object[5].parse().unwrap();
            game_options.camera_type = camera_type;
            game_options.camera_target = target_id;
            game_options.camera_distance = distance;
            game_options.camera_location = Vector3::new(x,y,z);
          },
          Err(e) => {
            logs.add_error(e.to_string());
          }
        }
      }
    },
    Err(e) => {
      logs.add_error("Camera: ".to_owned() + &e.to_string());
    }
  }
  
  (used_models, world_objects, game_options)
}
