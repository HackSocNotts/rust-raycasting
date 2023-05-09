use crate::tile::Tile;

#[rustfmt::skip]
/// For the map, we just use a 2D array for demonstration.
// pub const MAP: [[Tile; 8]; 8] = [
//     [Tile::Wall, Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
//     [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
//     [Tile::Wall, Tile::Floor, Tile::Wall,  Tile::Floor, Tile::Floor, Tile::Wall,  Tile::Floor, Tile::Wall],
//     [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
//     [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
//     [Tile::Wall, Tile::Floor, Tile::Wall,  Tile::Floor, Tile::Floor, Tile::Wall,  Tile::Floor, Tile::Wall],
//     [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
//     [Tile::Wall, Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
// ];


pub const MAP: [[Tile; 8]; 8] = [
    [Tile::Wall, Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
    [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
    [Tile::Wall, Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Wall],
    [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
    [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
    [Tile::Wall, Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Floor, Tile::Floor,  Tile::Floor, Tile::Wall],
    [Tile::Wall, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Floor, Tile::Wall],
    [Tile::Wall, Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall,  Tile::Wall],
];
