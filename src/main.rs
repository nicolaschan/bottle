extern crate pancurses;
extern crate websocket;

use std::thread;
use std::sync::mpsc::channel;

use pancurses::{initscr, endwin, Input, noecho, start_color};
mod chat_window;

fn main() {
  let (tx, rx) = channel();

  thread::spawn(move || {
    let window = initscr();
    let mut chat = chat_window::ChatWindow::new(&window);
    start_color();
    noecho();
    window.printw("BOTTLE ALPHA v0.1.0");
    window.keypad(true);
    window.mvaddstr(1, 0, "Press enter to go to the test chat mode thing");
    pancurses::use_default_colors();
    window.refresh();
    match window.getch() {
      Some(Input::KeyF1) => {},
      Some(Input::KeyEnter) | Some(Input::Character('\n')) => {
        window.nodelay(true);
        window.mv(window.get_max_y() / 2, window.get_max_x() / 2 - 5);
        window.printw("loading...");
        window.refresh();
        chat.run(rx, tx);
      },
      _ => ()
    }
    endwin();
  });

  loop {}
}
