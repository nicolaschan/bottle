
use websocket::{Message, OwnedMessage, ClientBuilder};
use websocket::sender::Writer;
use websocket::WebSocketError;
use std::net::TcpStream;
use std::thread;
use std::sync::mpsc::Sender;

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
        let message = match message {
          Ok(msg) => msg,
          Err(_) => { continue; }
        };
        match message {
          OwnedMessage::Text(msg) => {
            match tx.send(msg) {
              _ => {}
            };
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