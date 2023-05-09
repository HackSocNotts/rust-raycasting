extern crate sdl2;

use nalgebra::Rotation2;
use player::{Player, MOVE_SPEED};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::collections::HashSet;

mod map;
mod player;
mod ray_caster;
mod tile;

pub fn main() {
    // Create SDL context for video
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();

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

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut player = Player::new();

    let mut last_time = 0;

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
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

        // Calculate delta time (time since last loop). Needed to handle
        // movement without framerate messing it up (some Bethesda games don't
        // do this for physics)
        let current_time = timer_subsystem.performance_counter();
        let delta_time = current_time - last_time;
        last_time = current_time;

        // println!("{delta_time}");

        // Create a set of pressed Keys.
        let keys: HashSet<_> = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        // Get the mouse state. "Relative" means that the mouse position will be
        // a delta (from the last event) instead of an absolute value.
        let mouse_state = event_pump.relative_mouse_state();

        // Calculate the movement speed.
        let move_speed = MOVE_SPEED * delta_time as f64;

        // Handle movement with keyboard
        for key in keys {
            match key {
                Keycode::W => player.position += player.direction * move_speed,
                Keycode::A => {
                    player.position.x += player.direction.y * move_speed;
                    player.position.y -= player.direction.x * move_speed;
                }
                Keycode::S => player.position -= player.direction * move_speed,
                Keycode::D => {
                    player.position.x -= player.direction.y * move_speed;
                    player.position.y += player.direction.x * move_speed;
                }
                _ => {}
            }
        }

        // Rotate the player's direction using the mouse's (relative) X
        // position. 0.001 is a random small number.
        let rotation = Rotation2::new(mouse_state.x() as f64 * 0.001);
        player.direction = rotation * player.direction;

        canvas.set_draw_color(Color::RED);
        for x in 0_u32..1280 {
            let ray = ray_caster::calculate_ray(&player, x, 1280, 720);
            let start = Point::new(x as i32, ray.0);
            let end = Point::new(x as i32, ray.1);
            canvas.draw_line(start, end).expect("Failed to draw ray!");
        }

        canvas.present();
    }
}
