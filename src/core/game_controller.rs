pub mod dualsense;
pub mod dualshock4;
pub mod interface;

extern crate hidapi;
use hidapi::HidApi;
use interface::{Controller, ControllerConnectionType};

use super::thread;

pub struct ControllerManager
{
    pub controller_1 : thread::AsyncNode<Controller>,
    api:HidApi,
    pub input:Controller,
}

impl ControllerManager {
    pub fn new()->ControllerManager
    {
        ControllerManager {
            controller_1 : thread::AsyncNode::new(),
            api: HidApi::new().unwrap() ,
            input : Controller::new(),
        }
    }

    pub fn spawn_driver(&mut self)
    {
        let publisher_ = self.controller_1.get_publisher();
        
        match self.api.open(1356, 3302)
        {
            Ok(dr)=>{
                std::thread::spawn(move ||{
                    let mut dualsense = dualsense::DualSenseDriver{device : dr, mode : ControllerConnectionType::BLE};
                    loop {
                        let input = dualsense.task();

                        let _ = publisher_.send(input).unwrap();
                    }
                });
            }
            Err(_e)=>{

            }
        }
        println!("Spawned DualSense.");
    }
}