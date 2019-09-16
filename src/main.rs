extern crate pancurses;
extern crate websocket;

use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

use pancurses::{endwin, initscr, noecho, start_color, Input};
mod chat_window;

fn main() {
    let (tx, rx) = channel();

    let window = initscr();
    start_color();
    noecho();
    window.printw("BOTTLE ALPHA v0.1.0");
    window.keypad(true);
    window.mvaddstr(1, 0, "Press enter to go to the test chat mode thing");
    pancurses::use_default_colors();
    window.refresh();
    loop {
        match window.getch() {
            Some(Input::KeyF1) => {
                break;
            }
            Some(Input::KeyEnter) | Some(Input::Character('\n')) => {
                let mut chat = chat_window::ChatWindow::new(&window);
                //window.nodelay(true);
                window.timeout(200);
                window.mv(window.get_max_y() / 2, window.get_max_x() / 2 - 5);
                window.printw("loading...");
                window.refresh();
                chat.run(rx, tx);
                break;
            }
            _ => (),
        }
    }
    endwin();
}
