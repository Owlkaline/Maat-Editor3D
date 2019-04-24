extern crate winit;
extern crate maat_graphics;
extern crate maat_input_handler;
extern crate cgmath;
extern crate rand;

use maat_graphics::imgui::*;

mod modules;

use crate::modules::scenes::Scene;
use crate::modules::scenes::LoadScreen;

use maat_graphics::graphics::CoreRender;
use maat_graphics::CoreMaat;
use maat_graphics::DrawCall;

use cgmath::{Vector2, Vector4};

use std::time;

const MAJOR: u32 = 0;
const MINOR: u32 = 1;
const PATCH: u32 = 0;

fn benchmark(draw_calls: &mut Vec<DrawCall>, dimensions: Vector2<f32>) {
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(dimensions.x - 80.0, 15.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "v".to_string() + &MAJOR.to_string() + "." + &MINOR.to_string() + "." + &PATCH.to_string(), 
                                           "Arial".to_string()));
}

fn fps_overlay(draw_calls: &mut Vec<DrawCall>, dimensions: Vector2<f32>, fps: f64) {
  let  mut fps = fps.to_string();
  fps.truncate(6);
  
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(32.0, dimensions.y-32.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                           "fps: ".to_string() + &fps, 
                                           "Arial".to_string()));
}

fn main() {
  let mut imgui = ImGui::init();
  let mut graphics = CoreMaat::new("Maat Editor".to_string(), (MAJOR) << 22 | (MINOR) << 12 | (PATCH), 1920.0, 1080.0, true).use_imgui(&mut imgui);
  
  graphics.preload_font(String::from("Arial"),
                        String::from("./resources/Fonts/TimesNewRoman.png"),
                        include_bytes!("../resources/Fonts/TimesNewRoman.fnt"));
  graphics.preload_texture(String::from("Logo"), 
                           String::from("./resources/Textures/Logo.png"));
  
  graphics.add_model("Hexagon".to_string(), "./windys-modeling-agency/Unfinished/hexagon.glb".to_string());
  
  graphics.load_shaders();
  
  graphics.set_clear_colour(0.2, 0.2, 0.2, 1.0);
  
  let mut game: Box<Scene> = Box::new(LoadScreen::new());
  
  let mut draw_calls: Vec<DrawCall> = Vec::with_capacity(100);
  
  let mut delta_time;
  let mut last_time = time::Instant::now();
  
  let mut done = false;
  let mut dimensions;
  
  let mut frame_counter = 0;
  let mut fps_timer = 0.0;
  let mut last_fps = 0.0;
  
  loop {
    delta_time = last_time.elapsed().subsec_nanos() as f64 / 1000000000.0 as f64;
    last_time = time::Instant::now();
    
    frame_counter += 1;
    fps_timer += delta_time;
    if fps_timer > 1.0 {
      last_fps = frame_counter as f64 * (1.0/fps_timer);
      fps_timer = 0.0;
      frame_counter = 0;
    }
    
    dimensions = graphics.get_virtual_dimensions();
    
    if game.scene_finished() {
      game = game.future_scene(dimensions);
    }
    
    game.set_window_dimensions(dimensions);
    
    graphics.init();
    
    let frame_size = graphics.imgui_window(&mut imgui);
    let ui = imgui.frame(frame_size, delta_time as f32);
    
    game.draw(&mut draw_calls, Some(&ui));
    game.update(delta_time as f32);
    
    benchmark(&mut draw_calls, dimensions);
    fps_overlay(&mut draw_calls, dimensions, last_fps);
    
    let model_details = graphics.pre_draw();
    graphics.draw(&draw_calls, Some(ui), delta_time as f32);
    graphics.post_draw();
    
    draw_calls.clear();
    
    game.reset_scroll_value();
    for (reference, size) in &model_details {
      game.add_model_size(reference.to_string(), *size);
    }
    
    let events = graphics.get_events(Some(&mut imgui));
    let mouse_pos = graphics.get_mouse_position();
    
    game.set_mouse_position(mouse_pos);
    
    for ev in events {
      match &ev {
        winit::Event::WindowEvent{ event, .. } => {
          match event {
            winit::WindowEvent::CloseRequested => {
              done = true;
            },
            _ => {
              if game.handle_input(event) {
                done = true;
              }
            }
          }
        },
        _ => {},
      }
    }
    
    if done { break; }
  }
  
  println!("Game Loop ended");
}
