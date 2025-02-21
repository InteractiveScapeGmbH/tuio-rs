use crate::common::tuio_state::TuioState;
use crate::common::tuio_time::TuioTime;

pub struct Tuio20Object{
    start_time: TuioTime,
    current_time: TuioTime,
    session_id: u32,
    state: TuioState

}

impl Tuio20Object {
    fn new(start_time: TuioTime, session_id: u32) -> Tuio20Object{
        Tuio20Object{
            start_time,
            current_time: start_time,
            session_id,
            state: TuioState::Added
        }
    }

    pub fn update(&mut self, current_time: TuioTime){

    }
}