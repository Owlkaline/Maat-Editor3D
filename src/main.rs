extern crate winit;
extern crate maat_graphics;
extern crate maat_input_handler;
extern crate cgmath;
extern crate rand;

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

fn benchmark(draw_calls: &mut Vec<DrawCall>, dimensions: [f32; 2]) {
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(dimensions[0] - 80.0, 15.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(1.0, 1.0, 1.0, 1.0), 
                                           "v".to_string() + &MAJOR.to_string() + "." + &MINOR.to_string() + "." + &PATCH.to_string(), 
                                           "Arial".to_string()));
}

fn fps_overlay(draw_calls: &mut Vec<DrawCall>, dimensions: [f32; 2], fps: f64) {
  let  mut fps = fps.to_string();
  fps.truncate(6);
  
  draw_calls.push(DrawCall::draw_text_basic(Vector2::new(32.0, dimensions[1]-32.0), 
                                           Vector2::new(64.0, 64.0), 
                                           Vector4::new(0.0, 0.0, 0.0, 1.0), 
                                           "fps: ".to_string() + &fps, 
                                           "Arial".to_string()));
}

fn main() {
  let mut graphics = CoreMaat::new("Maat Editor".to_string(), (MAJOR) << 22 | (MINOR) << 12 | (PATCH), 1280.0, 720.0, true);
  
  graphics.preload_font(String::from("Arial"),
                        String::from("./resources/Fonts/TimesNewRoman.png"),
                        include_bytes!("../resources/Fonts/TimesNewRoman.fnt"));
  graphics.preload_texture(String::from("Logo"), 
                           String::from("./resources/Textures/Logo.png"));
  
  
  graphics.load_shaders();
  graphics.init();
  
  graphics.set_clear_colour(0.2, 0.2, 0.2, 1.0);
  
  let mut game: Box<Scene> = Box::new(LoadScreen::new());
  
  let mut draw_calls: Vec<DrawCall> = Vec::with_capacity(100);
  
  let mut delta_time;
  let mut last_time = time::Instant::now();
  
  let mut done = false;
  let mut dimensions;
  let dpi = 1.0;
  let mut dpi_changed = false;
  
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
    
    dimensions = {
      let dim = graphics.get_dimensions();
      [dim.width as f32 * dpi, dim.height as f32  * dpi]
    };
    
    if game.scene_finished() {
      game = game.future_scene(Vector2::new(dimensions[0], dimensions[1]));
    }
    
    game.set_window_dimensions(Vector2::new(dimensions[0], dimensions[1]));
    
    game.draw(&mut draw_calls);
    
    game.update(delta_time as f32);
    
    benchmark(&mut draw_calls, dimensions);
    fps_overlay(&mut draw_calls, dimensions, last_fps);
    
    let model_details = graphics.pre_draw();
    graphics.draw(&draw_calls, delta_time as f32);
    graphics.post_draw();
    
    draw_calls.clear();
    
    game.reset_scroll_value();
    for (reference, size) in &model_details {
      game.add_model_size(reference.to_string(), *size);
    }
    
    let mut resized = false;
    
    let _height = graphics.get_dimensions().height as f32;
    graphics.get_events().poll_events(|ev| {
      match ev {
        winit::Event::WindowEvent{ event, .. } => {
          match event {
            winit::WindowEvent::Resized(_new_size) => {
              resized = true;
            },
            winit::WindowEvent::CursorMoved{device_id: _, position, modifiers: _} => {
              game.set_mouse_position(Vector2::new(position.x as f32, dimensions[1] / dpi - position.y as f32));
            },
            winit::WindowEvent::CloseRequested => {
              done = true;
            },
            winit::WindowEvent::HiDpiFactorChanged(new_dpi) => {
              println!("Dpi Changed: {}", new_dpi);
              dpi_changed = true;
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
    });
    
    if dpi_changed {
      dpi_changed = false;
    }
    
    if resized {
      graphics.screen_resized();
    }
    
    if done { break; }
  }
  
  println!("Game Loop ended");
}
