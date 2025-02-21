use crate::common::tuio_time::TuioTime;
use crate::common::vector_2d::Vector2D;
use crate::tuio20::tuio_20_component::Tuio20Component;
use crate::tuio20::tuio_20_object::Tuio20Object;

pub struct Tuio20Bounds{
    component: Tuio20Component,
    size: Vector2D,
    area: f32,
}

impl Tuio20Bounds{
    pub fn new(start_time: TuioTime, container: Tuio20Object, position: Vector2D, angle: f32, size: Vector2D, area: f32, velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration: f32) -> Self{
        Self{
            component: Tuio20Component::new(start_time, container, position, angle, velocity, rotation_speed, acceleration, rotation_acceleration),
            size,
            area
        }
    }

    pub fn has_changed(&self, position: Vector2D, angle: f32, size: Vector2D, area: f32, velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration: f32) -> bool{
        self.component.has_changed(position, angle, velocity, acceleration, rotation_speed, rotation_acceleration) || !(self.size == size && self.area == area)
    }

    pub fn update(&mut self, current_time: TuioTime, position: Vector2D, angle: f32, size: Vector2D, area: f32, velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration: f32){
        self.component.update(current_time, position, angle, velocity, rotation_speed, acceleration, rotation_acceleration);
        self.size = size;
        self. area = area;
    }
}