# Rust Raycasting Tutorial

![Video demonstration](hacksoc-raycasting.mp4)

This project contains a basic raycaster written in Rust using SDL2. The method used is based off [this tutorial](https://lodev.org/cgtutor/raycasting.html) (which you can follow to further expand the raycaster). I've tried my best to add detailed comments that are easier to follow than the original tutorial, these can be seen in `src/ray_caster.rs`.

# Setup

This project uses SDL2, so you will need a working SDL2 install. The Rust-SDL2 repo contains good instructions for Windows, Mac, and Linux, which you can find [here](https://github.com/Rust-SDL2/rust-sdl2#sdl20-development-libraries).

# Copy-Paste Sections

To save having to type everything out by hand, here are a few bits of code that you should probably copy-paste:

* The entirety of `src/map.rs`
* This section of `src/main.rs`, which handles player movement:

```rust
// Handle movement with keyboard
for key in keys {
    match key {
        Keycode::W => new_position += player.direction * move_speed,
        Keycode::A => {
            new_position.x += player.direction.y * move_speed;
            new_position.y -= player.direction.x * move_speed;
        }
        Keycode::S => new_position -= player.direction * move_speed,
        Keycode::D => {
            new_position.x -= player.direction.y * move_speed;
            new_position.y += player.direction.x * move_speed;
        }
        _ => {}
    }
}
```

# License

This project is licensed with [The Unlicense](https://unlicense.org/). Basically, do whatever you want with it :)