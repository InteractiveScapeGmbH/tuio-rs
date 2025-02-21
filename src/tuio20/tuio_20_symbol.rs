use crate::common::tuio_state::TuioState;
use crate::common::tuio_time::TuioTime;
use crate::common::vector_2d::Vector2D;
use crate::tuio20::tuio_20_component::Tuio20Component;
use crate::tuio20::tuio_20_object::Tuio20Object;

#[derive(Copy)]
pub struct Tuio20Symbol{
    component: Tuio20Component,
    type_user_id: u32,
    component_id: u32,
    // group: str,
    // data: str,
}

impl Tuio20Symbol {
    pub fn new(start_time: TuioTime, container: Tuio20Object, type_user_id: u32, component_id: u32, group: String, data: String) -> Self{
        Self{
            component: Tuio20Component::new(start_time, container, Vector2D::new(0.0,0.0), 0.0, Vector2D::new(0.0, 0.0), 0.0, 0.0, 0.0),
            type_user_id,
            component_id,
            // group,
            // data
        }
    }

    pub fn get_state(&self) -> TuioState{
        self.component.get_state()
    }

    pub fn has_changed(&self, type_user_id: u32, component_id: u32, group: String, data: String) -> bool{
        !(self.type_user_id == type_user_id && self.component_id == component_id)
    }

    pub fn update(&mut self, current_time: TuioTime, type_user_id: u32, component_id: u32, group: String, data: String){
        self.component.update(current_time, Vector2D::new(0.0,0.0), 0.0, Vector2D::new(0.0,0.0), 0.0, 0.0, 0.0);
        self.type_user_id = type_user_id;
        self.component_id = component_id;
        // self.group = group;
        // self.data = data;
    }

    pub fn remove(&mut self, current_time: TuioTime){
        self.component.remove(current_time);
    }
}