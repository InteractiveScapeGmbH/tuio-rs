use crate::common::tuio_time::TuioTime;
use crate::common::vector_2d::Vector2D;

#[derive(Copy)]
pub struct Tuio20Point{
    start_time: TuioTime,
    position: Vector2D
}

impl Tuio20Point{
    pub fn new(start_time: TuioTime, position: Vector2D) -> Self{
        Self{
            start_time,
            position
        }
    }

    pub fn has_changed(&self, position: Vector2D) -> bool{
        !(self.position == position)
    }

    pub fn update(&mut self, position: Vector2D){
        self.position = position;
    }
}