extern crate pancurses;
extern crate websocket;

use pancurses::{initscr, endwin, Input, noecho, start_color};
mod chat_window;

fn main() {
  let window = initscr();
  start_color();
  noecho();
  window.printw("BOTTLE ALPHA v0.1.0");
  window.keypad(true);
  window.mvaddstr(1, 0, "Press enter to go to the test chat mode thing");
  window.refresh();
  loop {
    match window.getch() {
      Some(Input::KeyF1) => break,
      Some(Input::KeyEnter) | Some(Input::Character('\n')) => {
        window.mv(window.get_max_y() / 2, window.get_max_x() / 2 - 5);
        window.printw("loading...");
        window.refresh();
        (chat_window::ChatWindow::new(&window)).run();
      },
      _ => ()
    }
  }
  endwin();
}
