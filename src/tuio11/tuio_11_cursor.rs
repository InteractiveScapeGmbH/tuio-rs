use std::time::Duration;
use crate::common::vector_2d::Vector2D;

#[derive(Debug, Clone, Default)]
pub struct Tuio11Cursor {
    pub(crate) session_id: i32,
    pub(crate) position: Vector2D,
    pub(crate) velocity: Vector2D,
    pub(crate) acceleration: f32,
}

impl Tuio11Cursor {
    /// Creates a new [Tuio11Cursor]
    /// # Arguments
    /// * `session_id` - a unique session ID
    /// * `position` - a normalized [Position]
    pub fn new(session_id: i32, position: Vector2D) -> Self {
        Self {
            session_id,
            position,
            velocity: Vector2D::default(),
            acceleration: 0f32,
        }
    }

    /// Returns this [Tuio11Cursor] with motion
    /// # Arguments
    /// * `velocity` - a normalized [Velocity]
    /// * `acceleration` - a normalized acceleration
    pub fn with_motion(mut self, velocity: Vector2D, acceleration: f32) -> Self {
        self.velocity = velocity;
        self.acceleration = acceleration;
        self
    }

    pub fn get_session_id(&self) -> i32 {
        self.session_id
    }

    pub fn get_position(&self) -> &Vector2D {
        &self.position
    }

    pub fn get_x_position(&self) -> f32 {
        self.position.x
    }

    pub fn get_y_position(&self) -> f32 {
        self.position.y
    }

    pub fn get_velocity(&self) -> &Vector2D {
        &self.velocity
    }

    pub fn get_x_velocity(&self) -> f32 {
        self.velocity.x
    }

    pub fn get_y_velocity(&self) -> f32 {
        self.velocity.y
    }

    pub fn get_acceleration(&self) -> f32 {
        self.acceleration
    }

    /// Updates the [Tuio11Cursor], computing its velocity and acceleration
    /// # Arguments
    /// * `delta_time` - the [Duration] since last update
    /// * `position` - the new [Position]
    pub fn update(&mut self, delta_time: Duration, position: Vector2D) {
        let delta_time = delta_time.as_secs_f32();
        let distance = (position - self.position).length();
        let delta_x = position.x - self.position.x;
        let delta_y = position.y - self.position.y;

        let last_speed = self.velocity.length();
        let speed = distance / delta_time;

        self.velocity = Vector2D {
            x: delta_x / delta_time,
            y: delta_y / delta_time,
        };

        self.acceleration = (speed - last_speed) / delta_time;
        self.position = position;
    }
}

impl PartialEq for Tuio11Cursor {
    fn eq(&self, other: &Self) -> bool {
        self.session_id == other.session_id
            && self.get_x_position() == other.get_x_position()
            && self.get_x_position() == other.get_y_position()
            && self.velocity == other.velocity
            && self.acceleration == other.acceleration
    }
}

#[cfg(test)]
mod tests {
    use std::{f32::consts::SQRT_2, time::Duration};
    use crate::tuio11::Tuio11Cursor;
    use crate::tuio11::tuio_11_cursor::Vector2D;

    #[test]
    fn cursor_update() {
        let mut cursor = Tuio11Cursor::new(0, Vector2D { x: 0., y: 0. });

        cursor.update(Duration::from_secs(1), Vector2D { x: 1., y: 1. });

        assert_eq!(cursor.get_x_position(), 1.);
        assert_eq!(cursor.get_y_position(), 1.);
        assert_eq!(cursor.get_x_velocity(), 1.);
        assert_eq!(cursor.get_y_velocity(), 1.);
        assert_eq!(cursor.get_acceleration(), SQRT_2);
    }
}
