use std::net::{SocketAddr, UdpSocket};
use crate::actors::{Listener, Writer, Server};
use crate::{Result, BUFFER_SIZE, ContentsMessage, IdMessage};

pub struct ChatServer {
    socket_server: UdpSocket
}

impl ChatServer {
    pub fn new(port: u16) -> Result<Self> {
        return Ok(Self {
            socket_server: UdpSocket::bind(format!("127.0.0.1:{}", port))?
        });
    }
}

impl Server for ChatServer {
    fn handle_message(&self, sender_address: SocketAddr, received_message: &[u8]) -> Result<()>{
        
    }
    
    fn serve(&self) -> Result<()>{

    }
}