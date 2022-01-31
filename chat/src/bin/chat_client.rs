use structopt::StructOpt;
use chat::actors::pair_chat_peer::*;
use chat::Result;
use chat::actors::{Listener, Writer};
use std::net::SocketAddr;
use std::thread;
use std::sync::Arc;

#[derive(StructOpt)]
struct Options{
    nom_client: String,
    adresse_serveur: String,
    port_serveur: u16,
}

fn main() -> Result<()>{
    let my_options = Options::from_args();
    let socket_pair = format!("{}:{}", my_options.adresse_serveur, my_options.nom_client);
    let chat = PairChatPeer::new(my_options.port_serveur, socket_pair.parse::<SocketAddr>()?)?;
    let my_arc = Arc::new(chat);
    let my_arc_clone = my_arc.clone();
    let my_thread = thread::spawn(move || {
        my_arc_clone.write();
        
    });
    my_arc.listen();
    return Ok(());
}