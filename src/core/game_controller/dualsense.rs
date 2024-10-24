extern crate hidapi;
use hidapi::HidDevice;

use super::interface::{Buttons, Dpad, JoyStick, Controller, ControllerConnectionType};

pub struct DualSenseDriver
{
    pub device:HidDevice,
    pub mode:ControllerConnectionType,
}

impl DualSenseDriver {
    pub fn task(&mut self)->Controller
    {
            let mut buf = [0_u8;256];

            match self.device.read_timeout(&mut buf, 100) {
                Ok(_size)=>{
                    let get_data = &buf[.._size];
                    let (j, btn, d, main) = convert(get_data, self.mode);

                    if get_data[0] == 49
                    {
                        self.mode = ControllerConnectionType::BLE
                    }
                    else if get_data[0] == 1{
                        self.mode = ControllerConnectionType::SERIAL
                    }

                    Controller {mode:self.mode, state:true, sticks: j, btns: btn, dpad: d , option: main}
                }
                Err(_)=>{
                    Controller {mode:self.mode,state:false, sticks:JoyStick::new(), btns:Buttons::new(), dpad:Dpad::new(), option:false}
                }
            }
    }
    // pub fn color_change(&mut self)
    // {
    //     if self.rgb.red == 0 && self.rgb.blue == 0 && self.rgb.grenn == 0
    //     {
    //         self.rgb.blue = 255;
    //     }

    //     let mut buf = [0u8; 32];
    //     buf[0] = 0x05;
    //     buf[1] = 0xFF;
    //     buf[2] = 0x04;
    //     buf[6] = self.rgb.red;
    //     buf[7] = self.rgb.grenn;
    //     buf[8] = self.rgb.blue;

    //     match self.device.write(&buf) {
    //         Ok(_d)=>{

    //         }
    //         Err(_e)=>{

    //         }
    //     }
    // }
}

