use std::ops;

#[derive(Debug)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl ops::Add<Vector2D> for Vector2D{
    type Output = Vector2D;
    fn add(self, other: Vector2D) -> Vector2D{
        Vector2D::new(self.x + other.x, self.y + other.y)
    }
}

impl ops::Sub<Vector2D> for Vector2D{
    type Output = Vector2D;
    fn sub (self, other: Vector2D) -> Vector2D{
        Vector2D::new(self.x - other.x, self.y - other.y)
    }
}

impl PartialEq for Vector2D{
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector2D{
    fn new(x: f32, y: f32)->Vector2D{
        Vector2D{
            x,
            y
        }
    }

    fn length(self) -> f32{
        ((self.x * self.x) + (self.y * self.y)).sqrt()
    }
}

#[cfg(test)]
mod tests{
    use crate::common::vector_2d::Vector2D;

    #[test]
    fn vector_equal(){
        let vec_a = Vector2D::new(1.0, 2.0);
        let vec_b = Vector2D::new(1.0, 2.0);
        assert_eq!(vec_a, vec_b);
    }

    #[test]
    fn vector_unequal(){
        let vec_a = Vector2D::new(1.2, 3.4);
        let vec_b = Vector2D::new(1.3, 3.4);
        assert_ne!(vec_a, vec_b);
    }

    #[test]
    fn vector_add(){
        let vec_a = Vector2D::new(1.2, 3.5);
        let vec_b = Vector2D::new(3.1, 2.4);
        assert_eq!(vec_a + vec_b, Vector2D::new(4.3, 5.9));
    }

    #[test]
    fn vector_subtract(){
        let vec_a = Vector2D::new(1.2, 3.5);
        let vec_b = Vector2D::new(3.1, 2.4);
        assert_ne!(vec_a - vec_b, Vector2D::new(-2.1, 1.1));
    }

    #[test]
    fn vector_length(){
        let vec = Vector2D::new(3.0, 4.0);
        assert_eq!(vec.length(), 5.0);
    }

}