use websocket::{Message, OwnedMessage, ClientBuilder};
use websocket::sender::Writer;
use websocket::receiver::Reader;
use websocket::WebSocketError;
use std::net::TcpStream;

pub struct Socket {
  sender: Writer<TcpStream>,
  receiver: Reader<TcpStream>
}

impl Socket {
  pub fn new() -> Socket {
    let client = ClientBuilder::new("ws://localhost:8080")
      .unwrap()
      .connect_insecure()
      .unwrap();

    let (mut receiver, sender) = client.split().unwrap();
    // thread::spawn(move || {
    //   for message in receiver.incoming_messages() {
    //     println!("{:?}", message.unwrap())
    //   }
    // });

    Socket { sender, receiver }
  }

  pub fn send_message(&mut self, message: &str) -> Result<(), WebSocketError> {
    let message = Message::text(message);
    self.sender.send_message(&message)
  }

  pub fn recv_message(&mut self) -> Option<&str> {
    Some("Not yet implemented")
    // let message: WebSocketResult<OwnedMessage> = self.receiver.recv_message();
    // match message.unwrap() {
    //   OwnedMessage::Text(s) => Some(s),
    //   _ => None
    // }
  }
}