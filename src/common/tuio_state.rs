#[derive(Copy)]
pub enum TuioState{
    Added,
    Accelerating,
    Decelerating,
    Stopped,
    Removed,
    Rotating,
    Idle,
}