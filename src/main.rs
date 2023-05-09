extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

mod map;
mod tile;

pub fn main() {
    // Create SDL context for video
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // Create a window
    // position_centered - puts the window in the middle of the screen
    let window = video_subsystem
        .window("HackSoc Raysasting", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    // Make the mouse relative - hides it like in a 3D game
    sdl_context.mouse().set_relative_mouse_mode(true);

    // Create a canvas, which we use to draw to the window
    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
