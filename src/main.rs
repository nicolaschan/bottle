extern crate pancurses;

use pancurses::{initscr, endwin};

fn main() {
  let window = initscr();
  window.printw("Bottle");
  window.refresh();
  window.getch();
  endwin();
}
