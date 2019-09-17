use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;
use websocket::sender::Writer;
use websocket::WebSocketError;
use websocket::{ClientBuilder, Message, OwnedMessage};

pub struct Socket {
    sender: Writer<TcpStream>,
}

impl Socket {
    pub fn new(tx: Sender<Vec<u8>>) -> Socket {
        let client = ClientBuilder::new("ws://localhost:8080")
            .unwrap()
            .connect_insecure()
            .unwrap();

        let (mut receiver, sender) = client.split().unwrap();
        thread::spawn(move || {
            for message in receiver.incoming_messages() {
                let message = match message {
                    Ok(msg) => msg,
                    Err(_) => {
                        continue;
                    }
                };
                match message {
                    OwnedMessage::Binary(msg) => {
                        match tx.send(msg) {
                            _ => {}
                        };
                    }
                    _ => {}
                };
            }
        });

        Socket { sender }
    }

    pub fn send_message(&mut self, message: Vec<u8>) -> Result<(), WebSocketError> {
        self.sender.send_message(&Message::binary(message))
    }
}
