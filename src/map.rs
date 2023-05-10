use sdl2::pixels::Color;

use crate::tile::Tile;

#[rustfmt::skip]
/// For the map, we just use a 2D array for demonstration.
// pub const MAP: [[Tile; 8]; 8] = [
//     [Tile::Wall(Color::RED), Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED)],
//     [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
//     [Tile::Wall(Color::RED), Tile::Floor, Tile::Wall(Color::RED),  Tile::Floor, Tile::Floor, Tile::Wall(Color::RED),  Tile::Floor, Tile::Wall(Color::RED)],
//     [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
//     [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
//     [Tile::Wall(Color::RED), Tile::Floor, Tile::Wall(Color::RED),  Tile::Floor, Tile::Floor, Tile::Wall(Color::RED),  Tile::Floor, Tile::Wall(Color::RED)],
//     [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
//     [Tile::Wall(Color::RED), Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED)],
// ];


pub const MAP: [[Tile; 8]; 8] = [
    [Tile::Wall(Color::RED), Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED)],
    [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
    [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Wall(Color::RED)],
    [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
    [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
    [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Wall(Color::RED)],
    [Tile::Wall(Color::RED), Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall(Color::RED)],
    [Tile::Wall(Color::RED), Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED),  Tile::Wall(Color::RED)],
];
