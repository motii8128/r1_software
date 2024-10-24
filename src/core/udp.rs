const DESTINATION : &str = "192.168.11.2:64205";
const LOCAL : &str = "192.168.11.3:64201";

use std::net::UdpSocket;

use super::interface::Packet;

pub struct UDPDriver
{
    socket : UdpSocket,
    send : Packet,
    prev : Packet
}

impl UDPDriver {
    pub fn new()->UDPDriver
    {
        let sock = UdpSocket::bind(LOCAL).unwrap();
        println!("Create UDPDriver.");

        UDPDriver { socket: sock , send : Packet::new(), prev : Packet::new()}
    }

    pub fn send_packet(&mut self, target : Packet)
    {
        let mut vec = Packet::new();
        vec.x = target.x - self.prev.x;
        vec.y = target.y - self.prev.y;
        vec.rotation = target.rotation - self.prev.rotation;
        
        if vec.x > 0.0
        {
            self.send.x += 0.01;
        }
        else if vec.x < 0.0
        {
            self.send.x -= 0.01;
        }

        if vec.y > 0.0
        {
            self.send.y += 0.01;
        }
        else if vec.y < 0.0
        {
            self.send.y -= 0.01;
        }

        if vec.rotation > 0.0
        {
            self.send.rotation += 0.01;
        }
        else if vec.rotation < 0.0
        {
            self.send.rotation -= 0.01;
        }
        

        self.send.valve = target.valve;

        let diagonal_rate = (2.0_f32.sqrt()) / 2.0;

        let mut front = ((self.send.x + self.send.rotation*0.49) * 1023.0) as i32;
        let mut rl = ((-0.5*self.send.x + self.send.y*diagonal_rate + self.send.rotation*0.49) * 1023.0) as i32;
        let mut rr = ((-0.5*self.send.x - self.send.y*diagonal_rate + self.send.rotation*0.49)*1023.0) as i32;
        let valve = (1023.0 * self.send.valve as f32) as i32;

        if front.abs() < 100
        {
            front = 0
        }
        else if front > 1023
        {
            front = 1023
        }
        else if front < -1023
        {
            front = -1023
        }

        if rl.abs() < 100
        {
            rl = 0
        }
        else if rl > 1023
        {
            rl = 1023
        }
        else if rl < -1023
        {
            rl = -1023
        }

        if rr.abs() < 100
        {
            rr = 0
        }
        else if rr > 1023
        {
            rr = 1023
        }
        else if rr < -1023
        {
            rr = -1023
        }

        let p = format!("{},{},{},{}e", front+1023, rl+1023, rr+1023, valve+1023);
        

        match self.socket.send_to(p.as_bytes(), DESTINATION) {
            Ok(_size)=>{
                println!("{}", p)
            }
            Err(_e)=>{

            }
        }

        self.prev = self.send;
    }

    pub fn receive_packet(&mut self)->(bool, bool)
    {
        let mut buf = [0_u8; 10];
        match self.socket.recv(&mut buf) {
            Ok(size)=>{
                let get_data = &buf[..size];

                let mut str = String::from_utf8_lossy(get_data).to_string();

                // println!("Receive: {}", str);

                let mut sw2 = str.split_off(1);

                // println!("{}div{}", sw2, str);

                sw2.remove(0);

                let sw2_value = sw2.parse::<i32>().unwrap();
                let sw1_value = str.parse::<i32>().unwrap();

                (sw1_value == 1, sw2_value == 1)
            }
            Err(_e)=>{
                (false, false)
            }
        }
    }
}