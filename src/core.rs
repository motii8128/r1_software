mod game_controller;
mod interface;
mod udp;
mod thread;

use game_controller::ControllerManager;
use iced;
use interface::{AppMessage, Packet};

pub struct R1Software
{
    gamepad_driver : game_controller::ControllerManager,
    packet : interface::Packet,
    udp_driver : udp::UDPDriver,
    switch_1 : bool,
    switch_2 : bool,
    instant : std::time::Instant,
    passed : u64
}

impl iced::Application for R1Software {
    type Executor = iced::executor::Default;
    type Message = interface::AppMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let mut gamepad_driv = ControllerManager::new();
        gamepad_driv.spawn_driver();

        let udp_ = udp::UDPDriver::new();

        println!("App Start!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
        
        (
            Self{
                gamepad_driver : gamepad_driv,
                packet : Packet::new(),
                udp_driver : udp_,
                switch_1 : false,
                switch_2 : false,
                instant : std::time::Instant::now(),
                passed : 0
            },
            iced::Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("R1Software")
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::KanagawaDragon
    }

    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        let time_t = iced::widget::text(format!("{}:{}", self.passed / 60, self.passed % 60)).size(150);
        let str = format!("x : {:3.0}\ny : {:3.0}\nrotation : {:3.0}\nValve : {:3.0}", 
        self.packet.x*100.0, 
        self.packet.y*100.0,
        self.packet.rotation*100.0,
        self.packet.valve*100.0);
        let str_t = iced::widget::text(str).size(100);

        let sw_str = format!("Open : {}\nClose : {}", self.switch_1, self.switch_2);
        let sw_t = iced::widget::text(sw_str).size(100);

        let col = iced::widget::column![time_t, str_t, sw_t].align_items(iced::Alignment::Center);

        iced::widget::row![col].align_items(iced::Alignment::Center).into()
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            AppMessage::MainLoop(debug)=>{
                self.passed = self.instant.elapsed().as_secs();
                self.gamepad_driver.input = debug;

                let mut p = interface::Packet::new();
                p.x = self.gamepad_driver.input.sticks.left_x;
                p.y = self.gamepad_driver.input.sticks.left_y;
                p.rotation = self.gamepad_driver.input.sticks.right_x;
                
                if self.gamepad_driver.input.btns.r1
                {
                    p.valve = 0.7
                }
                else if self.gamepad_driver.input.btns.r2
                {
                    p.valve = -0.7
                }
                else {
                    p.valve = 0.0
                }

                if self.gamepad_driver.input.dpad.up_key
                {
                    p.y = 2.0
                }
                else if self.gamepad_driver.input.dpad.down_key
                {
                    p.y = -2.0
                }

                if self.gamepad_driver.input.dpad.right_key
                {
                    p.x = 1.0
                }
                else if self.gamepad_driver.input.dpad.left_key
                {
                    p.x = -1.0
                }

                self.udp_driver.send_packet(p);
                // let (sw_1, sw_2) = self.udp_driver.receive_packet();

                // self.switch_1 = sw_1;
                // self.switch_2 = sw_2;

                self.packet = p;
            }
        }

        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        iced::subscription::unfold(
            "gamepad_subscription", 
            self.gamepad_driver.controller_1.get_subscriber(), 
            move |mut subscriber| async move{
                let get = subscriber.as_mut().unwrap().recv().await.unwrap();

                (AppMessage::MainLoop(get), subscriber)
            })
    }
}