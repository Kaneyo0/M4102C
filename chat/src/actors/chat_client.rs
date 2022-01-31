use std::net::{SocketAddr, UdpSocket};
use std::str;
use crate::actors::{Listener, Writer};
use crate::{Result, BUFFER_SIZE, ContentsMessage, IdMessage};

pub struct ChatClient {
    username: String, 
    server_endpoint: SocketAddr
}

impl ChatClient {
    pub fn new(name: String, server_endpoint: SocketAddr) -> Result<Self> {
        return Ok(Self {
            username: name,
            server_endpoint,
        });
    }
}

impl Listener for ChatClient {
    fn listen(&self) -> Result<()>{
        let mut buf = [0 ; BUFFER_SIZE];
        while let Ok((nb_bytes, sender_endpoint)) = self.server_endpoint.recv_from(&mut buf) {
            let my_message = str::from_utf8(&buf[0..nb_bytes])?;
            println!("{:?}", my_message);
        }
        return Ok(())
    }
}

impl Writer for ChatClient {  
    fn dispatch_line(&self, line: &str) -> Result<()>{
        let message = line.split(":");
        if (message[0] )
        return Ok(());
    }
    
    fn write(&self) -> Result<()>{
        for line in std::io::stdin().lock().lines() {
            self.dispatch_line(&line.unwrap());
        }
        return Ok(())
    }
}