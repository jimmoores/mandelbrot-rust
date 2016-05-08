extern crate num;
extern crate sdl2;

use sdl2::pixels::Color;
use num::num_complex::Complex64;
use sdl2::rect::Point;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::mem;
use std::ptr;

fn mandelbrot_iterations(c: Complex64, max_iters: i32) -> i32  {
  let mut z: Complex64 = Complex64::new(0.0, 0.0);
  for i in 0..max_iters {
    if Complex64::norm_sqr(&z) > 4.0 {
      return i;
    }
    z = (z * z) + c;
  }
  return max_iters;
}

pub fn main() {
  let sdl_context = sdl2::init().unwrap();
  let video_subsystem = sdl_context.video().unwrap();
  let x_res = 800;
  let y_res = 600;
  const MAX_ITERS: usize = 256;
  let window = video_subsystem.window("mandelbrot-rust", x_res, y_res)
                              .position_centered()
                              .opengl()
                              .build()
                              .unwrap();

  let mut renderer = window.renderer().build().unwrap();

  renderer.set_draw_color(Color::RGB(0, 0, 0));
  renderer.clear();
  renderer.present();

  let mut event_pump = sdl_context.event_pump().unwrap();

  let x1: f64 = -2.0;
  let x2: f64 = 2.0;
  let y1: f64 = -2.0;
  let y2: f64 = 2.0;

  let x_delta = (x2 - x1) / x_res as f64;
  let y_delta = (y2 - y1) / y_res as f64;
  let colors = unsafe {
    // Create an uninitialized array.
    let mut array: [Color; MAX_ITERS] = mem::uninitialized();

    for (i, element) in array.iter_mut().enumerate() {
      let r = ((i & 0b111000000) >> 6) << 5;
      let g = ((i & 0b000111000) >> 3) << 5;
      let b = (i & 0b000000111) << 5;
      let color = Color::RGB(r as u8, g as u8, b as u8);
      // Overwrite `element` without running the destructor of the old value.
      // Since color does not implement Copy, it is moved.
      ptr::write(element, color)
    }

    array
  };

  'running: loop {
    for x in 0..x_res-1 {
      if !poll(&mut event_pump) { return; }
      for y in 0..y_res-1 {
        let i = x1 + (x as f64 * x_delta);
        let j = y1 + (y as f64 * y_delta);
        let iters = mandelbrot_iterations(Complex64::new(i, j), MAX_ITERS as i32 - 1);
        renderer.set_draw_color(colors[iters as usize]);
        assert!(renderer.draw_point(Point::new(x as i32, y as i32)).is_ok());
      }
    }
    renderer.present();
  }
}

fn poll(event_pump: &mut EventPump) -> bool {
    for event in event_pump.poll_iter() {
      match event {
        Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
          return false;
        },
        _ => {}
      }
    }
    return true;
}

