extern crate pancurses;
extern crate websocket;
extern crate tokio;

use pancurses::{initscr, endwin, Input, noecho};
mod chat_window;

mod client;

fn main() {
  let mut socket = client::Socket::new();
  socket.send_message("Hello from the client").unwrap();

  let window = initscr();
  window.printw("BOTTLE ALPHA v0.0.1");
  window.mvaddstr(1, 0, "Press 'k' to go to the test chat mode thing");
  window.refresh();
  window.keypad(true);
  noecho();
  loop {
    match window.getch() {
      Some(Input::KeyF1) => break,
      Some(Input::Character('k')) => { chat_window::init(&window); },
      Some(Input::Character(c)) => { window.addch(c); },
      Some(input) => { window.addstr(&format!("{:?}", input)); },
      None => ()
    }
  }
  endwin();
}
