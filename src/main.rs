extern crate pancurses;
extern crate websocket;
extern crate tokio;

use pancurses::{initscr, endwin, Input, noecho, start_color};
mod chat_window;

mod client;

fn main() {
  // let mut socket = client::Socket::new();
  // socket.send_message("Hello from the client").unwrap();

  let window = initscr();
  start_color();
  noecho();
  window.printw("BOTTLE ALPHA v0.0.1");
  window.keypad(true);
  window.mvaddstr(1, 0, "Press enter to go to the test chat mode thing");
  window.refresh();
  loop {
    match window.getch() {
      Some(Input::KeyF1) => break,
      Some(Input::KeyEnter) => { chat_window::init(&window); },
      Some(Input::Character('\n')) => { chat_window::init(&window); }
      _ => ()
    }
  }
  endwin();
}
