use std::cmp::{max, min};

use nalgebra::Vector2;
use sdl2::rect::Point;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::map::MAP;
use crate::player::Player;
use crate::tile::Tile;

/// The orientation of the wall we've hit. This can be used to shade the wall
/// depending on whether it was a vertical or horizontal hit.
enum HitSide {
    /// A horizontal hit (known by lodev as x-side)
    Horizontal,

    /// A vertical hit (known by lodev as y-side)
    Vertical,
}

/// Calculates the start and stop position for the ray line for the given x.
/// Player is a read-only borrow, as it is not modified in this function.
pub fn draw_ray(player: &Player, x: u32, canvas: &mut Canvas<Window>) {
    // Get the width and height from SDL
    let (window_width, window_height) = canvas.window().size();

    // camera_x is the x coordinate on the camera plane. The left of the screen
    // gets -1, right gets 1. This is used to calculate the direction of the
    // ray (as the ray is passed from the player through a point in the camera
    // plane).
    let camera_x = 2.0 * x as f64 / window_width as f64 - 1.0;

    // ray_dir is the vector that we use to calculate the ray's direction.
    let ray_dir = player.direction + player.plane() * camera_x;

    // This vector says which square the ray is in.
    let mut ray_map = Vector2::new(player.position.x as i32, player.position.y as i32);

    // This vector stores the initial distance that the ray has to travel to hit
    // a wall
    let mut side_dist = Vector2::new(0.0, 0.0);

    // The distance that the ray has to travel to go from one square to the
    // next. This would usually be done with pythagoras, but it can be
    // simplified as shown below. For an explanation on why this simplification
    // works, see https://lodev.org/cgtutor/raycasting.html
    let delta_dist = Vector2::new((1.0 / ray_dir.x).abs(), (1.0 / ray_dir.y).abs());

    // The distance that the ray has travelled
    let mut perp_wall_dist = 0.0;

    // Which direction to step in. The DDA will always jump exactly one square.
    let mut step = Vector2::new(0, 0);

    // The hit side of the wall. We could technically define this as an Option,
    // but it doesn't really matter what the intial value is.
    let mut hit_side: HitSide;

    let mut hit_tile: Tile;

    // Here, we calculate our step and initial distance using the ray direction.

    // if X is less than 0, we're going left. Otherwise, we're going right.
    if ray_dir.x < 0.0 {
        step.x = -1;
        side_dist.x = (player.position.x - ray_map.x as f64) * delta_dist.x;
    } else {
        step.x = 1;
        side_dist.x = (ray_map.x as f64 + 1.0 - player.position.x) * delta_dist.x;
    }

    // If Y is less than 0, we're going down. Otherwise, we're going up.
    if ray_dir.y < 0.0 {
        step.y = -1;
        side_dist.y = (player.position.y - ray_map.y as f64) * delta_dist.y;
    } else {
        step.y = 1;
        side_dist.y = (ray_map.y as f64 + 1.0 - player.position.y) * delta_dist.y;
    }

    // Here, we perform the DDA.
    loop {
        if side_dist.x < side_dist.y {
            side_dist.x += delta_dist.x;
            ray_map.x += step.x;
            hit_side = HitSide::Horizontal;
        } else {
            side_dist.y += delta_dist.y;
            ray_map.y += step.y;
            hit_side = HitSide::Vertical;
        }

        // If we hit a wall, store the hit tile and break out of this loop
        if let Tile::Wall(_) = MAP[ray_map.x as usize][ray_map.y as usize] {
            hit_tile = MAP[ray_map.x as usize][ray_map.y as usize];
            break;
        }
    }

    // Now that we know where the wall is, we calculate the distance. This is
    // done by calculating the distance to the camera plane in order to avoid
    // fisheye.

    perp_wall_dist = match hit_side {
        HitSide::Horizontal => side_dist.x - delta_dist.x,
        HitSide::Vertical => side_dist.y - delta_dist.y,
    };

    // Calculate the height of the line to draw
    let line_height: i32 = if perp_wall_dist as i32 == 0 {
        0
    } else {
        (window_height as f64 / perp_wall_dist) as i32
    };

    // Calculate the start and end positions of the line. There's probably a
    // cleaner way of doing this
    let draw_start = max(0, -line_height / 2 + window_height as i32 / 2);
    // let draw_end = max(
    //     (window_height - 1) as i32,
    //     line_height / 2 + window_height / 2,
    // );
    let draw_end = max(0, line_height / 2 + window_height as i32 / 2);

    if let Tile::Wall(colour) = hit_tile {
        canvas.set_draw_color(colour);
        let start = Point::new(x as i32, draw_start);
        let end = Point::new(x as i32, draw_end);
        canvas.draw_line(start, end).expect("Failed to draw ray!");
    }
}
