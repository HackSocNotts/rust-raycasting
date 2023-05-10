use nalgebra::Vector2;

/// The move speed of the player, to be compensated with a delta time.
pub const MOVE_SPEED: f64 = 0.00000001;

/// The Player struct stores our player's position, direction, and plane
pub struct Player {
    /// The position of the player, effectively its x and y.
    pub position: Vector2<f64>,

    /// The direction that the player is facing.
    pub direction: Vector2<f64>,
}

impl Player {
    /// Creates a new player with default values
    pub fn new() -> Player {
        Player {
            position: Vector2::new(3.0, 3.0),
            direction: Vector2::new(1.0, 1.0),
        }
    }

    /// Gets the camera plane, which is perpendicular to direction
    pub fn plane(&self) -> Vector2<f64> {
        // Note: you can mess about with the y value of this vector to change
        // FOV (dividing it by 2 will bring FOV from 90 to 45)
        Vector2::new(-self.direction.y, self.direction.x)
    }
}
