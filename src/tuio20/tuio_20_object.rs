use crate::common::tuio_state::TuioState;
use crate::common::tuio_time::TuioTime;
use crate::tuio20::tuio_20_bounds::Tuio20Bounds;
use crate::tuio20::tuio_20_pointer::Tuio20Pointer;
use crate::tuio20::tuio_20_symbol::Tuio20Symbol;
use crate::tuio20::tuio_20_token::Tuio20Token;

#[derive(Copy)]
pub struct Tuio20Object{
    start_time: TuioTime,
    current_time: TuioTime,
    session_id: i32,
    token: Option<Tuio20Token>,
    pointer: Option<Tuio20Pointer>,
    bounds: Option<Tuio20Bounds>,
    symbol: Option<Tuio20Symbol>,
    state: TuioState

}

impl Tuio20Object {
    pub fn new(start_time: TuioTime, session_id: i32) -> Self{
        Self{
            start_time,
            current_time: start_time,
            session_id,
            token: None,
            pointer: None,
            bounds: None,
            symbol: None,
            state: TuioState::Added
        }
    }

    pub fn set_token(&mut self, token: Tuio20Token){
        self.token = Some(token);
        self.state = TuioState::Added;
    }

    pub fn set_pointer(&mut self, pointer: Tuio20Pointer){
        self.pointer = Some(pointer);
        self.state = TuioState::Added;
    }

    pub fn set_bounds(&mut self, bounds: Tuio20Bounds){
        self.bounds = Some(bounds);
        self.state = TuioState::Added;
    }

    pub fn set_symbol(&mut self, symbol: Tuio20Symbol){
        self.symbol = Some(symbol);
        self.state = TuioState::Added;
    }

    pub fn contains_token(&self) -> bool{
        self.token.is_some()
    }

    pub fn contains_pointer(&self) -> bool{
        self.pointer.is_some()
    }

    pub fn contains_bounds(&self) -> bool{
        self.bounds.is_some()
    }

    pub fn contains_symbol(&self) -> bool{
        self.symbol.is_some()
    }

    pub fn contains_new_token(&self) -> bool{
        self.token.map_or(false, |token| matches!(token.get_state(), TuioState::Added))
    }

    pub fn contains_new_pointer(&self) -> bool{
        self.pointer.map_or(false, |pointer| matches!(pointer.get_state(), TuioState::Added))
    }

    pub fn contains_new_bounds(&self) -> bool{
        self.bounds.map_or(false, |bounds| matches!(bounds.get_state(), TuioState::Added))
    }

    pub fn contains_new_symbol(&self) -> bool{
        self.symbol.map_or(false, |symbol| matches!(symbol.get_state(), TuioState::Added))
    }

    pub fn update(&mut self, current_time: TuioTime){
        self.current_time = current_time;
        self.state = TuioState::Idle;
    }

    pub fn remove(&mut self, current_time: TuioTime){
        self.current_time = current_time;
        match self.token{
            Some(mut token) => token.remove(current_time),
            _ => {}
        }
        match self.pointer{
            Some(mut pointer) => pointer.remove(current_time),
            _ => {}
        }
        match self.bounds{
            Some(mut bounds) => bounds.remove(current_time),
            _ => {}
        }
        match self.symbol{
            Some(mut symbol)=> symbol.remove(current_time),
            _ => {}
        }
        self.state = TuioState::Removed;
    }
}