fn convert(buf:&[u8], mode:ControllerConnectionType)->(JoyStick, Buttons, Dpad, bool)
{
    if mode == ControllerConnectionType::BLE
    {
        let mut joy = JoyStick{left_x:0.0, left_y:0.0, right_x:0.0, right_y:0.0};
        let mut buttons = Buttons{circle:false, triangle:false, cube:false, cross:false, r1:false, r2:false, l1:false, l2:false, right_push:false, left_push:false};
        let mut dpad = Dpad{up_key:false,down_key:false, right_key:false, left_key:false};

        joy.left_x = map(buf[2], 0.0, 255.0, -1.0, 1.0);
        joy.left_y = -1.0*map(buf[3], 0.0, 255.0, -1.0, 1.0);
        joy.right_x = map(buf[4], 0.0, 255.0, -1.0, 1.0);
        joy.right_y = -1.0*map(buf[5], 0.0, 255.0, -1.0, 1.0);


        match buf[9] {
            8=>{},
            6=>dpad.left_key = true,
            4=>dpad.down_key = true,
            2=>dpad.right_key = true,
            0=>dpad.up_key = true,
            24=>buttons.cube = true,
            40=>buttons.cross = true,
            72=>buttons.circle = true,
            136=>buttons.triangle = true,
            16=>{dpad.up_key = true; buttons.cube= true},
            32=>{dpad.up_key = true; buttons.cross= true},
            64=>{dpad.up_key = true; buttons.circle= true},
            128=>{dpad.up_key = true; buttons.triangle= true},
            18=>{dpad.right_key = true; buttons.cube=true},
            34=>{dpad.right_key = true; buttons.cross=true},
            66=>{dpad.right_key = true; buttons.circle=true},
            130=>{dpad.right_key = true; buttons.triangle=true},
            20=>{dpad.down_key = true; buttons.cube=true},
            36=>{dpad.down_key = true; buttons.cross=true},
            68=>{dpad.down_key = true; buttons.circle=true},
            132=>{dpad.down_key = true; buttons.triangle=true},
            22=>{dpad.left_key = true; buttons.cube=true},
            38=>{dpad.left_key = true; buttons.cross=true},
            70=>{dpad.left_key = true; buttons.circle=true},
            134=>{dpad.left_key = true; buttons.triangle=true},
            _=>{}
        }

        match buf[10] {
            1=>buttons.l1 = true,
            2=>buttons.r1 = true,
            4=>buttons.l2 = true,
            8=>buttons.r2 = true,
            3=>{buttons.l1 = true; buttons.r1 = true},
            5=>{buttons.l1 = true; buttons.l2 = true},
            9=>{buttons.l1 = true; buttons.r2 = true},
            6=>{buttons.l2 = true; buttons.r1 = true},
            10=>{buttons.r2 = true; buttons.r1 = true},
            12=>{buttons.l2 = true; buttons.r2 = true},
            13=>{buttons.l2 = true; buttons.r2 = true; buttons.l1 = true},
            14=>{buttons.l2 = true; buttons.r2 = true; buttons.r1 = true},
            15=>{buttons.l2 = true; buttons.r2 = true; buttons.l1 = true; buttons.r2=true},
            _=>{}
        }

        let main = match buf[10] {
            32=>true,
            _=>false
        };

        (joy, buttons, dpad, main)
    }
    else if mode == ControllerConnectionType::SERIAL
    {
        let mut joy = JoyStick{left_x:0.0, left_y:0.0, right_x:0.0, right_y:0.0};
        let mut buttons = Buttons{circle:false, triangle:false, cube:false, cross:false, r1:false, r2:false, l1:false, l2:false, right_push:false, left_push:false};
        let mut dpad = Dpad{up_key:false,down_key:false, right_key:false, left_key:false};

        joy.left_x = map(buf[1], 0.0, 255.0, -1.0, 1.0);
        joy.left_y = -1.0*map(buf[2], 0.0, 255.0, -1.0, 1.0);
        joy.right_x = map(buf[3], 0.0, 255.0, -1.0, 1.0);
        joy.right_y = -1.0*map(buf[4], 0.0, 255.0, -1.0, 1.0);

        match buf[8] {
            8=>{},
            6=>dpad.left_key = true,
            4=>dpad.down_key = true,
            2=>dpad.right_key = true,
            0=>dpad.up_key = true,
            24=>buttons.cube = true,
            40=>buttons.cross = true,
            72=>buttons.circle = true,
            136=>buttons.triangle = true,
            16=>{dpad.up_key = true; buttons.cube= true},
            32=>{dpad.up_key = true; buttons.cross= true},
            64=>{dpad.up_key = true; buttons.circle= true},
            128=>{dpad.up_key = true; buttons.triangle= true},
            18=>{dpad.right_key = true; buttons.cube=true},
            34=>{dpad.right_key = true; buttons.cross=true},
            66=>{dpad.right_key = true; buttons.circle=true},
            130=>{dpad.right_key = true; buttons.triangle=true},
            20=>{dpad.down_key = true; buttons.cube=true},
            36=>{dpad.down_key = true; buttons.cross=true},
            68=>{dpad.down_key = true; buttons.circle=true},
            132=>{dpad.down_key = true; buttons.triangle=true},
            22=>{dpad.left_key = true; buttons.cube=true},
            38=>{dpad.left_key = true; buttons.cross=true},
            70=>{dpad.left_key = true; buttons.circle=true},
            134=>{dpad.left_key = true; buttons.triangle=true},
            _=>{}
        }

        match buf[9] {
            1=>buttons.l1 = true,
            2=>buttons.r1 = true,
            4=>buttons.l2 = true,
            8=>buttons.r2 = true,
            3=>{buttons.l1 = true; buttons.r1 = true},
            5=>{buttons.l1 = true; buttons.l2 = true},
            9=>{buttons.l1 = true; buttons.r2 = true},
            6=>{buttons.l2 = true; buttons.r1 = true},
            10=>{buttons.r2 = true; buttons.r1 = true},
            12=>{buttons.l2 = true; buttons.r2 = true},
            13=>{buttons.l2 = true; buttons.r2 = true; buttons.l1 = true},
            14=>{buttons.l2 = true; buttons.r2 = true; buttons.r1 = true},
            15=>{buttons.l2 = true; buttons.r2 = true; buttons.l1 = true; buttons.r2=true},
            _=>{}
        }

        let main = match buf[9] {
            32=>true,
            _=>false
        };

        (joy, buttons, dpad, main)
    }
    else {
        (
            JoyStick{left_x:0.0, left_y:0.0, right_x:0.0, right_y:0.0},
            Buttons{circle:false, triangle:false, cube:false, cross:false, r1:false, r2:false, l1:false, l2:false, right_push:false, left_push:false},
            Dpad{up_key:false,down_key:false, right_key:false, left_key:false},
            false
        )
    }
}   

fn map(value:u8,in_min:f32, in_max:f32, out_min:f32, out_max:f32)->f32
{
    let mut result = (value as f32 - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;

    if result.abs() < 0.07
    {
        result = 0.0;
    }

    result
}
