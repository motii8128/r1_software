// #[derive(Debug, PartialEq, Clone, Copy, Eq)]
// pub enum ControllerName
// {
//     DualShock4,
//     DualSense,
// }

// impl std::fmt::Display for ControllerName {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 ControllerName::DualSense=>"DualSense",
//                 ControllerName::DualShock4=>"DualShock4",
//             }
//         )
//     }
// }

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum ControllerConnectionType
{
    BLE,
    SERIAL
}
impl std::fmt::Display for ControllerConnectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ControllerConnectionType::BLE=>"Bluetooth",
                ControllerConnectionType::SERIAL=>"Serial"
            }
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Controller
{
    pub mode:ControllerConnectionType,
    pub state:bool,
    pub sticks:JoyStick,
    pub btns:Buttons,
    pub dpad:Dpad,
    pub option:bool
}

impl Controller {
    pub fn new()->Controller
    {
        Controller { mode:ControllerConnectionType::BLE,state:true, sticks: JoyStick::new(), btns: Buttons::new(), dpad: Dpad::new() , option:false}
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JoyStick
{
    pub left_x:f32,
    pub left_y:f32,
    pub right_x:f32,
    pub right_y:f32,
}
impl JoyStick {
    pub fn new()->JoyStick
    {
        JoyStick { left_x: 0.0, left_y: 0.0, right_x: 0.0, right_y: 0.0 }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dpad
{
    pub up_key:bool,
    pub down_key:bool,
    pub left_key:bool,
    pub right_key:bool,   
}
impl Dpad {
    pub fn new()->Dpad
    {
        Dpad { up_key: false, down_key: false, left_key: false, right_key: false }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Buttons
{
    pub circle:bool,
    pub cross:bool,
    pub triangle:bool,
    pub cube:bool,
    pub r1:bool,
    pub r2:bool,
    pub l1:bool,
    pub l2:bool,
    pub left_push:bool,
    pub right_push:bool
}
impl Buttons {
    pub fn new()->Buttons
    {
        Buttons { circle: false, cross: false, triangle: false, cube: false, r1: false, r2: false, l1: false, l2: false, left_push: false, right_push: false }
    }
}
