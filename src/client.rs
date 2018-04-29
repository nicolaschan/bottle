use websocket::{Message, OwnedMessage, ClientBuilder};
use websocket::sender::Writer;
use websocket::receiver::Reader;
use websocket::WebSocketError;
use std::net::TcpStream;
use std::thread;

pub struct Socket {
  sender: Writer<TcpStream>
}

impl Socket {
  pub fn new() -> Socket {
    let client = ClientBuilder::new("ws://bottle.dev.nc99.co")
      .unwrap()
      .connect_insecure()
      .unwrap();

    let (mut receiver, sender) = client.split().unwrap();
    thread::spawn(move || {
      for message in receiver.incoming_messages() {
        println!("{:?}", message.unwrap())
      }
    });

    Socket { sender }
  }

  pub fn send_message(&mut self, message: &str) -> Result<(), WebSocketError> {
    let message = Message::text(message);
    self.sender.send_message(&message)
  }
}