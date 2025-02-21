use crate::common::tuio_state::TuioState;
use crate::common::tuio_time::TuioTime;
use crate::common::vector_2d::Vector2D;
use crate::tuio20::tuio_20_component::Tuio20Component;
use crate::tuio20::tuio_20_object::Tuio20Object;

#[derive(Copy)]
pub struct Tuio20Token{
    component: Tuio20Component,
    type_user_id: i32,
    component_id: i32
}

impl Tuio20Token {
    pub fn new(start_time: TuioTime, container: Tuio20Object, type_user_id: i32, component_id: i32, position: Vector2D, angle: f32, velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration: f32) -> Self{
        Self{
            component: Tuio20Component::new(start_time, container, position, angle, velocity, rotation_speed, acceleration, rotation_acceleration),
            type_user_id,
            component_id
        }
    }

    pub fn get_state(&self) -> TuioState{
        self.component.get_state()
    }

    pub fn has_changed(&self, type_user_id: i32, component_id: i32, position: Vector2D, angle: f32, velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration: f32) -> bool{
        self.component.has_changed(position, angle, velocity, acceleration, rotation_speed, rotation_acceleration) ||
            !(self.type_user_id == type_user_id && self.component_id == component_id)
    }

    pub fn update(&mut self, current_time: TuioTime, type_user_id: i32, component_id: i32, position: Vector2D, angle: f32, velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration: f32){
        self.component.update(current_time, position, angle, velocity, rotation_speed, acceleration, rotation_acceleration);
        self.type_user_id = type_user_id;
        self.component_id = component_id;
    }

    pub fn remove(&mut self, current_time: TuioTime){
        self.component.remove(current_time);
    }
}

