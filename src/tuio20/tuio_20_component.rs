use crate::common::tuio_state::TuioState;
use crate::common::tuio_time::TuioTime;
use crate::common::vector_2d::Vector2D;
use crate::tuio20::tuio_20_object::Tuio20Object;
use crate::tuio20::tuio_20_point::Tuio20Point;

pub struct Tuio20Component{
    tuio_point: Tuio20Point,
    current_time: TuioTime,
    container: Tuio20Object,
    angle: f32,
    velocity: Vector2D,
    speed: f32,
    rotation_speed: f32,
    acceleration: f32,
    rotation_acceleration: f32,
    state: TuioState,
}

impl Tuio20Component{
    pub fn new(start_time: TuioTime, container: Tuio20Object, position: Vector2D, angle: f32,
    velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration:f32) -> Self{
        Self{
            tuio_point: Tuio20Point::new(start_time, position),
            current_time: start_time,
            container,
            angle,
            velocity,
            speed: velocity.length(),
            rotation_speed,
            acceleration,
            rotation_acceleration,
            state: TuioState::Added
        }
    }

    pub fn has_changed(&self, position: Vector2D, angle:f32, velocity: Vector2D, acceleration: f32, rotation_speed: f32, rotation_acceleration: f32) -> bool{
        self.tuio_point.has_changed(position) || !(self.angle == angle && self.velocity == velocity && self.acceleration == acceleration && self.rotation_speed == rotation_speed && self.rotation_acceleration == rotation_acceleration)
    }

    pub fn update(&mut self, current_time: TuioTime, position: Vector2D, angle: f32, velocity: Vector2D, rotation_speed: f32, acceleration: f32, rotation_acceleration: f32){
        self.current_time = current_time;
        self.tuio_point.update(position);
        self.angle = angle;
        self.velocity = velocity;
        self.rotation_speed = rotation_speed;
        self.acceleration = acceleration;
        self.rotation_acceleration = rotation_acceleration;
        if self.acceleration > 0.0 {
            self.state = TuioState::Accelerating;
        }else if self.acceleration < 0.0 {
            self.state = TuioState::Decelerating;
        }else if self.rotation_acceleration != 0.0 {
            match self.state{
                TuioState::Stopped => self.state = TuioState::Rotating,
                _ => {}
            }
        }else{
            self.state = TuioState::Stopped;
        }
        self.container.update(current_time);
    }
}