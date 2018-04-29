
use websocket::{Message, OwnedMessage, ClientBuilder};
use websocket::sender::Writer;
use websocket::receiver::Reader;
use websocket::WebSocketError;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;

use chat_window::ChatWindow;

pub struct Socket {
  sender: Writer<TcpStream>
}

impl Socket {
  pub fn new(tx: Sender<String>) -> Socket {
    let client = ClientBuilder::new("ws://bottle.dev.nc99.co")
      .unwrap()
      .connect_insecure()
      .unwrap();

    let (mut receiver, sender) = client.split().unwrap();
    thread::spawn(move || {
      for message in receiver.incoming_messages() {
        let message = message.unwrap();
        match message {
          OwnedMessage::Text(msg) => {
            tx.send(msg);
          },
          _ => {}
        };
      }
    });

    Socket { sender }
  }

  pub fn send_message(&mut self, message: &str) -> Result<(), WebSocketError> {
    let message = Message::text(message);
    self.sender.send_message(&message)
  }
}