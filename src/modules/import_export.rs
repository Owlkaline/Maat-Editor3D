use csv;

use crate::modules::WorldObject;

use cgmath::Vector3;

use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::process;

pub fn export(world_objects: &Vec<WorldObject>) {
  let mut file = csv::Writer::from_path("./test.csv").unwrap();
  file.write_record(&["id", "model", "x", "y", "z", "rot_x", "rot_y", "rot_z", "size_x", "size_y", "size_z"]).unwrap();
  for object in world_objects {
    let id = object.id().to_string();
    let model = object.model();
    let x = object.position().x.to_string();
    let y = object.position().y.to_string();
    let z = object.position().z.to_string();
    let rot_x = object.rotation().x.to_string();
    let rot_y = object.rotation().y.to_string();
    let rot_z = object.rotation().z.to_string();
    let size_x = object.size().x.to_string();
    let size_y = object.size().y.to_string();
    let size_z = object.size().z.to_string();
    file.write_record(&[id, model, x, y, z, rot_x, rot_y, rot_z, size_x, size_y, size_z]).unwrap();
  }
  
  file.flush().unwrap();
}

pub fn import() -> Vec<WorldObject> {
  let mut world_objects = Vec::new();
  
  let file = File::open("./test.csv").unwrap();
  
  let mut reader = csv::Reader::from_reader(file);
  
  for whole_object in reader.records() {
    let object = whole_object.unwrap();
    let id: u32 = object[0].parse().unwrap();
    let model: String = object[1].parse().unwrap();
    let x: f32 = object[2].parse().unwrap();
    let y: f32 = object[3].parse().unwrap();
    let z: f32 = object[4].parse().unwrap();
    let rot_x: f32 = object[5].parse().unwrap();
    let rot_y: f32 = object[6].parse().unwrap();
    let rot_z: f32 = object[7].parse().unwrap();
    let size_x: f32 = object[8].parse().unwrap();
    let size_y: f32 = object[9].parse().unwrap();
    let size_z: f32 = object[10].parse().unwrap();
    
    world_objects.push(WorldObject::new(id, model, 
                                        Vector3::new(x, y, z),
                                        Vector3::new(rot_x, rot_y, rot_z),
                                        Vector3::new(size_x, size_y, size_z)));
  }
  
  world_objects
}
