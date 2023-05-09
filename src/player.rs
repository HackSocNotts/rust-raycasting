use nalgebra::Vector2;

/// The move speed of the player, to be compensated with a delta time.
pub const MOVE_SPEED: f64 = 0.000000001;

/// The Player struct stores our player's position, as well as their direction.
pub struct Player {
    pub position: Vector2<f64>,
    pub direction: Vector2<f64>,
}

impl Player {
    /// Creates a new player with default values
    pub fn new() -> Player {
        Player {
            position: Vector2::new(4.0, 4.0),
            direction: Vector2::new(1.0, 1.0),
        }
    }
}
