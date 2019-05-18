use std::error::Error;
use std::net::UdpSocket;

extern crate Phizer;

//fn main() -> Result<(), Box<dyn Error>> {
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, Phizer");

    let mut url = String::new();

    url.push_str(Phizer::SERVER_IP);
    url.push_str(":");
    url.push_str(&Phizer::SERVER_PORT.to_string());

    let sock = UdpSocket::bind(&url)?;
    loop {
        println!("Frogs are gay");
        //let mut rec_buffer = [1; 128];

        //sock.recv(&rec_buffer);
            
        // .iter().for_each(|connection| {
        //})
    }


}
