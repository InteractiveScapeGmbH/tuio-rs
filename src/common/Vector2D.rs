use std::ops;

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl ops::Add<Vector2D> for Vector2D{
    type Output = Vector2D;
    fn add(self, _rhs: Vector2D) -> Vector2D{
        Vector2D::new(self.x + _rhs.x, self.y + _rhs.y)
    }
}

impl Vector2D{
    fn new(x: f32, y: f32)->Vector2D{
        Vector2D{
            x,
            y
        }
    }
}