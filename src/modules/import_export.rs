use csv;

use crate::modules::WorldObject;

use cgmath::Vector3;

use std::fs::File;
use std::fs;

pub fn get_models() -> Vec<(String, String, bool)> {
  fs::create_dir_all("./Models");
  let paths = fs::read_dir("./Models/").unwrap();
  
  let mut models = Vec::new();
  
  for path in paths {
    println!("{:?}", path);
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

pub fn export(scene_name: String, world_objects: &Vec<WorldObject>) {
  fs::create_dir_all("./Scenes/".to_owned() + &scene_name);
  let mut file = csv::Writer::from_path("./Scenes/".to_owned() + &scene_name + "/" + &scene_name + ".csv").unwrap();
  file.write_record(&["id", "name", "model", "location", "x", "y", "z", "rot_x", "rot_y", "rot_z", "size_x", "size_y", "size_z"]).unwrap();
  for object in world_objects {
    let id = object.id().to_string();
    let name = object.name().to_string();
    let model = object.model();
    let location = object.location();
    let x = object.position().x.to_string();
    let y = object.position().y.to_string();
    let z = object.position().z.to_string();
    let rot_x = object.rotation().x.to_string();
    let rot_y = object.rotation().y.to_string();
    let rot_z = object.rotation().z.to_string();
    let size_x = object.size().x.to_string();
    let size_y = object.size().y.to_string();
    let size_z = object.size().z.to_string();
    file.write_record(&[id, name, model, location, x, y, z, rot_x, rot_y, rot_z, size_x, size_y, size_z]).unwrap();
  }
  
  file.flush().unwrap();
}

pub fn import(scene_name: String) -> (Vec<(String, String)>, Vec<WorldObject>) {
  let mut world_objects = Vec::new();
  let mut used_models: Vec<(String, String)> = Vec::new();
  
  //let file = File::open("./".to_owned() + &scene_name.to_string() + "test.csv").unwrap();
  if let Ok(file) = File::open("./Scenes/".to_owned() + &scene_name + "/" + &scene_name + ".csv") {
    let mut reader = csv::Reader::from_reader(file);
    
    for whole_object in reader.records() {
      let object = whole_object.unwrap();
      let id: u32 = object[0].parse().unwrap();
      let name: String = object[1].parse().unwrap();
      let model: String = object[2].parse().unwrap();
      let location: String = object[3].parse().unwrap();
      let x: f32 = object[4].parse().unwrap();
      let y: f32 = object[5].parse().unwrap();
      let z: f32 = object[6].parse().unwrap();
      let rot_x: f32 = object[7].parse().unwrap();
      let rot_y: f32 = object[8].parse().unwrap();
      let rot_z: f32 = object[9].parse().unwrap();
      let size_x: f32 = object[10].parse().unwrap();
      let size_y: f32 = object[11].parse().unwrap();
      let size_z: f32 = object[12].parse().unwrap();
      
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
      
      world_objects.push(WorldObject::new_with_name(id, name, scene_name.to_string(), model, location,
                                          Vector3::new(x, y, z),
                                          Vector3::new(rot_x, rot_y, rot_z),
                                          Vector3::new(size_x, size_y, size_z)));
    }
  } else {
    println!("Import failed");
  }
  
  (used_models, world_objects)
}
