use crate::common::tuio_state::TuioState;
use crate::common::tuio_time::TuioTime;
use crate::common::vector_2d::Vector2D;
use crate::tuio20::tuio_20_component::Tuio20Component;
use crate::tuio20::tuio_20_object::Tuio20Object;

#[derive(Copy)]
pub struct Tuio20Pointer{
    component: Tuio20Component,
    type_user_id: u32,
    component_id: u32,
    shear: f32,
    radius: f32,
    pressure: f32,
    pressure_speed: f32,
    pressure_acceleration: f32
}

impl Tuio20Pointer{
    fn new(start_time: TuioTime, container: Tuio20Object, type_user_id: u32, component_id: u32, position: Vector2D, angle: f32, shear: f32, radius: f32, pressure: f32, velocity: Vector2D, pressure_speed: f32, acceleration: f32, pressure_acceleration: f32) -> Self{
        Tuio20Pointer{
            component: Tuio20Component::new(start_time, container, position, angle, velocity, 0.0, acceleration, 0.0),
            type_user_id,
            component_id,
            shear,
            radius,
            pressure,
            pressure_speed,
            pressure_acceleration
        }
    }

    pub fn get_state(&self) -> TuioState{
        self.component.get_state()
    }

    pub fn has_changed(&self, type_user_id: u32, component_id: u32, position: Vector2D, angle: f32, shear: f32, radius: f32, pressure: f32, velocity: Vector2D, pressure_speed: f32, acceleration: f32, pressure_acceleration: f32) -> bool{
        self.component.has_changed(position, angle, velocity, acceleration, 0.0, 0.0) ||
        !(self.type_user_id == type_user_id &&
            self.component_id == component_id &&
            self.shear == shear &&
            self.radius == radius &&
            self.pressure == pressure &&
            self.pressure_speed == pressure_speed &&
            self.pressure_acceleration == pressure_acceleration)
    }

    pub fn update(&mut self, current_time: TuioTime, type_user_id: u32, component_id: u32, position: Vector2D, angle:f32, shear: f32, radius: f32, pressure: f32, velocity: Vector2D, pressure_speed: f32, acceleration: f32, pressure_acceleration: f32){
        self.component.update(current_time, position, angle, velocity, 0.0, acceleration, 0.0);
        self.type_user_id = type_user_id;
        self.component_id = component_id;
        self.shear = shear;
        self.radius = radius;
        self.pressure = pressure;
        self.pressure_speed = pressure_speed;
        self.pressure_acceleration = pressure_acceleration;
    }

    pub fn remove(&mut self, current_time: TuioTime){
        self.component.remove(current_time);
    }
}