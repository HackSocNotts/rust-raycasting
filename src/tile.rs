use sdl2::pixels::Color;

#[derive(Clone, Copy, PartialEq)]
pub enum Tile {
    Wall(Color),
    Floor,
}
