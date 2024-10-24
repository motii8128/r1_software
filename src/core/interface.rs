use super::game_controller::interface::Controller;

#[derive(Debug, Clone, Copy)]
pub enum AppMessage
{
    MainLoop(Controller)
}

#[derive(Debug, Clone, Copy)]
pub struct Packet
{
    pub x : f32,
    pub y : f32,
    pub rotation : f32,
    pub valve : f32
}
impl Packet {
    pub fn new()->Packet
    {
        Packet { x: 0.0, y: 0.0, rotation: 0.0, valve: 0.0 }
    }
}