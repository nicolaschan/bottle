extern crate pancurses;
extern crate websocket;

use pancurses::{initscr, endwin};

mod client;

fn main() {
  let mut socket = client::Socket::new();
  socket.send_message("Hello from the client").unwrap();

  let window = initscr();
  window.printw("Bottle");
  window.refresh();
  window.getch();
  endwin();
}
