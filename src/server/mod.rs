//! This module specifies the necessary Traits that a server that
//! 


pub mod wifi_comms;
pub mod bluetooth_comms; 
pub mod wire_comms;

use std::collections::HashMap;

pub trait LightReceiver { 

    fn listen(&self);

    fn recv(&self);

}

pub trait LightClient { 
    fn send(&self); 
}

// A server which keeps track of the connections or data pipeline from one Light Receiver to any type of Light Clients
pub struct Server { 
    connections : HashMap< Box<dyn LightReceiver>, Vec<Box<dyn LightClient>> >
}

