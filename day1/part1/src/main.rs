use std::net::TcpListener;
use std::net::TcpStream;
use chrono::{DateTime, Local};



fn main() {
    let listener =
        TcpListener::bind("127.0.0.1:7878").unwrap();

        for stream in listener.incoming() {
            match stream{
                Ok(ref stream) => {
                    let peeraddr = stream.peer_addr().unwrap();
                    let now: DateTime<Local>  = Local::now();
                    let local_addr = stream.local_addr().unwrap();
                    let formatted = now.format("%H:%M %d/%m/%y");
                    println!("Connection achieved" );
                    println!("On ip : {:?}", peeraddr.ip() );
                    println!("At port : {:?}", peeraddr.port());
                    println!("To ip: {:?}", local_addr);
                    println!("At timestamp: {}", formatted);

                }
                
                Err(ref stream) => {
                    println!("Error occured")
                }
            }
            let stream  = stream.unwrap();
        }
}